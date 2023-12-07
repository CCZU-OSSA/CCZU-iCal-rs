use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue};
use std::{fs::read_to_string, path::Path};
use uuid::Uuid;

pub static COMMON_HEADER: Lazy<HeaderMap> = Lazy::new(|| {
    let mut header = HeaderMap::new();
    header.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36"));
    header
});

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ScheduleElement {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Schedule {
    pub classtime: Vec<ScheduleElement>,
}

impl Schedule {
    pub fn get_schedule() -> Self {
        let name = "custom.config.json";
        let _default = include_str!("classtime.config.json");
        if Path::new(name).exists() {
            serde_json::from_str(&read_to_string(name).unwrap_or(_default.to_string())).unwrap()
        } else {
            serde_json::from_str(_default).unwrap()
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub name: String,
    pub oe: usize,
    pub day: usize,
    pub week: Vec<String>,
    pub classtime: Vec<usize>,
    pub classroom: String,
}

#[allow(dead_code)]
impl ClassInfo {
    pub fn new(
        name: String,
        oe: usize,
        day: usize,
        week: Vec<String>,
        classtime: Vec<usize>,
        classroom: String,
    ) -> Self {
        Self {
            name,
            oe,
            day,
            week,
            classtime,
            classroom,
        }
    }

    pub fn add_classtime(&mut self, value: usize) {
        self.classtime.push(value)
    }

    pub fn add_week(&mut self, value: String) {
        self.week.push(value)
    }

    pub fn merge(&mut self, rhs: &ClassInfo) -> &mut Self {
        rhs.week.iter().for_each(|v| {
            if !self.week.contains(v) {
                self.add_week(v.clone());
            }
        });
        self
    }

    pub fn identify(&self) -> String {
        uuid::Uuid::new_v3(
            &Uuid::NAMESPACE_DNS,
            format!("{}-{}-{}-{}", self.name, self.oe, self.day, self.classroom).as_bytes(),
        )
        .to_string()
    }
}
