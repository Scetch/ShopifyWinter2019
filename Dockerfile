FROM rust:1.28.0

WORKDIR /usr/src/shopifyw19

COPY . .

RUN cargo install

EXPOSE 8000

CMD ["shopifyw19"]
