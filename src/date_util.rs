use chrono::{Datelike, Weekday};

#[allow(dead_code)]
pub fn check_time() -> String {
    let dt = chrono::offset::Local::now();
    format!(
        "the date is {} the {} of {} {}",
        cure_day(dt.weekday()),
        cure_date(dt.day()),
        match cure_month(dt.month()) {
            Ok(m) => m,
            Err(e) => panic!("{}", e),
        },
        dt.year()
    )
}

pub fn cure_day(day: Weekday) -> String {
    match day {
        Weekday::Mon => String::from("Monday"),
        Weekday::Tue => String::from("Tuesday"),
        Weekday::Wed => String::from("Wednesday"),
        Weekday::Thu => String::from("Thursday"),
        Weekday::Fri => String::from("Friday"),
        Weekday::Sat => String::from("Saturday"),
        Weekday::Sun => String::from("Sunday"),
    }
}

pub fn cure_date(date: u32) -> String {
    match date {
        1 | 21 | 31 => format!("{}{}", date, "st"),
        2 | 22 => format!("{}{}", date, "nd"),
        3 | 23 => format!("{}{}", date, "rd"),
        _ => format!("{}{}", date, "th"),
    }
}

pub fn cure_month(month: u32) -> Result<String, String> {
    match month {
        1 => Ok(String::from("January")),
        2 => Ok(String::from("February")),
        3 => Ok(String::from("March")),
        4 => Ok(String::from("April")),
        5 => Ok(String::from("May")),
        6 => Ok(String::from("June")),
        7 => Ok(String::from("July")),
        8 => Ok(String::from("August")),
        9 => Ok(String::from("September")),
        10 => Ok(String::from("October")),
        11 => Ok(String::from("November")),
        12 => Ok(String::from("December")),
        _ => Err(format!("out of bounds\nexpected: 1-12\nrecieved: {month}")),
    }
}
