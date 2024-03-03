FROM --platform=linux/arm/v7 navikey/raspbian-bullseye

RUN mkdir /opt/kiel

COPY Cargo.toml Cargo.lock /opt/kiel

COPY src/ /opt/kiel/src

COPY server/ /opt/kiel/server

COPY rustup-init /opt

RUN /opt/rustup-init -y

WORKDIR /opt/kiel

RUN cp -r /usr/local/cargo ~/.cargo

# RUN chmod +x /usr/local/cargo/env

RUN . ~/.cargo/env && cargo fetch

RUN . ~/.cargo/env && cargo build --release

RUN echo done

