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

# if user selects docker than EXE_CMD_TOOL should be set to docker , if user selects 2 than it should be set to $EXE_CMD_TOOL
if [ $choice -eq 1 ]
then
    EXE_CMD_TOOL="docker"
    USER_IDS="$(id -u):$(id -g)"
elif [ $choice -eq 2 ]
then
    EXE_CMD_TOOL="podman"
    USER_IDS="root"
else
    echo "Invalid choice"
    exit 1
fi

if [ ${#1} -le 2 ]; then
    BUILD="dev"
else
    BUILD=$1
fi

CONTAINER=$($EXE_CMD_TOOL ps| grep $SERVICE_NAME)
echo $CONTAINER

if [ ${#CONTAINER} -ge 5 ]; then
    echo "Continer is already running";
    echo "Entering Continer ........";
    $EXE_CMD_TOOL exec -it $SERVICE_NAME /bin/bash;
    exit 1;
else
    echo "Continer not running";
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

if [ ${#NETWORK_NAME} -ge 5 ]; then
    NETWORK=$($EXE_CMD_TOOL network ls| grep $NETWORK_NAME)
    if [ ${#NETWORK} -ge 5 ]; then
        NETWORK_NAME="--network ${NETWORK_NAME}"
        echo "Network Exists";
    else
        echo "Given Network Does not exits, creating one";
        $EXE_CMD_TOOL network create ${NETWORK_NAME};
        NETWORK_NAME="--network ${NETWORK_NAME}"
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

CMD="$EXE_CMD_TOOL run --user $USER_IDS --hostname $SERVICE_NAME $INTERACTIVE $NETWORK_NAME --name $SERVICE_NAME $PORT_ADDRESS $ADDITIONAL_VOLUMES -v ${PROJECT_PWD}/../:${PROJECT_PWD}/../ \"${SERVICE_IMAGE}:latest\" /bin/bash"
echo $CMD

echo "";
echo "********************";
echo "********************";
echo " Test Build will run ";
echo "********************";

eval $CMD

TAG_NUMBER=$($EXE_CMD_TOOL ps -a|grep $SERVICE_NAME|awk '{ print $1}');
$EXE_CMD_TOOL commit $TAG_NUMBER $SERVICE_IMAGE:latest;
$EXE_CMD_TOOL rm $TAG_NUMBER;

echo "----------------"
echo "If Quiting happened peacefully than all data is saved to image";
echo "----------------"
