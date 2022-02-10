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

DATABASE=$(docker ps| grep $DATABASE_NAME)

if [ ${#DATABASE} -ge 5 ]; then
    echo "Database Exists";
else
    echo "Run Database container server name '${DATABASE_NAME}' and docker Network '${NETWORK_NAME}' first";
    docker network create ${NETWORK_NAME};
    exit 1
fi

IMAGE=$(docker images| grep $SERVICE_IMAGE)

if [ ${#IMAGE} -ge 5 ]; then
    echo "Image Exists";
else
    echo "Build New Image";
    docker build --build-arg USERNAME="${USER}" --build-arg UID="${UID}" --build-arg PROJECT_PWD="${PROJECT_PWD}" -t "${SERVICE_IMAGE}:latest" .;
fi

#docker run --user "$(id -u):$(id -g)" -it --network bluebasket-net --name store_service -p 8001:8000 -v "/home/hayathms/GitWorld/":"/home/${USER}/GitWorld" storeservice:latest /bin/bash;
docker run --hostname $SERVICE_NAME --user "$(id -u):$(id -g)" -it --network bluebasket-net --name $SERVICE_NAME $PORT_ADDRESS -v ${PROJECT_PWD}:${PROJECT_PWD} "${SERVICE_IMAGE}:latest" /bin/bash;

TAG_NUMBER=$(docker ps -a|grep $SERVICE_NAME|awk '{ print $1}');
docker commit $TAG_NUMBER $SERVICE_IMAGE:latest;
docker rm $TAG_NUMBER;

echo "----------------"
echo "If Quiting happened peacefully than all data is saved to image";
echo "----------------"
