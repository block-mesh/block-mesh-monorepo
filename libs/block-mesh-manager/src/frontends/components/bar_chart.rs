use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use crate::frontends::utils::sleep::sleep;
use charming::{
    component::Axis,
    element::{AxisType, BackgroundStyle},
    series::Bar,
    Chart, WasmRenderer,
};
use leptos::*;
use std::time::Duration;

#[component]
pub fn BarChart() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let ready = Signal::derive(move || async_data.get().is_some());

    let resource = create_local_resource(
        move || ready.get(),
        move |_| async move {
            sleep(Duration::from_millis(200)).await;
            let res = async_data.get();
            let r = match res {
                Some(r) => r,
                None => return,
            };

            let data = match r {
                Some(d) => d.daily_stats,
                None => return,
            };

            let chart = Chart::new()
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Category)
                        .data(data.iter().map(|i| i.day.to_string()).collect()),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(
                    Bar::new()
                        .show_background(true)
                        .background_style(BackgroundStyle::new().color("rgba(180, 180, 180, 0.2)"))
                        .data(data.iter().map(|i| i.points).collect()),
                );
            let renderer = WasmRenderer::new(600, 400);
            let _ = renderer.render("chart", &chart);
        },
    );
    view! {
        <div class="flex justify-center items-center border-off-white border m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
            <div class="m-2 grid grid-cols-1">
                <div class="text-off-white">Daily Points Earnings</div>
                <div id="chart">{resource.get()}</div>
            </div>
        </div>
    }
}
