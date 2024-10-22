FROM blockmesh/blockmesh-ubuntu-base:latest AS base
ARG DEBIAN_FRONTEND=noninteractive
ARG MODE=""
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /code
COPY . /code
RUN cargo fetch
#RUN cargo leptos build --project leptos-empty-for-caching
#RUN cargo leptos build --project leptos-empty-for-caching --release

EXPOSE 8000
#CMD ["cargo", "leptos", "watch", "--project", "block-mesh-manager", "--release"]
CMD ["/bin/bash", "-c", "cargo leptos watch --project block-mesh-manager $MODE"]