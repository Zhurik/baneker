FROM rust:1.85 AS builder
ENV NAME=baneker

# First build a dummy project with our dependencies to cache them in Docker
WORKDIR /usr/src
RUN cargo new --bin ${NAME}
WORKDIR /usr/src/${NAME}
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --locked --release && \
    rm src/*.rs

# Now copy the sources and do the real build
COPY . .
RUN cargo build --release

# Second stage putting the build result into a debian jessie-slim image
FROM scratch
ENV NAME=baneker

COPY --from=builder /usr/src/${NAME}/target/release/${NAME} /usr/local/bin/${NAME}

CMD ["${NAME}"]
