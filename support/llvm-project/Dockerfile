FROM ubuntu:bionic
WORKDIR /opt
RUN apt update \
    && apt install -y gcc cmake ccache ninja-build git python3 curl g++ pkg-config libssl-dev

RUN git clone https://github.com/espressif/llvm-project.git
ADD cmake/caches /opt/cmake/caches
RUN mkdir -p  llvm-project/build \
    && cd llvm-project/build \
    && cmake -C /opt/cmake/caches/DistributionEspressif.cmake ../llvm -G "Ninja" -DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD="Xtensa" -DCMAKE_BUILD_TYPE=Release \
    && ninja stage2-distribution || echo "Build failed - silencing error to persist current build artifacts"
