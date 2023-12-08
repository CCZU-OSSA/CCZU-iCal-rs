use std::ffi::{c_char, CStr, CString};
mod ical;
mod typeddata;
mod user;
use futures;
use ical::{get_reminder, ICal};
use user::UserClient;

#[no_mangle]
pub extern "C" fn generate_ics(
    username: *const c_char,
    password: *const c_char,
    firstweekdate: *const c_char,
    reminder: *const c_char,
) -> *const c_char {
    let data: String = futures::executor::block_on(async {
        let client = UserClient::new(translate(username), translate(password));
        client.login().await.unwrap();
        let cl = client.get_classlist().await.unwrap();
        let mut ical = ICal::new(translate(firstweekdate).to_string(), cl);
        ical.to_ical(get_reminder(translate(reminder))).to_string()
    });

    CString::new(data).unwrap().into_raw()
}

fn translate(v: *const c_char) -> &'static str {
    unsafe { CStr::from_ptr(v) }.to_str().unwrap()
}
