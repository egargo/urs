FROM blackdex/rust-musl:x86_64-musl AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM blackdex/rust-musl:x86_64-musl AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


FROM blackdex/rust-musl:x86_64-musl AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/recipe.json recipe.json
RUN cargo build --release


FROM alpine:latest AS minimizer
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/urs /app/urs
RUN apk add upx
RUN upx --best --lzma /app/urs


FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/urs.toml /app/urs.toml
COPY --from=minimizer /app/urs /app/urs
EXPOSE 2122
CMD [ "/app/urs" ]
