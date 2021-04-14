  
<<<<<<< HEAD
FROM ekidd/rust-musl-builder:1.49.0 AS BUILDER
=======
FROM shtripok/rust-musl-builder:arm AS BUILDER
>>>>>>> 93b45ce2c80ac408a69ffeecb33489181e26752c

ADD --chown=rust:rust . ./


RUN sudo chmod 777 ~/.cargo/config \
    && cargo build --release

#  && echo '[source.crates-io]' >> ~/.cargo/config \
#     && echo 'replace-with = \047ustc\047' >> ~/.cargo/config \
#     && echo '[source.ustc]' >> ~/.cargo/config \
#     && echo 'registry = "git://mirrors.ustc.edu.cn/crates.io-index"' >> ~/.cargo/config \


FROM alpine:3.11

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/fudu_rust \
    /usr/local/bin/

ENTRYPOINT ["fudu_rust"]
