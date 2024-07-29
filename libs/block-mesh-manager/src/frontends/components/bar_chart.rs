use crate::frontends::context::size_context::SizeContext;
use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use charming::{
    component::Axis,
    element::{AxisType, BackgroundStyle},
    series::Bar,
    Chart, HtmlRenderer,
};
use leptos::*;

#[component]
pub fn BarChart() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let size_context = use_context::<SizeContext>().unwrap();
    let width = Signal::derive(move || {
        let w = size_context.width.get();
        w * 0.5
    });

    let html_chart = create_rw_signal(String::new());
    let ready = Signal::derive(move || {
        async_data.get().is_some_and(|r| {
            if let Some(data) = r {
                let data = data.daily_stats;
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
                            .background_style(
                                BackgroundStyle::new().color("rgba(180, 180, 180, 0.2)"),
                            )
                            .data(data.iter().map(|i| i.points).collect()),
                    );
                let html_renderer = HtmlRenderer::new("Daily Points", width.get() as u64, 400);
                let res = html_renderer.render(&chart);
                if let Ok(ref html) = res {
                    html_chart.set(html.clone());
                }
                true
            } else {
                false
            }
        })
    });
    view! {
        <div class="flex justify-center items-center border-off-white border m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
            <div class="m-2 grid grid-cols-1">
                <div class="text-off-white">Daily Points Earnings</div>
                <Show when=move || ready.get()>
                    <iframe srcdoc=move || html_chart.get() width=move || width.get() height="450"></iframe>
                </Show>
            </div>
        </div>
    }
}
