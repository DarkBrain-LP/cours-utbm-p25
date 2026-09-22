service docker restart

sudo docker pull sonarqube:community
sudo docker pull postgres:15-alpine
docker network create sonarnet

sudo docker network create sonarnet
sudo docker volume create sonarqube_data
sudo docker volume create sonarqube_extensions
sudo docker volume create sonarqube_logs

sudo docker run -d --name sonarqube-db --network sonarnet -e POSTGRES_USER=sonar -e POSTGRES_PASSWORD=sonar -e POSTGRES_DB=sonarqube -v sonarqube_data:/var/lib/postgresql/data postgres:15-alpine

sudo docker run -d --name sonarqube --network sonarnet -p 9000:9000 -e SONAR_JDBC_URL=jdbc:postgresql://sonarqube-db:5432/sonarqube -e SONAR_JDBC_USERNAME=sonar -e SONAR_JDBC_PASSWORD=sonar -v sonarqube_extensions:/opt/sonarqube/extensions -v sonarqube_logs:/opt/sonarqube/logs sonarqube:community