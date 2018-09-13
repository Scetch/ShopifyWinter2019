FROM rust:1.28.0

WORKDIR /usr/src/shopifyw19
COPY . . 

RUN cargo install

CMD ["shopifyw19"]