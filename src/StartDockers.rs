use std::process::Command;




docker build -t postgres:latest - < dockers/PostgreSql;

docker run -d --rm -p 5432:5432 --name prd_service_db postgres:latest;
docker exec prd_service_db /bin/bash -c "psql --command \"CREATE USER prd_service_db WITH SUPERUSER PASSWORD 'prd_service_db';\" && createdb -O prd_service_db prd_service_db";
#
#docker run -d --rm -p 5432:5432 --name prd_service_db postgres:latest;
#docker exec prd_service_db /bin/bash -c "psql --command \"CREATE USER prd_service_db WITH SUPERUSER PASSWORD 'prd_service_db';\" && createdb -O prd_service_db prd_service_db";
#
## docker rm rustbox;docker build -t rustbox
#docker run --rm -it --name rust -p 8090:8000 -v '/home/spiderman/GitWorld/':'/home/GitWorld' --link prd_service_db rust:latest /bin/bash;
#
#docker stop prd_service_db;

#docker run --rm -it --name rust -p 8090:8000 -v '/home/spiderman/GitWorld/':'/home/GitWorld' rust:latest /bin/bash
