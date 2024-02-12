mod ical;
mod typeddata;
mod user;
use ical::ICal;
use reqwest::Result;
use std::fs::File;
use std::io::{stdin, Write};
use std::process::exit;
use user::UserClient;

use crate::typeddata::ClassInfo;

fn block_exit(message: String) {
    println!("{}", message);
    println!("❌ 程序已中止");
    stdin().read_line(&mut String::new()).unwrap();
    exit(0);
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut username = String::new();
    let mut pwd = String::new();
    println!("输入学号");
    stdin().read_line(&mut username).unwrap();
    println!("输入密码(默认身份证后六位)");
    stdin().read_line(&mut pwd).unwrap();
    let user = UserClient::new(username.trim(), pwd.trim());
    println!("尝试登录...");
    if let Err(message) = user.login().await {
        block_exit(message)
    };
    println!("登录成功");
    println!("格式化课表...");
    let mut classlist: Vec<ClassInfo> = vec![];
    let classlist_result = user.get_classlist().await;
    if let Ok(classinfo) = classlist_result {
        classlist = classinfo;
    } else if let Err(message) = classlist_result {
        block_exit(message);
    }

    println!("格式化成功");

    let mut start = String::new();
    let mut rmd = String::new();
    println!("输入此学期第一周的星期一日期(eg 20230904)");
    stdin().read_line(&mut start).unwrap();
    let mut ical = ICal::new(start.trim().to_string(), classlist);

    println!(
        "正在配置提醒功能,请以分钟为单位设定课前提醒时间(eg 15),不配置直接回车即可"
    );
    stdin().read_line(&mut rmd).unwrap();
    let cand = ical.to_ical(ical::get_reminder(rmd.trim()));
    let save_pth: &'static str;
    if cfg!(target_os = "macos") {
        save_pth = "Downloads/class.ics"
    } else {
        save_pth = "./class.ics"
    }
    let mut f = File::create(save_pth).unwrap();
    f.write_all(cand.to_string().as_bytes()).unwrap();
    println!("已保存至 {}...", save_pth);

    stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
