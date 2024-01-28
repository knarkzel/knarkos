build:
    cd os/ && cargo build --release && cp target/x86_64-knarkos/release/knarkos ../knarkos.elf
    ls -lah knarkos.elf
