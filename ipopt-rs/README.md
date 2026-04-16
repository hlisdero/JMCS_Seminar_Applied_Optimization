# ipopt-rs-test

## Dependencies

Install for your distribution MUMPS, METIS, OpenBlas.

```bash
sudo dnf install \
    coin-or-Ipopt-devel \
    lapack-devel \
    flexiblas-devel \
    flexiblas-openblas-openmp \
    gcc-c++ \
    clang-devel \
    openssl-devel \
    libgfortran \
    openblas-devel \
    MUMPS \
    MUMPS-devel \
    MUMPS-openmpi \
    metis \
    metis-devel
```

## Compile

```bash
cargo clean && cargo build
```
