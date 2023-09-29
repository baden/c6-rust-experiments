## Мої дослідження по ESP32-C6 + Rust

```sh
cargo install cargo-generate
cargo install ldproxy
cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cargo
cd c6-hello1
rustup target add riscv32imc-unknown-none-elf
cargo build
cargo install espflash
espflash flash target/riscv32imac-esp-espidf/debug/c6-hello1 -p /dev/cu.usbserial-2120 --monitor
```

Дивно, але працює.

Спробую погратись так як на відео
https://www.youtube.com/watch?v=CXm7NdBBegk

Трохи є зміни.

