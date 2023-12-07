use crate::typeddata::{ClassInfo, Schedule};

pub struct ICal {
    pub firstweekdate: String,
    pub schedule: Schedule,
    pub classlist: Vec<ClassInfo>,
}
impl ICal {
    pub fn new(firstweekdate: String, classlist: Vec<ClassInfo>) -> Self {
        Self {
            firstweekdate,
            schedule: Schedule::get_schedule(),
            classlist,
        }
    }

    pub fn get_remider(remider: &str) -> String {
        let ireminder = remider.parse::<i32>().unwrap_or(15);
        "".to_string()
    }
}
