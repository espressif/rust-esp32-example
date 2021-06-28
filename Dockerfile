FROM espressif/idf
#RUN apt update \
#    && apt install -y build-essential curl
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/cargo
ENV PATH=/opt/cargo/bin:/opt/rust/bin:/opt/xtensa-esp32-elf-clang/bin:/opt/llvm-patch/bin:$PATH
RUN curl https://sh.rustup.rs -sSf | bash -s -- --profile minimal --default-toolchain nightly  -y
WORKDIR /opt

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/bionic/rust-1.50.0-dev-x86_64-unknown-linux-gnu-bionic.tar.xz \
    && tar xvf rust-1.50.0-dev-x86_64-unknown-linux-gnu-bionic.tar.xz \
    && cd rust-1.50.0-dev-x86_64-unknown-linux-gnu \
    && ./install.sh --destdir=/opt/xtensa --prefix="" --without=rust-docs \
    && cd /opt \
    && rm -rf rust-1.50.0-dev-x86_64-unknown-linux-gnu*

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/rust-src-1.50.0-dev.tar.xz \
    && tar xvf rust-src-1.50.0-dev.tar.xz \
    && cd rust-src-1.50.0-dev \
    && ./install.sh --destdir=/opt/xtensa --prefix="" --without=rust-docs \
    && cd /opt \
    && rm -rf rust-src-1.50.0-dev* \
    && rustup toolchain link xtensa /opt/xtensa \
    && rustup default xtensa

RUN wget -q https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-linux-amd64.tar.xz \
    && tar xf xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-linux-amd64.tar.xz \
    && rm xtensa-esp32-elf-llvm11_0_0-llvmorg-11-init-21247-g65ed48e-linux-amd64.tar.xz \
    && wget -q https://dl.espressif.com/dl/idf-rust/dist/x86_64-unknown-linux-gnu/bionic/llvm-patch-0.1.x86_64-unknown-linux-gnu-bionic.tar.gz \
    && tar xzf llvm-patch-0.1.x86_64-unknown-linux-gnu-bionic.tar.gz \
    && rm llvm-patch-0.1.x86_64-unknown-linux-gnu-bionic.tar.gz

RUN git clone https://github.com/espressif/rust-esp32-example.git

WORKDIR /opt/rust-esp32-example
