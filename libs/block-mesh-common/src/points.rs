pub const UPTIME_FACTOR: f64 = 100.0 / (24.0 * 60.0 * 60.0);
pub const TASKS_FACTOR: f64 = 10.0;

pub fn raw_points(uptime: f64, tasks_count: i64) -> f64 {
    uptime * UPTIME_FACTOR + tasks_count as f64 * TASKS_FACTOR
}
