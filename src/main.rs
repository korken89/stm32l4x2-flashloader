#![no_std]
#![no_main]
// Define necessary functions for flash loader
//
// These are taken from the [ARM CMSIS-Pack documentation]
//
// [ARM CMSIS-Pack documentation]: https://arm-software.github.io/CMSIS_5/Pack/html/algorithmFunc.html

//#[cfg(debug_assertions)]
use panic_never as _;
use stm32l4xx_hal::stm32::FLASH;

// #[link_section = ".text"]
// fn transfer_byte(spi: &qspi0::RegisterBlock, data: u8) -> u8 {
//     // Wait until TX fifo is empty
//     while spi.txdata.read().full().bit_is_set() {}
//
//     unsafe {
//         spi.txdata.write(|w| w.data().bits(data));
//     }
//
//     loop {
//         let rxdata = spi.rxdata.read().bits();
//
//         // check if bit 31 is set, indicating
//         // that the FIFO was empty
//         if rxdata & (1 << 31) == 0 {
//             return (rxdata & 0xff) as u8;
//         }
//     }
// }
//
// #[link_section = ".text"]
// fn write_enable(spi: &qspi0::RegisterBlock) {
//     transfer_byte(spi, 0x06);
// }
//
// #[link_section = ".text"]
// fn read_status_register(spi: &qspi0::RegisterBlock) -> u8 {
//     spi.csmode.write(|w| w.mode().hold());
//
//     transfer_byte(spi, 0x05);
//
//     // Read back response
//     let val = transfer_byte(spi, 0);
//
//     spi.csmode.write(|w| w.mode().auto());
//
//     val
// }
//

#[link_section = "PrgCode"]
/// Wait till last flash operation is complete
fn wait() -> i32 {
    let sr = unsafe { &(*FLASH::ptr()).sr };
    while sr.read().bsy().bit_is_set() {}

    status()
}

#[link_section = "PrgCode"]
fn status() -> i32 {
    let sr = unsafe { &(*FLASH::ptr()).sr }.read();

    if sr.bsy().bit_is_set() {
        1
    } else if sr.pgaerr().bit_is_set() {
        1
    } else if sr.progerr().bit_is_set() {
        1
    } else if sr.wrperr().bit_is_set() {
        1
    } else {
        0
    }
}

/// Erase the sector at the given address in flash
///
/// Returns 0 on success, 1 on failure.
#[no_mangle]
#[inline(never)]
#[link_section = "PrgCode"]
pub extern "C" fn EraseSector(adr: u32) -> i32 {
    let cr = unsafe { &(*FLASH::ptr()).cr };

    // Address to sector
    let sector = (adr - 0x0800_0000) / 2048;

    cr.modify(|_, w| unsafe {
        w.bker()
            .clear_bit()
            .pnb()
            .bits(sector as u8)
            .per()
            .set_bit()
    });

    cr.modify(|_, w| w.start().set_bit());

    let status = wait();

    cr.modify(|_, w| w.per().clear_bit());

    status
}

/// Erase the chip.
///
/// Returns 0 on success, 1 on failure.
#[no_mangle]
#[inline(never)]
#[link_section = "PrgCode"]
pub extern "C" fn EraseChip() -> i32 {
    let cr = unsafe { &(*FLASH::ptr()).cr };

    cr.modify(|_, w| w.mer1().set_bit());
    cr.modify(|_, w| w.start().set_bit());

    let status = wait();

    cr.modify(|_, w| w.mer1().clear_bit());

    status
}

const FLASH_KEY1: u32 = 0x4567_0123;
const FLASH_KEY2: u32 = 0xCDEF_89AB;

/// Setup the device for the
#[no_mangle]
#[inline(never)]
#[link_section = "PrgCode"]
pub extern "C" fn Init(_adr: u32, _clk: u32, _fnc: u32) -> i32 {
    // let spi = unsafe { &(*QSPI0::ptr()) };

    // // disable memory-mapped flash
    // spi.fctrl.write(|w| w.enable().clear_bit());

    let keyr = unsafe { &(*FLASH::ptr()).keyr };
    let cr = unsafe { &(*FLASH::ptr()).cr };

    // Unlock flash
    unsafe {
        keyr.write(|w| w.bits(FLASH_KEY1));
        keyr.write(|w| w.bits(FLASH_KEY2));
    }

    if cr.read().lock().bit_is_clear() {
        0
    } else {
        1
    }
}

