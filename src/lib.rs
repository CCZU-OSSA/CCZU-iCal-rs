use std::ffi::{c_char, CStr, CString};
mod ical;
mod typeddata;
mod user;
use ical::{get_reminder, ICal};
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