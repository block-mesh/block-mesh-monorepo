pub fn calc_points(uptime: f64, tasks_count: i64, perks: &Vec<f64>) -> f64 {
    let mut points = (uptime / (24 * 60 * 60) as f64) * 100.0 + (tasks_count as f64 * 10.0);
    for perk in perks {
        points = perk * points;
    }
    points
}
