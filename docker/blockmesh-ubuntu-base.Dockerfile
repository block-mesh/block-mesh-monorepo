FROM --platform=$BUILDPLATFORM ubuntu:22.04 AS build
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install curl gzip git-all -y