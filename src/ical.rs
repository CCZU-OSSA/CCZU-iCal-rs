use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use icalendar::{Alarm, Calendar, Component, Event, Property, Trigger};
use uuid::Uuid;

use crate::typeddata::{ClassInfo, Schedule, EVENT_PROP, ICAL_PROP};

pub struct ICal {
    pub firstweekdate: String,
    pub schedule: Schedule,
    pub classlist: Vec<ClassInfo>,
}

pub fn get_reminder(reminder: &str) -> i32 {
    reminder.parse::<i32>().unwrap_or(15)
}

impl ICal {
    pub fn new(firstweekdate: String, classlist: Vec<ClassInfo>) -> Self {
        Self {
            firstweekdate,
            schedule: Schedule::get_schedule(),
            classlist,
        }
    }

    pub fn to_ical(&mut self, reminder: i32) -> Calendar {
        let mut cal = Calendar::new();
        ICAL_PROP.iter().for_each(|(k, v)| {
            cal.append_property(Property::new(k, v));
        });
        self.classlist.iter_mut().for_each(|e| {
            e.with_startdate(&self.firstweekdate);
        });

        for info in self.classlist.iter() {
            let start_time = self.schedule.classtime[info.classtime.first().unwrap() - 1]
                .clone()
                .start_time;
            let end_time = self.schedule.classtime[info.classtime.last().unwrap() - 1]
                .clone()
                .end_time;
            let create_time = Local::now().timestamp().to_string();
            let summary = format!("{} | {}", info.name, info.classroom);
            for day in info.daylist.iter() {
                let uid = format!("{}@gmail.com", Uuid::new_v4());
                let mut event_prop = EVENT_PROP.clone();
                event_prop.insert("SUMMARY", &summary);
                event_prop.insert("CREATED", &create_time);
                event_prop.insert("DTSTAMP", &create_time);
                event_prop.insert("LAST-MODIFIED", &create_time);
                event_prop.insert("UID", &uid);

                let start = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, start_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap()
                .to_string();
                let end = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, end_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap()
                .to_string();
                event_prop.insert("DTSTART", &start);
                event_prop.insert("DTEND", end.as_str());
                let mut event = Event::new();
                let alarm = Alarm::display(
                    "This is an event reminder",
                    Trigger::before_start(Duration::minutes(reminder as i64)),
                );
                event_prop.iter().for_each(|(k, v)| {
                    event.add_property(k, v);
                });
                event.append_component(alarm);
                cal.push(event);
            }
        }

        // week

        let fweek = NaiveDateTime::new(
            NaiveDate::parse_from_str(&self.firstweekdate.clone(), "%Y%m%d").unwrap(),
            NaiveTime::default(),
        );
        let create_time = Local::now().timestamp().to_string();
        for wn in 1..=19 {
            let summary = format!("学期第 {} 周", wn);
            let mut event_prop = EVENT_PROP.clone();
            let uid = format!("{}@gmail.com", Uuid::new_v4());
            event_prop.insert("CREATED", &create_time);
            event_prop.insert("DTSTAMP", &create_time);
            event_prop.insert("LAST-MODIFIED", &create_time);
            event_prop.insert("UID", &uid);
            event_prop.insert("SUMMARY", &summary);
            let start = fweek.timestamp().to_string();
            let end = (fweek + Duration::days(7)).timestamp().to_string();
            event_prop.insert("DTSTART", &start);
            event_prop.insert("DTEND", &end);
            let mut event = Event::new();
            event_prop.iter().for_each(|(k, v)| {
                event.add_property(k, v);
            });
            cal.push(event);
        }

        cal
    }
}
