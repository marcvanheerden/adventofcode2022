FROM rust

WORKDIR /home

RUN apt-get update && apt-get install -y vim strace bsdmainutils linux-perf software-properties-common ripgrep gdb
RUN rustup default nightly
RUN rustup override set nightly-2022-12-01
RUN cargo install flamegraph hyperfine cargo-asm cargo-tarpaulin dprint
RUN cargo install --locked bat
RUN rustup component add rustfmt clippy --toolchain nightly-2022-12-01-x86_64-unknown-linux-gnu
RUN rustup component add rustfmt clippy --toolchain nightly-x86_64-unknown-linux-gnu

