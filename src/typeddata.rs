use chrono::{Duration, NaiveDate};
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue};
use std::{collections::HashMap, fs::read_to_string, path::Path};
use uuid::Uuid;

pub static COMMON_HEADER: Lazy<HeaderMap> = Lazy::new(|| {
    let mut header = HeaderMap::new();
    header.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36"));
    header
});

pub static ICAL_PROP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("PRODID", "-//Google Inc//Google Calendar 70.9054//EN");
    map.insert("VERSION", "2.0");
    map.insert("METHOD", "PUBLISH");
    map.insert("X-WR-CALNAME", "课程表");
    map.insert("X-WR-TIMEZONE", "Asia/Shanghai");
    map
});

pub static EVENT_PROP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("TRANSP", "OPAQUE");
    map.insert("X-APPLE-TRAVEL-ADVISORY-BEHAVIOR", "AUTOMATIC");
    map.insert("SEQUENCE", "0");
    map
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
    pub daylist: Vec<String>,
}

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
            daylist: vec![],
        }
    }

    pub fn add_classtime(&mut self, value: usize) {
        self.classtime.push(value)
    }
    #[allow(dead_code)]
    pub fn add_week(&mut self, value: String) {
        self.week.push(value)
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, rhs: &ClassInfo) -> &mut Self {
        rhs.week.iter().for_each(|v| {
            if !self.week.contains(v) {
                self.add_week(v.clone());
            }
        });
        self
    }
    #[allow(dead_code)]
    pub fn identify(&self) -> String {
        uuid::Uuid::new_v3(
            &Uuid::NAMESPACE_DNS,
            format!("{}-{}-{}-{}", self.name, self.oe, self.day, self.classroom).as_bytes(),
        )
        .to_string()
    }

    pub fn with_startdate(&mut self, start_date: &str) -> &mut Self {
        let firstdate = NaiveDate::parse_from_str(start_date, "%Y%m%d").unwrap();
        for week in self.week.iter() {
            let v: Vec<i32> = week.split("-").map(|v| v.parse::<i32>().unwrap()).collect();
            let (mut start_week, end_week) = (v[0], v[1]);

            let mut startdate =
                firstdate + Duration::days(((start_week - 1) * 7 + self.day as i32 - 1) as i64);
            let enddate =
                firstdate + Duration::days(((end_week - 1) * 7 + self.day as i32 - 1) as i64);

            loop {
                if self.oe == 3
                    || ((self.oe == 1) && (start_week % 2 == 1))
                    || (self.oe == 2) && (start_week % 2 == 0)
                {
                    self.daylist.push(startdate.format("%Y%m%d").to_string());
                }
                startdate += Duration::days(7);
                start_week += 1;
                if startdate > enddate {
                    break;
                }
            }
        }
        self
    }
}

#[test]
fn test_cif() {
    let mut info = ClassInfo::new(
        "TouHou Project通识".to_string(),
        3,
        1,
        vec!["6-11".to_string()],
        vec![1, 2],
        "幻想乡".to_string(),
    );

    info.with_startdate("20230905");
    dbg!(info);
}