fn write_native(address: usize, data: u64) -> i32 {
    let cr = unsafe { &(*FLASH::ptr()).cr };
    let sr = unsafe { &(*FLASH::ptr()).sr };

    // NB: The check for alignment of the address, and that the flash is erased is made by the
    // flash controller. The `wait` function will return the proper error codes.
    let address = address as *mut u32;

    cr.modify(|_, w| w.pg().set_bit());

    unsafe {
        core::ptr::write_volatile(address, data as u32);
        core::ptr::write_volatile(address.add(1), (data >> 32) as u32);
    }

    if wait() == 1 {
        return 1;
    }

    if sr.read().eop().bit_is_set() {
        sr.modify(|_, w| w.eop().clear_bit());
    }

    cr.modify(|_, w| w.pg().clear_bit());

    0
}

#[no_mangle]
#[inline(never)]
#[link_section = "PrgCode"]
pub extern "C" fn ProgramPage(adr: u32, sz: u32, buf: *const u8) -> i32 {
    // Handle aligned address data
    let data = unsafe { core::slice::from_raw_parts(buf, sz as usize) };
    let mut address = adr as usize;
    let mut chunks = data.chunks_exact(8);

    while let Some(exact_chunk) = chunks.next() {
        use core::convert::TryInto;
        // Write chunks
        if write_native(address, u64::from_ne_bytes(exact_chunk.try_into().unwrap())) == 1 {
            return 1;
        }
        address += 8;
    }

    let rem = chunks.remainder();

    if rem.len() > 0 {
        let mut data = 0xffff_ffff_ffff_ffffu64;
        // Write remainder
        for b in rem.iter().rev() {
            data = (data << 8) | *b as u64;
        }

        if write_native(address, data) == 1 {
            return 1;
        }
    }

    0
}

#[no_mangle]
#[inline(never)]
#[link_section = "PrgCode"]
pub extern "C" fn UnInit(_fnc: u32) -> i32 {
    // Nothing to de-init
    let cr = unsafe { &(*FLASH::ptr()).cr };

    // Lock flash
    cr.modify(|_, w| w.lock().set_bit());

    0
}

const fn sectors() -> [FlashSector; 512] {
    let mut sectors = [FlashSector::default(); 512];

    sectors[0] = FlashSector {
        size: 2048,
        address: 0,
    };
    sectors[1] = SECTOR_END;

    sectors
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
#[link_section = "DeviceData"]
pub static FlashDevice: FlashDeviceDescription = FlashDeviceDescription {
    vers: 0x0101,
    dev_name: [b'a'; 128],
    dev_type: 1,
    dev_addr: 0x2000_0000,
    device_size: 1024 * 128,
    page_size: 1024,
    _reserved: 0,
    empty: 0xff,
    program_time_out: 400,
    erase_time_out: 400,
    flash_sectors: sectors(),
};

#[repr(C)]
pub struct FlashDeviceDescription {
    vers: u16,
    dev_name: [u8; 128],
    dev_type: u16,
    dev_addr: u32,
    device_size: u32,
    page_size: u32,
    _reserved: u32,
    empty: u8,
    program_time_out: u32,
    erase_time_out: u32,

    flash_sectors: [FlashSector; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FlashSector {
    size: u32,
    address: u32,
}

impl FlashSector {
    const fn default() -> Self {
        FlashSector {
            size: 0,
            address: 0,
        }
    }
}

const SECTOR_END: FlashSector = FlashSector {
    size: 0xffff_ffff,
    address: 0xffff_ffff,
};

#[link_section = "PrgData"]
#[used]
static DUMMY_DATA: u32 = 4;

/// Dummy reset.
#[no_mangle]
#[inline(never)]
#[link_section = "PrgCode"]
pub extern "C" fn Reset() -> ! {
    Init(0, 0, 0);
    EraseSector(0);
    EraseChip();
    ProgramPage(0, 0, 0 as *const _);
    UnInit(0);
    loop {
        continue;
    }
}
