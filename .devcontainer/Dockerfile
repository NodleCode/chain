FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu

USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    curl -fsSL https://deb.nodesource.com/setup_17.x | sudo -E bash - && \
    apt-get -y install --no-install-recommends cmake pkg-config libssl-dev git clang build-essential curl ca-certificates nodejs && \
    npm i polkadot-launch -g

ADD .devcontainer/version /version
RUN chmod +x /version

RUN export POLKADOT_VERSION=`/version paritytech polkadot` && \
    curl -L https://github.com/paritytech/polkadot/releases/download/${POLKADOT_VERSION}/polkadot > /usr/local/bin/polkadot && \
    chmod +x /usr/local/bin/polkadot

COPY ./scripts/init.sh /tmp/init.sh
RUN chmod +x /tmp/init.sh

USER vscode
WORKDIR /home/vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    /tmp/init.sh && \
    export SUBWASM_VERSION=`/version chevdor subwasm` && \
    cargo install --locked --git https://github.com/chevdor/subwasm --tag ${SUBWASM_VERSION}
