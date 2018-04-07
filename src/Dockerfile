FROM rust:1.25
RUN mkdir /ipassigner
ADD Cargo.toml /ipassigner
ADD src /ipassigner/src
WORKDIR ipassigner
RUN cargo build
CMD ["cargo", "run"]
