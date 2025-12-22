use std::cmp::{max, min};
use time::{Date, Duration};

pub fn date_range(i_since: &Date, i_until: &Date) -> Vec<(Date, Date)> {
    let mut dates = Vec::new();
    let since = *min(i_since, i_until);
    let until = *max(i_since, i_until);
    let mut current_date = since;
    while current_date <= until {
        let pair = (current_date, current_date + Duration::days(1));
        dates.push(pair);
        current_date += Duration::days(1);
    }
    dates
}
