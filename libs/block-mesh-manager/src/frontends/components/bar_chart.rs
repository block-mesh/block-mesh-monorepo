use charming::component::Grid;
use charming::datatype::CompositeValue;
use charming::element::{AxisLabel, AxisPointer, AxisPointerType, Tooltip, Trigger};
use charming::{component::Axis, element::AxisType, series::Bar, Chart, HtmlRenderer};
use leptos::*;

use crate::frontends::context::size_context::SizeContext;
use crate::frontends::context::webapp_context::WebAppContext;

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
                    .grid(
                        Grid::new()
                            .contain_label(true)
                            .left(CompositeValue::String("3%".to_string()))
                            .right(CompositeValue::String("4%".to_string()))
                            .bottom(CompositeValue::String("3%".to_string())),
                    )
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .axis_label(AxisLabel::new().show(true))
                            .data(data.iter().map(|i| i.day.to_string()).collect()),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow))
                    .tooltip(Tooltip::new().trigger(Trigger::Axis))
                    .series(
                        Bar::new()
                            .bar_width(60)
                            .name("Points")
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
        <div class="flex justify-center items-center mt-4 m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
            <div class="m-2 grid grid-cols-1">
                <Show when=move || ready.get()>
                    <iframe
                        srcdoc=move || html_chart.get()
                        width=move || width.get()
                        height="450"
                    ></iframe>
                </Show>
            </div>
        </div>
    }
}
