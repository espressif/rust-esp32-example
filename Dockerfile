FROM espressif/idf
#RUN apt update \
#    && apt install -y build-essential curl
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/cargo
ENV PATH=/opt/cargo/bin:/opt/rust/bin:/opt/xtensa-esp32-elf-clang/bin:$PATH
RUN curl https://sh.rustup.rs -sSf | bash -s -- --profile minimal --default-toolchain nightly  -y
WORKDIR /opt

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/bionic/rust-1.53.0-dev-x86_64-unknown-linux-gnu-bionic.tar.xz \
    && tar xvf rust-1.53.0-dev-x86_64-unknown-linux-gnu-bionic.tar.xz \
    && ./rust-1.53.0-dev-x86_64-unknown-linux-gnu/install.sh --destdir=/opt/esp --prefix="" --without=rust-docs \
    && rm -rf rust-1.53.0-dev-x86_64-unknown-linux-gnu*

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/noarch/rust-src-1.53.0-dev.tar.xz \
    && tar xvf rust-src-1.53.0-dev.tar.xz \
    && ./rust-src-1.53.0-dev/install.sh --destdir=/opt/esp --prefix="" --without=rust-docs \
    && rm -rf rust-src-1.53.0-dev* \
    && rustup toolchain link esp /opt/esp \
    && rustup default esp

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz \
    && tar xf xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz \
    && rm xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21249-g36dbc8b-linux-amd64.tar.xz

RUN git clone https://github.com/espressif/rust-esp32-example.git

WORKDIR /opt/rust-esp32-example
