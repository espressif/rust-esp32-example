# Support files

## Rust

The directory `rust` contains Dockerfile to build custom version of rust compiler with enabled Xtensa support.

### Usage

Building and starting container:
```
cd support/rust
docker build -t rust-xtensa:bionic -f Dockerfile .
docker run --rm --name rust-xtensa -it rust-xtensa:bionic /bin/bash
```

Copy Rust dist file from running container:
```
docker cp rust-xtensa /opt/rust/build/dist/rust-dev-1.50.0-dev-x86_64-unknown-linux-gnu.tar.xz rust-1.50.0-dev-x86_64-unknown-linux-gnu-bionic.tar.xz
```
