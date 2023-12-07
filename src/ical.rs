use crate::typeddata::{ClassInfo, Schedule};

pub struct ICal {
    pub firstweekdate: String,
    pub schedule: Schedule,
    pub classlist: Vec<ClassInfo>,
}

#[allow(dead_code)]
impl ICal {
    pub fn new(firstweekdate: String, classlist: Vec<ClassInfo>) -> Self {
        Self {
            firstweekdate,
            schedule: Schedule::get_schedule(),
            classlist,
        }
    }

    pub fn get_reminder(reminder: &str) -> String {
        let v = reminder.parse::<i32>().unwrap_or(15);
        if v < 0 || v > 60 {
            panic!("Error Reminder (must range from 0 to 60)")
        }
        format!("-P0DT0H{}M0S", v)
    }
}

#[test]
fn test_or() {
    dbg!("".parse::<i32>().unwrap_or(15));
}
