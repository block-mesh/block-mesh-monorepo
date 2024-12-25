use crate::frontends::components::claims_card::ClaimsCard;
use crate::frontends::components::heading::Heading;
use crate::frontends::components::icons::xeno_icon::XenoIcon;
use crate::frontends::components::sub_heading::Subheading;
// use block_mesh_common::constants::BUTTON_CLASS;
use leptos::*;

#[component]
pub fn Claims() -> impl IntoView {
    view! {
        <div class="lg:flex items-start justify-start gap-4">
            <Heading>Claims</Heading>
            // <a rel="external" href="#" class=BUTTON_CLASS>
            //     "Claim your $XENO"
            // </a>
        </div>
        <Subheading class="mt-14">Token Claims</Subheading>
        <div class="mt-10 grid gap-8 sm:grid-cols-2 xl:grid-cols-3">
            <ClaimsCard
                title="Xenopus laevis"
                title_link="https://x.com/Xenopus_v1"
                value="$XENO"
                value_scale=""
                claim_link="https://xeno-claim.blockmesh.xyz"
                cookie_fun_link="https://www.cookie.fun/en/agent/xenopus-laevis"
                disabled=false
            >
                <XenoIcon/>
            </ClaimsCard>
            // <ClaimsCard
            //     title="Who will be next?"
            //     title_link=""
            //     value="Ticker?"
            //     value_scale=""
            //     dexscreener_link=""
            //     cookie_fun_link=""
            //     disabled=true
            // >
            //     <span class="material-symbols-outlined" style="font-size: 2.3em">
            //         "help"
            //     </span>
            // </ClaimsCard>
        </div>
    }
}
