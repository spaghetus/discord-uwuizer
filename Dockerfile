FROM rust AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian
COPY --from=builder /build/target/release/discord-uwuizer /bin/uwuizer
CMD /bin/uwuizer
