use crate::domain::perk::Perk;

pub const UPTIME_FACTOR: f64 = 100.0 / (24.0 * 60.0 * 60.0);
pub const TASKS_FACTOR: f64 = 10.0;

pub fn raw_points(uptime: f64, tasks_count: i64) -> f64 {
    uptime * UPTIME_FACTOR + tasks_count as f64 * TASKS_FACTOR
}

pub fn calc_points_daily(uptime: f64, tasks_count: i64, perks: &Vec<Perk>) -> f64 {
    let mut points = raw_points(uptime, tasks_count);
    for perk in perks {
        points = perk.multiplier * points;
    }
    points
}

pub fn calc_total_points(uptime: f64, tasks_count: i64, perks: &Vec<Perk>) -> f64 {
    let mut points = raw_points(uptime, tasks_count);
    for perk in perks {
        points = perk.multiplier * points;
    }
    for perk in perks {
        points = perk.one_time_bonus + points;
    }
    points
}
