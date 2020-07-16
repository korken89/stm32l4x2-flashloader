# Flash loader for 128 kB STM32L4x2

## Instructions to run

1. Generate binary:

```console
cargo objcopy --release --bin stm32l4x2_flashloader -- -O binary loader.bin
```

2. Generate the YAML file:

```console
target-gen elf target/thumbv7m-none-eabi/release/stm32l4x2_flashloader test.yml
```

Which gave the following YAML in my case:

```yaml
flash_algorithms:
  stm32l4x2_flashloader:
    name: stm32l4x2_flashloader
    description: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    default: true
    instructions: T/T/YUD2+nMB6hAhQvIUAMTyAgACaJpDEUQCMQFgAWhB9IAxAWBQ+AQcyQMM1VD4BBzJA0S/UPgEHF/qwTED1VD4BBzJA+7UUPgEHAJoIvACAgJgOCDA8gEACEAYvwEgcEdC8hQAxPICAAFoQfAEAQFgAWhB9IAxAWBQ+AQcyQMM1VD4BBzJA0S/UPgEHF/qwTED1VD4BBzJA+7UUPgEHAJoIvAEAgJgOCDA8gEACEAYvwEgcEdC8ggAQPIjEcTyAgDE8mdRAWBI9qsRzPbvUQFgwGjAD3BH8LUDry3pAAcB8AcIQPI4DELyEAOh6wgOwPIBDMTyAgO+8QgPL9MWRvJGZrOq8QgKBvEICTRodmhdaEXwAQVdYARgRmAcaOQDRL8caF/qxDQF1Rxo5AMC1Rxo5APy1BxoFOoMD2nRHGgIME5G5AcevxxoJPABBBxgXGi68QgPJPABBFxg0dK48QAPZdAC6w4GCkQS+AEdlkJh8P8BI9AS+AFdlkJF6gElINAS+AFNlkJE6gUoHtAS+AFNlkJE6gguHNAS+AFNlkJE6g4uGtAS+AEdlkJB6g4hF9AS+AEsQuoBIUJGEuBP8P8yD+BP8P8yKUYL4E/w/zJBRgfgT/D/MnFGA+AKRnFGAOAqRl5oRvABBl5gAWBCYBhowANEvxhoX+rAMAXVGGjAAwLVGGjAA/LUGGgQ6gwPA9ABIL3oAAfwvRhowAcevxhoIPABABhgWGgg8AEAWGAAIL3oAAfwvULyFADE8gIAAWhB8ABBAWAAIHBHgLVvRv/3Mf8AIP/30/7/9wL/ACAAIQAi//c4///35v/+59TUBAAAAA==
    pc_init: 183
    pc_uninit: 569
    pc_program_page: 217
    pc_erase_sector: 1
    pc_erase_all: 99
    data_section_offset: 624
    flash_properties:
      address_range:
        start: 536870912
        end: 537001984
      page_size: 1024
      erased_byte_value: 255
      program_page_timeout: 400
      erase_sector_timeout: 400
      sectors:
        - size: 2048
          address: 0
```

