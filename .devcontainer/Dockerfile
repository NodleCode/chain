FROM ubuntu as builder

WORKDIR /build
COPY . /build/nodle

RUN apt-get update && \
    apt-get upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake pkg-config libssl-dev git clang build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    nodle/scripts/init.sh && \
    git clone https://github.com/paritytech/polkadot && \
    cd polkadot && \
    cargo build --release && \
    cd ../nodle && \
    cargo build --release

FROM ubuntu

USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y install --no-install-recommends cmake pkg-config libssl-dev git clang build-essential curl ca-certificates && \
    curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get update && \
    apt-get install -y nodejs && \
    npm i polkadot-launch -g

RUN useradd -ms /bin/bash codespace
USER codespace
WORKDIR /home/codespace
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin

COPY --from=builder /build/nodle/target/release/nodle-chain .
COPY --from=builder /build/polkadot/target/release/polkadot .
COPY ./.devcontainer/config.json .
COPY ./types.json .

CMD [ "polkadot-launch", "config.json" ]