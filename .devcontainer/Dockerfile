FROM ubuntu

USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends cmake pkg-config libssl-dev git clang build-essential curl ca-certificates gnupg netcat \
    && curl -L https://deb.nodesource.com/setup_16.x | bash \
    && apt-get update -y \
    && apt-get -y install nodejs \
    && npm install -g yarn \
    && yarn global add @polkadot/api-cli

RUN useradd -ms /bin/bash codespace
USER codespace
WORKDIR /workspace
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

COPY ./.devcontainer/init.sh /init.sh
ENTRYPOINT [ "/init.sh" ]