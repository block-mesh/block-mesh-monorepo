FROM --platform=$BUILDPLATFORM blockmesh/blockmesh-extra-rust
RUN git clone --depth 1 https://github.com/block-mesh/block-mesh-monorepo.git
RUN cd block-mesh-monorepo && cargo fetch
RUN mkdir -p /code/target && touch /code/target/hello.txt
#RUN cd block-mesh-monorepo && cargo leptos build --project leptos-empty-for-caching
#RUN cd block-mesh-monorepo && cargo leptos build --project leptos-empty-for-caching --release
#RUN cd block-mesh-monorepo && cargo leptos build --project block-mesh-manager
#RUN cd block-mesh-monorepo && cargo leptos build --project block-mesh-manager --release