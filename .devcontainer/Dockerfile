FROM ubuntu

USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends cmake pkg-config libssl-dev git clang build-essential curl ca-certificates

RUN useradd -ms /bin/bash codespace
USER codespace
WORKDIR /home/codespace
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH=$PATH:$HOME/.cargo/bin