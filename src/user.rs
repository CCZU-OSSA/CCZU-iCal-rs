use regex::Regex;
use reqwest::{cookie::Cookie, Client, Result};
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use uuid::Uuid;

use crate::typeddata::{ClassInfo, COMMON_HEADER};

pub struct UserClient {
    pub stuid: String,
    pub pwd: String,
    client: Client,
}

impl UserClient {
    pub fn new(stuid: &str, pwd: &str) -> Self {
        UserClient {
            stuid: stuid.to_string(),
            pwd: pwd.to_string(),
            client: Client::builder().cookie_store(true).build().unwrap(),
        }
    }

    pub async fn get_classlist(&self) -> Result<Vec<ClassInfo>> {
        let text = self
            .client
            .get("http://219.230.159.132/web_jxrw/cx_kb_xsgrkb.aspx")
            .headers(COMMON_HEADER.clone())
            .send()
            .await?
            .text()
            .await?;
        if text.is_empty() {
            panic!("😭 程序出错，请重试，请确保你连接校园网且目前教务系统开放")
        }

        let doc = Html::parse_document(&text);
        let tb_up_seletor = Selector::parse(r#"table[id="GVxkall"]"#).unwrap();
        let tb_dn_seletor = Selector::parse(r#"table[id="GVxkkb"]"#).unwrap();
        let tb_up_itemseletor =
            Selector::parse(r#"tr[class="dg1-item"] > td[style="width:20%;"]"#).unwrap();
        let tb_dn_rowseletor = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
        let tb_dn_itemseletor = Selector::parse(r#"td[style="width:12%;"]"#).unwrap();
        let class_namelist: Vec<String> = doc
            .select(&tb_up_seletor)
            .next()
            .unwrap()
            .select(&tb_up_itemseletor)
            .map(|e| e.inner_html())
            .collect();

        let row_matrix: Vec<Vec<String>> = doc
            .select(&tb_dn_seletor)
            .next()
            .unwrap()
            .select(&tb_dn_rowseletor)
            .map(|e| {
                e.select(&tb_dn_itemseletor)
                    .map(|item| item.inner_html())
                    .collect()
            })
            .collect();
        let mut column_matrix: Vec<Vec<String>> = vec![];
        for i in 0..7 {
            let mut tmp: Vec<String> = vec![];
            for v in row_matrix.iter() {
                tmp.push(v[i].clone())
            }
            column_matrix.push(tmp.clone());
        }

        let mut course_info: HashMap<String, ClassInfo> = HashMap::new();

        for (day, courses) in column_matrix.iter().enumerate() {
            for (time, course_cb) in courses.iter().enumerate() {
                let mut target: Vec<String> = course_cb
                    .split("/")
                    .filter(|v| !v.is_empty())
                    .map(|v| v.to_string())
                    .collect();
                let targetlen = target.len();
                for index in 0..targetlen {
                    let course = target[index].clone();
                    if course != "&nbsp;" {
                        let id = Uuid::new_v3(
                            &Uuid::NAMESPACE_DNS,
                            format!("{}{}", course, day).as_bytes(),
                        )
                        .to_string();

                        if !course_info.contains_key(&id) || time == 0 {
                            let nl: Vec<String> = class_namelist
                                .iter()
                                .filter(|e| course.starts_with(e.as_str()))
                                .map(|e| e.clone())
                                .collect();
                            if nl.is_empty() {
                                if index < targetlen - 1 {
                                    target[index + 1] =
                                        format!("{}/{}", course.clone(), target[index + 1]);
                                    continue;
                                } else {
                                    panic!("Unable to resolve course name correctly")
                                }
                            }

                            let classname = nl[0].clone();

                            let re = Regex::new(r#"(\w+)? *([单双]?) *((\d+-\d+,?)+)"#).unwrap();
                            let pattern = course.replace(&classname, "").trim().to_string();
                            let Some(data) = re.captures(pattern.as_str()) else {
                                panic!("Course information parsing abnormal")
                            }; //'X立德楼409  7-8,'

                            let info = ClassInfo::new(
                                classname,
                                match data.get(2).map_or("", |m| m.as_str()) {
                                    "单" => 1,
                                    "双" => 2,
                                    _ => 3,
                                },
                                day + 1,
                                data.get(3)
                                    .map_or("", |m| m.as_str())
                                    .split(",")
                                    .filter(|e| !e.is_empty())
                                    .map(|e| e.to_string())
                                    .collect(),
                                vec![time + 1],
                                data.get(1)
                                    .map_or(String::new(), |m| m.as_str().to_string()),
                            );
                            course_info.insert(id, info);
                        } else {
                            course_info.get_mut(&id).unwrap().add_classtime(time + 1);
                        }
                    }
                }
            }
        }

        Ok(course_info.values().map(|e| e.clone()).collect())
    }

    pub async fn login(&self) -> Result<()> {
        let url = "http://jwcas.cczu.edu.cn/login";
        let text = self
            .client
            .get(url)
            .headers(COMMON_HEADER.clone())
            .send()
            .await?
            .text()
            .await?;
        let seletor = Selector::parse(r#"input[type="hidden"]"#).unwrap();
        let doc = Html::parse_document(&text);
        let mut post_data: HashMap<String, String> = HashMap::new();
        doc.select(&seletor).for_each(|e| {
            post_data.insert(
                e.attr("name").unwrap().to_string(),
                e.attr("value").unwrap().to_string(),
            );
        });

        post_data.insert("username".to_string(), self.stuid.clone());
        post_data.insert("password".to_string(), self.pwd.clone());
        post_data.insert("warn".to_string(), "true".to_string());

        let resp = self
            .client
            .post(url)
            .headers(COMMON_HEADER.clone())
            .form(&post_data)
            .send()
            .await?;
        let cookies: Vec<Cookie> = resp.cookies().collect();
        if cookies.is_empty() {
            panic!("❌ 用户名/密码错误")
        }

        post_data.clear();

        let text = self
            .client
            .get("http://jwcas.cczu.edu.cn/login?service=http://219.230.159.132/login7_jwgl.aspx")
            .headers(COMMON_HEADER.clone())
            .send()
            .await?
            .text()
            .await?;
        let doc = Html::parse_document(&text);

        let selector = Selector::parse(r#"a[href]"#).unwrap();

        self.client
            .get(
                doc.select(&selector).collect::<Vec<ElementRef>>()[1]
                    .attr("href")
                    .unwrap(),
            )
            .headers(COMMON_HEADER.clone())
            .send()
            .await?;

        Ok(()) //Ok(format!("{}={}", cookie.name(), cookie.value()))
    }
}
