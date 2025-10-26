use std::convert::TryInto;
use std::ffi::{c_char, c_void};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use skyline::nn::{account, oe, time};

use crate::common::release::CURRENT_VERSION;
use training_mod_sync::*;

pub static EVENT_QUEUE: RwLock<Vec<Event>> = RwLock::new(vec![]);
static SESSION_ID: LazyLock<String> = LazyLock::new(|| unsafe {
    let mut device_uuid = Uuid {
        size: 16,
        string_size: 300,
        data: [0u8; 16],
    };
    GetPseudoDeviceId(&mut device_uuid as *mut Uuid);
    time::Initialize();
    let event_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let mut session_id_hash = Sha256Hash { hash: [0; 0x20] };
    let event_time_bytes: [u8; 16] = std::mem::transmute(event_time.to_be());
    let session_id_bytes: [u8; 32] = [event_time_bytes, device_uuid.data]
        .concat()
        .try_into()
        .expect("Session_id_bytes not the correct length");

    GenerateSha256Hash(
        &mut session_id_hash as *mut _ as *mut c_void,
        0x20 * 8,
        session_id_bytes.as_ptr() as *const c_void,
        32 * 8,
    );
    session_id_hash
        .hash
        .iter()
        .map(|i| format!("{i:02x}"))
        .collect::<Vec<String>>()
        .join("")
});
static DEVICE_ID: LazyLock<String> = LazyLock::new(|| unsafe {
    let mut device_uuid = Uuid {
        size: 16,
        string_size: 300,
        data: [0u8; 16],
    };
    GetPseudoDeviceId(&mut device_uuid as *mut Uuid);
    let mut device_id_hash = Sha256Hash { hash: [0; 0x20] };
    GenerateSha256Hash(
        &mut device_id_hash as *mut _ as *mut c_void,
        0x20 * 8,
        device_uuid.data.as_ptr() as *const c_void,
        64 * 2,
    );
    device_uuid
        .data
        .iter()
        .map(|i| format!("{i:02x}"))
        .collect::<Vec<String>>()
        .join("")
});
static USER_ID: LazyLock<String> = LazyLock::new(|| unsafe {
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
    user_uid
        .id
        .iter()
        .map(|i| format!("{i:02x}"))
        .collect::<Vec<String>>()
        .join("")
});
pub static SMASH_VERSION: LazyLock<String> = LazyLock::new(|| {
    let mut smash_version = oe::DisplayVersion { name: [0; 16] };
    unsafe {
        oe::GetDisplayVersion(&mut smash_version);

        std::ffi::CStr::from_ptr(smash_version.name.as_ptr() as *const c_char)
            .to_string_lossy()
            .into_owned()
    }
});

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
        unsafe {
            time::Initialize();
            let event_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            Event {
                user_id: USER_ID.clone(),
                device_id: DEVICE_ID.clone(),
                event_time,
                session_id: SESSION_ID.clone(),
                mod_version: CURRENT_VERSION.clone(),
                smash_version: SMASH_VERSION.clone(),
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

pub fn events_loop() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let mut event_queue_lock = lock_write(&EVENT_QUEUE);
        while let Some(event) = (*event_queue_lock).pop() {
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
        drop(event_queue_lock);
    }
}
