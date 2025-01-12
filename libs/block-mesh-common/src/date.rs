use chrono::{Duration, NaiveDate};
use std::cmp::{max, min};

pub fn date_range(i_since: &NaiveDate, i_until: &NaiveDate) -> Vec<(NaiveDate, NaiveDate)> {
    let mut dates = Vec::new();
    let since = min(i_since, i_until).clone();
    let until = max(i_since, i_until).clone();
    let mut current_date = since;
    while current_date <= until {
        let pair = (current_date, current_date + Duration::days(1));
        dates.push(pair);
        current_date += Duration::days(1);
    }
    dates
}
