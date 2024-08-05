use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn Table(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div className="flow-root">
            <div class=tw_join!(& class, "-mx-[--gutter] overflow-x-auto whitespace-nowrap")>
                <div class=tw_join!("inline-block min-w-full align-middle", "sm:px-[--gutter]")>
                    <table class="min-w-full text-left text-sm/6 text-zinc-950 text-white">
                        {children()}
                    </table>
                </div>
            </div>
        </div>
    }
}
