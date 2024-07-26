use crate::frontends::utils::sleep::sleep;
use charming::{
    component::Axis,
    element::{AxisType, BackgroundStyle},
    series::Bar,
    Chart, WasmRenderer,
};
use chrono::{DateTime, Utc};
use leptos::logging::log;
use leptos::*;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct ChartData {
    pub x: DateTime<Utc>,
    pub y: f64,
}

#[component]
pub fn BarChart(_debug: Signal<bool>, data: Signal<Vec<ChartData>>) -> impl IntoView {
    let _resource = create_local_resource(
        || (),
        move |_| async move {
            sleep(Duration::from_millis(200)).await;
            let chart = Chart::new()
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Time)
                        .data(data.get().iter().map(|i| i.x.to_string()).collect()),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(
                    Bar::new()
                        .show_background(true)
                        .background_style(BackgroundStyle::new().color("rgba(180, 180, 180, 0.2)"))
                        .data(data.get().iter().map(|i| i.y).collect()),
                );
            let renderer = WasmRenderer::new(600, 400);
            let result = renderer.render("chart", &chart);
            log!("result {:?}", result.is_ok());
        },
    );
    view! {
        <div>
            <div id="chart"></div>
        </div>
    }
}
