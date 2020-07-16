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
    FLASH :       ORIGIN = 0x20000000, LENGTH = 16K
    RAM :       ORIGIN = 0x20004000, LENGTH = 16K
}

/* # Entry point = reset vector */
ENTRY(Reset);

/* # Sections */
SECTIONS
{
  PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

  /* ### .text */
  .text :
  {
    *(.text .text.* KEEP(.text.flashloader.*));
    *(.HardFaultTrampoline);
    *(.HardFault.*);
    . = ALIGN(4);
    __etext = .;
  } > FLASH

  /* ### .rodata */
  .rodata __etext : ALIGN(4)
  {
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  /* ## Sections in RAM */
  /* ### .data */
  .data : AT(__erodata) ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __edata = .;
  } > RAM

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .bss */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __ebss = .;
  } > RAM

  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  } > RAM

  /* Place the heap right after `.uninit` */
  . = ALIGN(4);
  __sheap = .;

  DeviceData (INFO) :
  {
    KEEP(*(DeviceData DeviceData.*));
  }

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* Do not exceed this mark in the error messages below                                    | */
/* # Alignment checks */
ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(cortex-m-rt): the start of the FLASH region must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(cortex-m-rt): the start of the RAM region must be 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
BUG(cortex-m-rt): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
BUG(cortex-m-rt): the LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
BUG(cortex-m-rt): .bss is not 4-byte aligned");

ASSERT(__sheap % 4 == 0, "
BUG(cortex-m-rt): start of .heap is not 4-byte aligned");

/* # Other checks */
ASSERT(SIZEOF(.got) == 0, "
ERROR(cortex-m-rt): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");
/* Do not exceed this mark in the error messages above                                    | */
