FROM --platform=linux/arm/v7 rust

RUN mkdir /opt/kiel

COPY Cargo.toml Cargo.lock /opt/kiel

COPY src/ /opt/kiel/src

COPY server/ /opt/kiel/server

WORKDIR /opt/kiel

RUN cargo fetch

RUN cargo build --release

RUN echo done

