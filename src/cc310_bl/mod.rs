#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use cty;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use nrf52840_hal::nrf52840_pac as pac;

pub fn cc310_init(peripherals: &mut pac::CRYPTOCELL) -> Result<u32, u32> {
    static mut initialized: bool = false;

    match unsafe { initialized } {
        false => {
            enable_cc310(peripherals);
            let ret = unsafe { nrf_cc310_bl_init() };
            disable_cc310(peripherals);
            match ret {
                CRYS_OK => {
                    unsafe { initialized = true };
                    return Ok(ret);
                }
                _ => return Err(ret),
            }
        }
        true => return Ok(0),
    }
}

fn enable_cc310(p: &mut pac::CRYPTOCELL) {
    p.enable.write(|w| w.enable().enabled());
}

fn disable_cc310(p: &mut pac::CRYPTOCELL) {
    p.enable.write(|w| w.enable().disabled());
}

pub fn bl_secp256r1_validate(
    peripherals: &mut pac::CRYPTOCELL,
    hash: [u8; 64],
    hash_len: u32,
    public_key: [u8; 64],
    signature: [u8; 64],
) -> CRYSError_t {
    let mut context = nrf_cc310_bl_ecdsa_verify_context_secp256r1_t {
        init_val: 0,
        context_buffer: [0; 160usize],
    };

    enable_cc310(peripherals);
    let ret = unsafe {
        nrf_cc310_bl_ecdsa_verify_secp256r1(
            &mut context, /*(nrf_cc310_bl_ecc_public_key_secp256r1_t *)*/
            public_key.as_ptr() as *const nrf_cc310_bl_ecc_public_key_secp256r1_t, /*(nrf_cc310_bl_ecc_signature_secp256r1_t *)*/
            signature.as_ptr() as *const nrf_cc310_bl_ecc_signature_secp256r1_t,
            hash.as_ptr(),
            hash_len,
        )
    };
    disable_cc310(peripherals);
    /*
    match ret {
        CRYS_ECDSA_VERIFY_SIGNER_PUBL_KEY_VALIDATION_TAG_ERROR => return ret,
        CRYS_ECDSA_VERIFY_INVALID_SIGNATURE_IN_PTR_ERROR => return ret,
        CRYS_ECDSA_VERIFY_INVALID_MESSAGE_DATA_IN_PTR_ERROR => return ret,
        CRYS_ECDSA_VERIFY_INVALID_MESSAGE_DATA_IN_SIZE_ERROR => return ret,
        CRYS_ECDSA_VERIFY_INCONSISTENT_VERIFY_ERROR => return ret,
        _ => return ret,
    }
    */
    return ret;
}
