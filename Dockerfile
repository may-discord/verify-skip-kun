FROM rust:1.79.0-alpine3.20 as builder

RUN apk add --no-cache \
  alpine-sdk=1.0-r1 \
  build-base=0.5-r3 \
  && echo "nobody:*:65534:65534:nobody:/:/bin/false" > /passwd

WORKDIR /usr/src/verify-skip-kun

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl


FROM scratch as app

COPY --from=builder /passwd /etc/passwd
COPY --from=builder /usr/src/verify-skip-kun/target/x86_64-unknown-linux-musl/release/verify-skip-kun /bin/verify-skip-kun

USER nobody

CMD ["/bin/verify-skip-kun"]
