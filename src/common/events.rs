use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use skyline::libc;
use skyline::libc::c_void;
use skyline::nn::{account, oe, time};

use crate::common::release::CURRENT_VERSION;

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
#[repr(C)]
pub struct Uuid {
    size: u32,
    string_size: u32,
    data: [u8; 16],
}

impl Uuid {
    pub fn to_str(&self) -> String {
        use std::fmt::Write;
        self.data.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
    }
}

struct Sha256Hash {
    hash: [u8; 0x20],
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn6crypto18GenerateSha256HashEPvmPKvm"]
    pub fn GenerateSha256Hash(arg1: *mut c_void, arg2: u64, arg3: *const c_void, arg4: u64);
}

impl Event {
    pub fn new() -> Event {
        let mut device_uuid = Uuid {
            size: 16,
            string_size: 300,
            data: [0u8; 16],
        };
        unsafe {
            GetPseudoDeviceId(&mut device_uuid as *mut Uuid);
        }

        unsafe {
            time::Initialize();
            let event_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            if SESSION_ID.get().is_none() {
                account::Initialize();
                let mut user_uid = account::Uid::new();
                account::GetLastOpenedUser(&mut user_uid);

                let mut user_id_hash = Sha256Hash { hash: [0; 0x20] };
                GenerateSha256Hash(
                    &mut user_id_hash as *mut _ as *mut c_void,
                    0x20 * 8,
                    user_uid.id.as_ptr() as *const c_void,
                    16 * 8,
                );

                USER_ID
                    .set(
                        user_uid
                            .id
                            .iter()
                            .map(|i| format!("{i:02x}"))
                            .collect::<Vec<String>>()
                            .join(""),
                    )
                    .unwrap();

                let mut device_id_hash = Sha256Hash { hash: [0; 0x20] };
                GenerateSha256Hash(
                    &mut device_id_hash as *mut _ as *mut c_void,
                    0x20 * 8,
                    device_uuid.data.as_ptr() as *const c_void,
                    64 * 2,
                );
                DEVICE_ID
                    .set(
                        device_uuid
                            .data
                            .iter()
                            .map(|i| format!("{i:02x}"))
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

                GenerateSha256Hash(
                    &mut session_id_hash as *mut _ as *mut c_void,
                    0x20 * 8,
                    session_id_bytes.as_ptr() as *const c_void,
                    32 * 8,
                );
                SESSION_ID
                    .set(
                        session_id_hash
                            .hash
                            .iter()
                            .map(|i| format!("{i:02x}"))
                            .collect::<Vec<String>>()
                            .join(""),
                    )
                    .unwrap();
            }

            Event {
                user_id: USER_ID.get().unwrap().to_string(),
                device_id: DEVICE_ID.get().unwrap().to_string(),
                event_time,
                session_id: SESSION_ID.get().unwrap().to_string(),
                mod_version: CURRENT_VERSION.to_string(),
                smash_version: smash_version(),
                ..Default::default()
            }
        }
    }

    pub fn smash_open() -> Event {
        Event {
            event_name: "SMASH_OPEN".to_string(),
            ..Event::new()
        }
    }

    pub fn menu_open(menu_settings: String) -> Event {
        Event {
            event_name: "MENU_OPEN".to_string(),
            menu_settings,
            ..Event::new()
        }
    }
}

pub fn smash_version() -> String {
    let mut smash_version = oe::DisplayVersion { name: [0; 16] };

    unsafe {
        oe::GetDisplayVersion(&mut smash_version);

        std::ffi::CStr::from_ptr(smash_version.name.as_ptr() as *const libc::c_char)
            .to_string_lossy()
            .into_owned()
    }
}

pub fn events_loop() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        unsafe {
            while let Some(event) = EVENT_QUEUE.pop() {
                let host = "https://my-project-1511972643240-default-rtdb.firebaseio.com";
                let path = format!(
                    "/event/{}/device/{}/{}.json",
                    event.event_name, event.device_id, event.event_time
                );

                let url = format!("{host}{path}");
                minreq::post(url)
                    .with_json(&event)
                    .expect("Failed to send info to firebase")
                    .send()
                    .ok();
            }
        }
    }
}
