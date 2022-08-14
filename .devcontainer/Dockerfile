FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu22.04

USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y install --no-install-recommends cmake pkg-config libssl-dev git clang build-essential curl ca-certificates podman uidmap slirp4netns
RUN echo 'unqualified-search-registries = ["docker.io"]' >> /etc/containers/registries.conf

ADD .devcontainer/version /version
RUN chmod +x /version

RUN export ZOMBIENET_VERSION=`/version paritytech zombienet` && \
    curl -L https://github.com/paritytech/zombienet/releases/download/${ZOMBIENET_VERSION}/zombienet-linux > /usr/local/bin/zombienet && \
    chmod +x /usr/local/bin/zombienet

COPY ./scripts/init.sh /tmp/init.sh
RUN chmod +x /tmp/init.sh

USER vscode
WORKDIR /home/vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    /tmp/init.sh && \
    export SUBWASM_VERSION=`/version chevdor subwasm` && \
    cargo install --locked --git https://github.com/chevdor/subwasm --tag ${SUBWASM_VERSION}