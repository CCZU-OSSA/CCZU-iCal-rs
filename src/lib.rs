use std::{
    ffi::{c_char, CStr, CString},
    panic::catch_unwind,
};
mod ical;
mod typeddata;
mod user;

use ical::{get_reminder, ICal};
use serde::Serialize;
use tokio::runtime;
use user::UserClient;

#[no_mangle]
pub extern "C" fn generate_ics(
    username: *const c_char,
    password: *const c_char,
    firstweekdate: *const c_char,
    reminder: *const c_char,
) -> *const c_char {
    CString::new(inner(
        translate(username),
        translate(password),
        translate(firstweekdate),
        translate(reminder),
    ))
    .unwrap()
    .into_raw()
}

#[no_mangle]
pub extern "C" fn generate_ics_safejson(
    username: *const c_char,
    password: *const c_char,
    firstweekdate: *const c_char,
    reminder: *const c_char,
) -> *const c_char {
    CString::new(inner_json(
        translate(username),
        translate(password),
        translate(firstweekdate),
        translate(reminder),
    ))
    .unwrap()
    .into_raw()
}

fn translate(v: *const c_char) -> &'static str {
    unsafe { CStr::from_ptr(v) }.to_str().unwrap()
}

pub fn inner(
    username: &'static str,
    password: &'static str,
    firstweekdate: &'static str,
    reminder: &'static str,
) -> String {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let client = UserClient::new(username, password);
            client.login().await.unwrap();
            let cl = client.get_classlist().await.unwrap();
            let mut ical = ICal::new(firstweekdate.to_string(), cl);
            ical.to_ical(get_reminder(reminder)).to_string()
        })
}

#[derive(Clone, Serialize)]
pub struct JsonCallback {
    pub data: String,
    pub ok: bool,
}

impl JsonCallback {
    fn new(data: String, ok: bool) -> Self {
        Self { data, ok }
    }

    fn default() -> Self {
        Self {
            data: String::new(),
            ok: false,
        }
    }

    fn ok(&mut self, data: String) -> &mut Self {
        self.data = data;
        self.ok = true;
        self
    }

    fn err(&mut self, data: String) -> &mut Self {
        self.data = data;
        self.ok = false;
        self
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub fn inner_json(
    username: &'static str,
    password: &'static str,
    firstweekdate: &'static str,
    reminder: &'static str,
) -> String {
    let result = catch_unwind(|| {
        let mut data = JsonCallback::new(String::new(), false);
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let client = UserClient::new(username, password);
                client.login().await.unwrap();
                match client.get_classlist().await {
                    Ok(cl) => {
                        let mut ical = ICal::new(firstweekdate.to_string(), cl);
                        data.ok(ical.to_ical(get_reminder(reminder)).to_string())
                            .to_json()
                    }
                    Err(e) => data.err(e.to_string()).to_json(),
                }
            })
    });

    if result.is_ok() {
        result.unwrap()
    } else {
        JsonCallback::default().to_json()
    }
}
