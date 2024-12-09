use crate::domain::perk::Perk;
use block_mesh_common::points::raw_points;

pub fn calc_points_daily(uptime: f64, tasks_count: i64, perks: &Vec<Perk>) -> f64 {
    let mut points = raw_points(uptime, tasks_count);
    for perk in perks {
        points *= perk.multiplier;
    }
    points
}

pub fn calc_total_points(uptime: f64, tasks_count: i64, perks: &Vec<Perk>) -> f64 {
    let mut points = raw_points(uptime, tasks_count);
    for perk in perks {
        points *= perk.multiplier;
    }
    for perk in perks {
        points += perk.one_time_bonus;
    }
    points
}

pub fn calc_one_time_bonus_points(uptime: f64, tasks_count: i64, perks: &Vec<Perk>) -> f64 {
    let mut points = raw_points(uptime, tasks_count);
    for perk in perks {
        points += perk.one_time_bonus;
    }
    points
}
