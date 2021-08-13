FROM espressif/idf-rust

# Some tools to make life with examples easier
RUN apt update \
    && apt install -y vim nano

# Dependency for Cargo first example
RUN cargo install cargo-pio

COPY support/idf-rust-examples/entrypoint.sh /opt/esp/entrypoint.sh
COPY support/idf-rust-examples/motd /etc/motd

RUN if [ ! -e /opt/rust-esp32-example ]; then git clone https://github.com/espressif/rust-esp32-example.git /opt/rust-esp32-example; fi \
    && git clone https://github.com/ivmarkov/rust-esp32-std-hello.git /opt/rust-esp32-std-hello

WORKDIR /opt/

