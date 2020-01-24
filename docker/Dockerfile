FROM phusion/baseimage:0.11 as builder

ARG PROFILE=release
WORKDIR /nodle-chain

COPY . /nodle-chain

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y cmake pkg-config libssl-dev git clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH=$PATH:$HOME/.cargo/bin && \
        scripts/init.sh && \
        cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11

ARG PROFILE=release
ARG NETWORK=nergal

COPY --from=builder /nodle-chain/target/$PROFILE/nodle-chain /usr/local/bin
COPY ./docker/entrypoint.sh .
COPY ./docker/load_keys.sh .
COPY ./networks/$NETWORK/spec_raw.json .

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

ENTRYPOINT ["./entrypoint.sh"]