mod ical;
mod typeddata;
mod user;
use ical::ICal;
use reqwest::Result;
use user::UserClient;

#[tokio::main]
async fn main() -> Result<()> {
    let user = UserClient::new_from_str("", "");
    println!("尝试登录");
    user.login().await?;
    println!("登录成功");
    let cl = user.get_classlist().await?;
    let _ = ICal::new("".to_string(), cl);

    //ASP.NET_SessionId=rrhngysv5ipxf3wvpjmrgbpa;
    Ok(())
}
