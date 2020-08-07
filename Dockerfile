FROM rust:latest as cargo-build

RUN apt-get update -yqq && apt-get install -yqq cmake g++

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/hlc_log_enrich
COPY . .
RUN cargo clean
RUN RUSTFLAGS="-Clinker=musl-gcc -Ctarget-cpu=native" cargo build --release --target=x86_64-unknown-linux-musl 
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/hlc_log_enrich*
COPY . .

RUN cargo install --path . 

FROM alpine:latest
COPY --from=cargo-build /usr/src/hlc_log_enrich/target/x86_64-unknown-linux-musl/release/hlc_log_enrich /usr/local/bin/hlc_log_enrich

CMD ["hlc_log_enrich"]

