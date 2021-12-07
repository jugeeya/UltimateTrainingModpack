use core::lazy::OnceCell;
use serde::{Deserialize, Serialize};
use skyline::libc::c_void;
use skyline::nn::{account, crypto, oe, time};
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

pub static mut EVENT_QUEUE: Vec<Event> = vec![];
static mut SESSION_ID: OnceCell<String> = OnceCell::new();
static mut DEVICE_ID: OnceCell<String> = OnceCell::new();
static mut USER_ID: OnceCell<String> = OnceCell::new();

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Event {
    pub event_name: String,
    pub user_id: String,
    pub device_id: String,
    pub event_time: u128,
    pub session_id: String,
    pub menu_settings: String,
    pub mod_version: String,
    pub smash_version: String,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn2oe17GetPseudoDeviceIdEPNS_4util4UuidE"]
    pub fn GetPseudoDeviceId(arg1: *mut Uuid);
}

#[derive(Debug)]
pub struct Uuid {
    Size: u32,
    StringSize: u32,
    data: [u8; 16],
}

impl Uuid {
    pub fn to_str(&self) -> String {
        self.data
            .into_iter()
            .map(|i| format!("{:02x}", i))
            .collect::<Vec<String>>()
            .join("")
    }
}

struct Sha256Hash {
    hash: [u8; 0x20],
}

impl Event {
    pub fn new() -> Event {
        let mut device_uuid = Uuid {
            Size: 16,
            StringSize: 300,
            data: [0u8; 16],
        };
        unsafe {
            GetPseudoDeviceId(&mut device_uuid as *mut Uuid);
        }

        let mut time = skyline::nn::time::PosixTime { time: 0 };
        unsafe {
            time::Initialize();
            let event_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            let mut smash_version = oe::DisplayVersion {
                name: [0 as skyline::libc::c_char; 16],
            };
            oe::GetDisplayVersion(&mut smash_version);
            if SESSION_ID.get().is_none() {
                account::Initialize();
                let mut user_uid = account::Uid::new();
                account::GetLastOpenedUser(&mut user_uid);

                let mut user_id_hash = Sha256Hash { hash: [0; 0x20] };
                crypto::GenerateSha256Hash(
                    &mut user_id_hash as *mut _ as *mut c_void,
                    0x20 * 8,
                    user_uid.id.as_ptr() as *const c_void,
                    16 * 8,
                );

                USER_ID
                    .set(
                        user_uid
                            .id
                            .into_iter()
                            .map(|i| format!("{:02x}", i))
                            .collect::<Vec<String>>()
                            .join(""),
                    )
                    .unwrap();

                let mut device_id_hash = Sha256Hash { hash: [0; 0x20] };
                crypto::GenerateSha256Hash(
                    &mut device_id_hash as *mut _ as *mut c_void,
                    0x20 * 8,
                    device_uuid.data.as_ptr() as *const c_void,
                    64 * 2,
                );
                DEVICE_ID
                    .set(
                        device_uuid
                            .data
                            .into_iter()
                            .map(|i| format!("{:02x}", i))
                            .collect::<Vec<String>>()
                            .join(""),
                    )
                    .unwrap();

                let mut session_id_hash = Sha256Hash { hash: [0; 0x20] };
                // let mut device_id_0_bytes : [u8; 8] = Default::default();
                // device_id_0_bytes.copy_from_slice(&device_uuid.data[0..8]);
                // let mut device_id_1_bytes : [u8; 8] = Default::default();
                // device_id_1_bytes.copy_from_slice(&device_uuid.data[8..16]);
                let event_time_bytes: [u8; 16] = std::mem::transmute(event_time.to_be());
                let session_id_bytes: [u8; 32] = [event_time_bytes, device_uuid.data]
                    .concat()
                    .try_into()
                    .unwrap();

                crypto::GenerateSha256Hash(
                    &mut session_id_hash as *mut _ as *mut c_void,
                    0x20 * 8,
                    session_id_bytes.as_ptr() as *const c_void,
                    32 * 8,
                );
                SESSION_ID
                    .set(
                        session_id_hash
                            .hash
                            .into_iter()
                            .map(|i| format!("{:02x}", i))
                            .collect::<Vec<String>>()
                            .join(""),
                    )
                    .unwrap();
            }

            let mut event = Event::default();
            event.user_id = USER_ID.get().unwrap().to_string();
            event.device_id = DEVICE_ID.get().unwrap().to_string();
            event.event_time = event_time;
            event.session_id = SESSION_ID.get().unwrap().to_string();
            event.mod_version = crate::common::release::CURRENT_VERSION.to_string();
            event.smash_version =
                std::ffi::CStr::from_ptr(smash_version.name.as_ptr() as *const i8)
                    .to_owned()
                    .to_str()
                    .unwrap()
                    .to_string();
            event
        }
    }

    pub fn smash_open() -> Event {
        let mut event = Event::new();
        event.event_name = "SMASH_OPEN".to_string();
        event
    }

    pub fn menu_open(menu_settings: String) -> Event {
        let mut event = Event::new();
        event.event_name = "MENU_OPEN".to_string();
        event.menu_settings = menu_settings;
        event
    }
}
