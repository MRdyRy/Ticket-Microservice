FROM maven:3.9.9-eclipse-temurin-21-alpine as BUILD-APPS

LABEL DEVELOPER="ryanserfanru@gmail.com"
LABEL APPS-NAME="tiket-service"
WORKDIR /build-apps

COPY . /build-apps
RUN ls -ltr

RUN mvn verify -DskipTests
RUN mvn clean install -DskipTests
RUN ls -ltr

FROM openjdk:21 as RUN-APPS
USER root
LABEL DEVELOPER="ryanserfanru@gmail.com"
LABEL APPS-NAME="tiket-service"
WORKDIR /apps
RUN ls -ltr
ENTRYPOINT ["java","-jar","apps.jar"]