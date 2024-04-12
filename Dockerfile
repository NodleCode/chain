FROM rust:1.77.2-bookworm as builder

ARG PROFILE=release
WORKDIR /nodle-chain

COPY . /nodle-chain

RUN apt-get update && apt-get install -qy   cmake pkg-config libssl-dev git clang build-essential curl protobuf-compiler
RUN rustup component add rust-src && rustup target add wasm32-unknown-unknown --toolchain stable
RUN cargo build -p nodle-parachain --$PROFILE && \
	bunzip2 node/res/paradis.json.bz2

# ===== SECOND STAGE ======

FROM rust:1.77.2-slim-bookworm as runtime

ARG PROFILE=release

RUN install -d /usr/local/share/nodle
COPY --from=builder /nodle-chain/target/$PROFILE/nodle-parachain /usr/local/bin
COPY --from=builder /nodle-chain/node/res/paradis.json /usr/local/share/nodle

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	rm -rf /usr/lib/python* && \
	useradd -m -u 1000 -U -s /bin/sh -d /nodle-chain nodle-chain && \
	mkdir -p /nodle-chain/.local/share/nodle-chain && \
	chown -R nodle-chain:nodle-chain /nodle-chain/.local && \
	ln -s /nodle-chain/.local/share/nodle-chain /data

USER nodle-chain
EXPOSE 30333 9933 9944
VOLUME ["/data"]

ENTRYPOINT ["nodle-parachain"]
