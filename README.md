# Installing toolchain

Original setup can be found on [Rust on ESP](https://esp-rs.github.io/book/installation/index.htm)

Install nightly version of the toolchain with the `rust-src` component
`rustup toolchain install nightly --component rust-src`

Add RISC-V target
`rustup target add riscv32imc-unknown-none-elf`

Install espup, the tool for maintaining ESP toolchain
`cargo install espup`

Install all necesary tools
`espup install`


