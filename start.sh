#!/bin/bash

#cp docker/DevDockerFile Dockerfile;

# Setting env sp database may pick the names
export DATABASE_NAME=$DATABASE_NAME
export NETWORK_NAME=$NETWORK_NAME

CONTAINER=$(docker ps| grep $SERVICE_NAME)
echo $CONTINER

if [ ${#CONTAINER} -ge 5 ]; then
    echo "Continer is already running";
    echo "Entering Continer ........";
    docker exec -it $SERVICE_NAME /bin/bash;
    exit 1;
else
    echo "Continer not running";
fi

if [ ${#DATABASE_NAME} -ge 5 ]; then
    DATABASE=$(docker ps| grep $DATABASE_NAME)
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
    NETWORK=$(docker network ls| grep $NETWORK_NAME)
    if [ ${#NETWORK} -ge 5 ]; then
        NETWORK_NAME="--network '${NETWORK_NAME}'"
        echo "Network Exists";
    else
        echo "Given Network Does not exits, creating one";
        docker network create ${NETWORK_NAME};
    fi
else
    echo "NETWORK_NAME vairable not provided";
fi

IMAGE=$(docker images| grep $SERVICE_IMAGE)

if [ ${#IMAGE} -ge 5 ]; then
    echo "Image Exists";
else
    echo "Build New Image";
    docker build --build-arg USERNAME="${USER}" --build-arg UID="${UID}" --build-arg PROJECT_PWD="${PROJECT_PWD}" -t "${SERVICE_IMAGE}:latest" .;
fi

#docker run --user "$(id -u):$(id -g)" -it --network bluebasket-net --name store_service -p 8001:8000 -v "/home/hayathms/GitWorld/":"/home/${USER}/GitWorld" storeservice:latest /bin/bash;
docker run --hostname $SERVICE_NAME --user "$(id -u):$(id -g)" -it $NETWORK_NAME --name $SERVICE_NAME $PORT_ADDRESS $ADDITIONAL_VOLUMES -v ${PROJECT_PWD}/../:${PROJECT_PWD}/../ "${SERVICE_IMAGE}:latest" /bin/bash;

TAG_NUMBER=$(docker ps -a|grep $SERVICE_NAME|awk '{ print $1}');
docker commit $TAG_NUMBER $SERVICE_IMAGE:latest;
docker rm $TAG_NUMBER;

echo "----------------"
echo "If Quiting happened peacefully than all data is saved to image";
echo "----------------"
