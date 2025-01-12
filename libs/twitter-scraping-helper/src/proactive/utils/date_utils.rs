use chrono::{Duration, NaiveDate};
#[tracing::instrument(name = "date_range")]
pub fn date_range(since: NaiveDate, until: NaiveDate) -> Vec<(NaiveDate, NaiveDate)> {
    let mut dates = Vec::new();
    let mut current_date = since;

    while current_date <= until {
        let pair = (current_date, current_date + Duration::days(1));
        dates.push(pair);
        current_date += Duration::days(1);
    }

    dates
}
