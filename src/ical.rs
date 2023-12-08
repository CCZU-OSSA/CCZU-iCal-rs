use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use icalendar::{Alarm, Calendar, Component, Event, EventLike, Property, Trigger};
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
            let create_time = Utc::now();
            let summary = format!("{} | {}", info.name, info.classroom);
            for day in info.daylist.iter() {
                let uid = format!("{}@gmail.com", Uuid::new_v4());
                let start = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, start_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap();
                let end = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, end_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap();

                let mut event = Event::new();

                EVENT_PROP.iter().for_each(|(k, v)| {
                    event.add_property(k, v);
                });

                event
                    .summary(&summary)
                    .timestamp(create_time)
                    .uid(&uid)
                    .starts(start)
                    .ends(end)
                    .alarm(Alarm::display(
                        "This is an event reminder",
                        Trigger::before_start(Duration::minutes(reminder as i64)),
                    ));

                cal.push(event);
            }
        }

        // week

        let mut fweek = NaiveDateTime::new(
            NaiveDate::parse_from_str(&self.firstweekdate.clone(), "%Y%m%d").unwrap(),
            NaiveTime::default(),
        );

        let create_time = Utc::now();
        for wn in 1..=19 {
            let summary = format!("学期第 {} 周", wn);
            let uid = format!("{}@gmail.com", Uuid::new_v4());
            let mut event = Event::new();
            event
                .uid(&uid)
                .summary(&summary)
                .timestamp(create_time)
                .starts(fweek)
                .ends(fweek + Duration::days(7));

            EVENT_PROP.iter().for_each(|(k, v)| {
                event.add_property(k, v);
            });

            cal.push(event.clone());
            fweek += Duration::days(7);
        }

        cal
    }
}
