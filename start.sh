#!/bin/bash

# Setting env sp database may pick the names
export DATABASE_NAME=$DATABASE_NAME
export NETWORK_NAME=$NETWORK_NAME
export SERVICE_NAME=$SERVICE_NAME

# Take user input to ask weather they want to build using docker or $EXE_CMD_TOOL
echo "Enter\
	 1. for docker\
	 2. for podman\
	 3. to exit\
"

read -p "Enter your choice: " choice

# Validate input is a number
if ! [[ "$choice" =~ ^[0-9]+$ ]]; then
    echo "Invalid choice: Please enter a number"
    exit 1
fi

# if user selects docker than EXE_CMD_TOOL should be set to docker , if user selects 2 than it should be set to $EXE_CMD_TOOL
if [ "$choice" -eq 1 ]
then
    EXE_CMD_TOOL="docker"
    USER_IDS="host"
elif [ "$choice" -eq 2 ]
then
    EXE_CMD_TOOL="podman"
    USER_IDS="keep-id"
else
    echo "Invalid choice"
    exit 1
fi

if [ ${#1} -le 2 ]; then
    BUILD="dev"
else
    BUILD=$1
fi

# Check if container is running
CONTAINER=$($EXE_CMD_TOOL ps --format "{{.Names}}" | grep -w "$SERVICE_NAME")
if [ -n "$CONTAINER" ]; then
    echo "Container is already running";
    echo "Entering Container ........";
    $EXE_CMD_TOOL exec -it $SERVICE_NAME /bin/bash;
    exit 0;
else
    echo "Container not running";
fi

if [ ${#DATABASE_NAME} -ge 5 ]; then
    DATABASE=$($EXE_CMD_TOOL ps| grep $DATABASE_NAME)
    if [ ${#DATABASE} -ge 5 ]; then
        echo "Database Exists";
    else
        echo "First run Database container server name '${DATABASE_NAME}'";
        exit 1
    fi
else
    echo "Database vairable not provided";
fi

# Handle network creation differently for podman vs docker
if [ ${#NETWORK_NAME} -ge 5 ]; then
    NETWORK=$($EXE_CMD_TOOL network ls| grep $NETWORK_NAME)
    if [ ${#NETWORK} -ge 5 ]; then
        echo "Network Exists";
        NETWORK_OPTION="--network ${NETWORK_NAME}"
    else
        echo "Given Network Does not exits, creating one";
        if [ "$EXE_CMD_TOOL" = "podman" ]; then
            # For podman, try creating network with bridge driver first
            $EXE_CMD_TOOL network create ${NETWORK_NAME} 2>/dev/null
            if [ $? -ne 0 ]; then
                echo "Failed to create network with default driver, trying bridge driver..."
                $EXE_CMD_TOOL network create --driver bridge ${NETWORK_NAME}
            fi
        else
            $EXE_CMD_TOOL network create ${NETWORK_NAME}
        fi
        NETWORK_OPTION="--network ${NETWORK_NAME}"
    fi
else
    echo "NETWORK_NAME vairable not provided";
    exit 1
fi

IMAGE=$($EXE_CMD_TOOL images| grep $SERVICE_IMAGE)

if [ ${#IMAGE} -ge 5 ]; then
    echo "Image Exists";
else
    echo "Build New Image";
    $EXE_CMD_TOOL build --build-arg USERNAME="${USER}" --build-arg UID="${UID}" --build-arg PROJECT_PWD="${PROJECT_PWD}" -t "${SERVICE_IMAGE}:latest" .;
fi

EXE_COMMAND="/bin/bash"
INTERACTIVE="-it";

# Build the base command without network option first
# Mount project directory to container's home directory
CONTAINER_PROJECT_PATH="/home/$USER/GitWorld/supercontra"
BASE_CMD="$EXE_CMD_TOOL run --userns=$USER_IDS --user $USER --hostname $SERVICE_NAME $INTERACTIVE --name $SERVICE_NAME $PORT_ADDRESS $ADDITIONAL_VOLUMES -v ${PROJECT_PWD}:${CONTAINER_PROJECT_PATH}:z"

echo "";
echo "********************";
echo "********************";
echo " Test Build will run ";
echo "********************";

# Try running with different network options for podman if pasta fails
if [ "$EXE_CMD_TOOL" = "podman" ]; then
    # First try to clean up any stale pasta files (non-sudo attempt)
    echo "Cleaning up stale pasta files..."
    rm -rf /tmp/podman-run-$(id -u)/ 2>/dev/null || true
    rm -rf ~/.local/share/containers/storage/networks/ 2>/dev/null || true
    
    # Try network options in order: custom network -> slirp4netns -> host
    NETWORK_TRIED=false
    
    # Try custom network first if it exists
    if [ -n "$NETWORK_OPTION" ]; then
        echo "Trying podman with network: ${NETWORK_NAME}"
        CMD="$BASE_CMD $NETWORK_OPTION \"${SERVICE_IMAGE}:latest\" /bin/bash -c \"cd ${CONTAINER_PROJECT_PATH} && /bin/bash\""
        echo $CMD
        if eval $CMD; then
            CUSTOM_NETWORK_EXIT=0
        else
            CUSTOM_NETWORK_EXIT=$?
        fi
        NETWORK_TRIED=true
    else
        CUSTOM_NETWORK_EXIT=1
    fi
    
    # If custom network failed or doesn't exist, try slirp4netns
    if [ $CUSTOM_NETWORK_EXIT -ne 0 ] || [ "$NETWORK_TRIED" = "false" ]; then
        echo "Container failed to start with network ${NETWORK_NAME}"
        echo "Trying with slirp4netns network (bypasses pasta)..."
        $EXE_CMD_TOOL rm $SERVICE_NAME 2>/dev/null || true
        CMD="$BASE_CMD --network=slirp4netns \"${SERVICE_IMAGE}:latest\" /bin/bash -c \"cd ${CONTAINER_PROJECT_PATH} && /bin/bash\""
        echo $CMD
        if eval $CMD; then
            SLIRP_EXIT=0
        else
            SLIRP_EXIT=$?
        fi
    else
        SLIRP_EXIT=0
    fi
    
    # If slirp4netns failed, try host network
    if [ $SLIRP_EXIT -ne 0 ]; then
        echo "Container failed to start with slirp4netns"
        echo "Trying with host network..."
        $EXE_CMD_TOOL rm $SERVICE_NAME 2>/dev/null || true
        CMD="$BASE_CMD --network=host \"${SERVICE_IMAGE}:latest\" /bin/bash -c \"cd ${CONTAINER_PROJECT_PATH} && /bin/bash\""
        echo $CMD
        if eval $CMD; then
            HOST_NETWORK_EXIT=0
        else
            HOST_NETWORK_EXIT=$?
        fi
    else
        HOST_NETWORK_EXIT=0
    fi
    
    # If all failed, exit
    if [ $HOST_NETWORK_EXIT -ne 0 ]; then
        echo "All network options failed. Container cannot start."
        echo "Try running: podman system reset"
        echo "Or: rm -rf /var/lib/podman/tmpstorage/networks/rootless-netns/"
        exit 1
    fi
else
    # For docker, just run normally
    CMD="$BASE_CMD $NETWORK_OPTION \"${SERVICE_IMAGE}:latest\" /bin/bash -c \"cd ${CONTAINER_PROJECT_PATH} && /bin/bash\""
    echo $CMD
    eval $CMD
fi

TAG_NUMBER=$($EXE_CMD_TOOL ps -a|grep $SERVICE_NAME|awk '{ print $1}');
$EXE_CMD_TOOL commit $TAG_NUMBER $SERVICE_IMAGE:latest;
$EXE_CMD_TOOL rm $TAG_NUMBER;

echo "----------------"
echo "If Quiting happened peacefully than all data is saved to image";
echo "----------------"