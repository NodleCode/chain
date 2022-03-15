FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu

USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends cmake pkg-config libssl-dev git clang build-essential curl ca-certificates

COPY ./scripts/init.sh /tmp/init.sh
RUN chmod +x /tmp/init.sh

USER vscode
WORKDIR /home/vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    /tmp/init.sh
