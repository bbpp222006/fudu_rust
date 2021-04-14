  
FROM ekidd/rust-musl-builder:1.49.0 AS BUILDER

ADD --chown=rust:rust . ./


RUN sudo chmod 777 ~/.cargo/config \
    && echo '[source.crates-io]' >> ~/.cargo/config \
    && echo 'replace-with = \047ustc\047' >> ~/.cargo/config \
    && echo '[source.ustc]' >> ~/.cargo/config \
    && echo 'registry = "git://mirrors.ustc.edu.cn/crates.io-index"' >> ~/.cargo/config \
    && cargo build --release




FROM alpine:3.11

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/fudu_rust \
    /usr/local/bin/

ENTRYPOINT ["fudu_rust"]
