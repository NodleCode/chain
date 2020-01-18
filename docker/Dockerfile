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
COPY --from=builder /nodle-chain/target/$PROFILE/nodle-chain /usr/local/bin

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	rm -rf /usr/lib/python* && \
	useradd -m -u 1000 -U -s /bin/sh -d /nodle-chain nodle-chain && \
	mkdir -p /nodle-chain/.local/share/nodle-chain && \
	chown -R nodle-chain:nodle-chain /nodle-chain/.local && \
	ln -s /nodle-chain/.local/share/nodle-chain /data && \
	rm -rf /usr/bin /usr/sbin

USER nodle-chain
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/nodle-chain"]