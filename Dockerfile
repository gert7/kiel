FROM --platform=linux/arm/v7 rust

COPY . /opt/kiel

WORKDIR /opt/kiel

RUN cargo build --release

