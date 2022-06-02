FROM rust:1.61 as builder

WORKDIR /usr/src/arrow-flight-sql-client
COPY ./Cargo.toml ./Cargo.toml
## Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
#COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN rustup component add rustfmt
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update 
RUN apt-get install --no-install-recommends -y ca-certificates
#RUN apt-get install -y extra-runtime-dependencies
#RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/arrow-flight-sql-client/target/release/arrow-flight-sql-client /usr/local/bin

CMD ["arrow-flight-sql-client"]