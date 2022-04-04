FROM rust:1.57

WORKDIR /usr/sea

COPY ./ .

CMD RUST_LOG=DEBUG cargo r 
