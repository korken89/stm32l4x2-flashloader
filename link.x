/* # Developer notes

- Symbols that start with a double underscore (__) are considered "private"

- Symbols that start with a single underscore (_) are considered "semi-public"; they can be
  overridden in a user linker script, but should not be referred from user code (e.g. `extern "C" {
  static mut __sbss }`).

- `EXTERN` forces the linker to keep a symbol in the final binary. We use this to make sure a
  symbol if not dropped if it appears in or near the front of the linker arguments and "it's not
  needed" by any of the preceding objects (linker arguments)

- `PROVIDE` is used to provide default values that can be overridden by a user linker script

- On alignment: it's important for correctness that the VMA boundaries of both .bss and .data *and*
  the LMA of .data are all 4-byte aligned. These alignments are assumed by the RAM initialization
  routine. There's also a second benefit: 4-byte aligned boundaries means that you won't see
  "Address (..) is out of bounds" in the disassembly produced by `objdump`.
*/

/* Provides information about the memory layout of the device */
/* This will be provided by the user (see `memory.x`) or by a Board Support Crate */
MEMORY
{
    FLASH :       ORIGIN = 0, LENGTH = 16K
}

/* # Entry point = reset vector */
ENTRY(Reset);

/* # Sections */
SECTIONS
{
  PrgCode : {
      . = ALIGN(4);

      KEEP(*(PrgCode))
      KEEP(*(PrgCode.*))

      . = ALIGN(4);
  }  > FLASH

  PrgData . : {
      KEEP(*(PrgData))
      KEEP(*(PrgData.*))
  } > FLASH

  DeviceData . : {
      KEEP(*(DeviceData))
      KEEP(*(DeviceData.*))
  } > FLASH

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

