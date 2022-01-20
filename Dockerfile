FROM ubuntu as builder

ARG PROFILE=release
WORKDIR /nodle-chain

COPY . /nodle-chain

RUN apt-get update && \
	apt-get upgrade -y && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y cmake pkg-config libssl-dev git clang build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH=$PATH:$HOME/.cargo/bin && \
	scripts/init.sh && \
	cargo build -p nodle-chain --$PROFILE

# ===== SECOND STAGE ======

FROM ubuntu

ARG PROFILE=release

COPY --from=builder /nodle-chain/target/$PROFILE/nodle-chain /usr/local/bin

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y curl netcat

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

ENTRYPOINT ["nodle-chain"]
