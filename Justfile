set shell := ["bash", "-c"]

build:
    cargo build --release

flash: build
    riscv64-objcopy -O binary target/riscv32imac-unknown-none-elf/release/adc-test firmware.bin
    echo "Hold BOOT0 and press reset to prepare Longan Nano for flash. Then press enter..."
    read -n 1
    dfu-util -a 0 -s 0x08000000:leave -D firmware.bin -d 28e0:0189

list:
    dfu-util -l

