FROM rust:1.55

COPY . .

RUN cargo --version

ENTRYPOINT ["cargo", "run"]
EXPOSE 8888