use mem_cmp::*;

const PM_PROVISION_ADDRESS: u32 = 0x7000;
const FW_INFO_MAGIC: u32 = 0x8fcebb4c;
const PUBLIC_KEY_HASH_LEN: usize = 16;
const INVALID: u32 = 0xFFFF0000;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct FwInfo {
    /// Magic value to verify that the structure is valid LEN/WORDSIZE
    magic: [u32; 12 / 4],
    total_size: u32,
    pub size: u32,
    pub version: u32,
    pub address: u32,
    pub boot_address: u32,
    /* TODO: Populate rest of the structure for External ABI
        valid : u32,
        reserved : [u32; 4],
        numOfExtAPIs : u32,
        numOfRequestsExtAPI : u32,
        extApis : [ExtAPI]

    */
}

impl FwInfo {
    fn check(fw_info: FwInfo) -> bool {
        let magic = fw_info.magic;
        let ret = magic[1].mem_eq(&FW_INFO_MAGIC);
        return ret;
    }

    pub fn find(address: u32) -> Result<FwInfo, i32> {
        let fw_info = unsafe { *((address + 0x200) as *const FwInfo) };
        match FwInfo::check(fw_info) {
            true => Ok(fw_info.clone()),
            false => Err(-1),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyData {
    valid: u32,
    hash: [u8; PUBLIC_KEY_HASH_LEN],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProvisionData {
    pub slot_0_address: u32,
    pub slot_1_address: u32,
    number_of_public_keys: u32,
    key_data: [KeyData; 1],
}

impl ProvisionData {
    pub fn get() -> Option<ProvisionData> {
        let provision_data = unsafe { *(PM_PROVISION_ADDRESS as *const ProvisionData) };
        return Some(provision_data.clone());
    }
}
