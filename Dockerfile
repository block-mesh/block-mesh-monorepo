#FROM blockmesh/blockmesh-ubuntu-base:latest AS base
FROM --platform=$BUILDPLATFORM blockmesh/blockmesh-repo

ARG DEBIAN_FRONTEND=noninteractive
ARG MODE=""
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /code
#COPY . /code
#RUN --mount=target=. \
#    --mount=type=cache,target=/root/.cache/go-build \
#    --mount=type=cache,target=/go/pkg \
#    cargo fetch
#RUN cargo fetch
#RUN #--mount=type=bind,source=.,target=. \
#    cargo fetch
#RUN cargo leptos build --project leptos-empty-for-caching
#RUN cargo leptos build --project leptos-empty-for-caching --release

EXPOSE 8000
#CMD ["cargo", "leptos", "watch", "--project", "block-mesh-manager", "--release"]
CMD ["/bin/bash", "-c", "cargo leptos watch --project block-mesh-manager $MODE"]