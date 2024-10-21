FROM ubuntu:22.04
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install curl gzip git-all -y
RUN apt-get install build-essential -y
RUN apt-get install cargo -y