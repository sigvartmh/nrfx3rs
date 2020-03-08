#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         //extern crate panic_semihosting;

use cortex_m::{asm, peripheral};
use cortex_m_rt::entry;
use nrf52840_hal::nrf52840_pac::{CorePeripherals, Peripherals};
mod cc310_bl;
mod fw_info;
mod serial;
use cc310_bl::*;
use fw_info::*;

pub fn boot_from_address(scb: &mut peripheral::SCB, address: u32) {
    let reset_vector_address = unsafe { *((address + 4) as *const u32) };
    let reset: Option<extern "C" fn()> =
        Some(unsafe { core::mem::transmute(reset_vector_address) });
    let stack_pointer = unsafe { *(address as *const u32) };
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    unsafe {
        cortex_m::register::msp::write(stack_pointer);
        scb.vtor.write(reset_vector_address);
    };
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    (reset.unwrap())();
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut cryptocell = peripherals.CRYPTOCELL;
    serial::init(peripherals.P0, peripherals.UARTE0);
    match cc310_init(&mut cryptocell) {
        Ok(_result) => (),
        Err(e) => loop {},
    }

    let mut slot_0_address = 0;
    let mut slot_1_address = 0;

    let provision_data = ProvisionData::get();
    match provision_data {
        Some(data) => {
            slot_0_address = data.slot_0_address;
            slot_1_address = data.slot_1_address;
        }
        None => loop {},
    }
    let slot_0_info = FwInfo::find(slot_0_address);
    let slot_1_info = FwInfo::find(slot_1_address);
    let mut core_peripherals = CorePeripherals::take().unwrap();
    match slot_1_info {
        Err(_e) => {
            boot_from_address(&mut core_peripherals.SCB, slot_0_info.unwrap().boot_address);
        }
        Ok(slot_1_info) => {
            if slot_0_info.unwrap().version >= slot_1_info.version {
                boot_from_address(&mut core_peripherals.SCB, slot_0_info.unwrap().boot_address);
                boot_from_address(&mut core_peripherals.SCB, slot_1_info.boot_address);
            } else {
                boot_from_address(&mut core_peripherals.SCB, slot_1_info.boot_address);
                boot_from_address(&mut core_peripherals.SCB, slot_0_info.unwrap().boot_address);
            }
        }
    }

    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
