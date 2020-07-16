# Flash loader for 128 kB STM32L4x2

## Instructions to run

1. Generate binary:

```console
cargo objcopy --release --bin stm32l4x2_flashloader -- -O binary loader.bin
```

2. Generate the base64 encoded blob:

```console
 base64 -w 0 loader.bin
```
