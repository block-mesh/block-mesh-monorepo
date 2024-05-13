use crate::components::show_check_box::ShowCheckBox;
use crate::pages::client_node_settings::ClientNodeSettingsForm;
use crate::pages::proxy_endpoint_settings::ProxyEndpointSettingsForm;
use crate::pages::proxy_master_settings::ProxyMasterSettingsForm;
use crate::state::LeptosTauriAppState;
use block_mesh_common::cli::CommandsEnum;
use leptos::*;

#[component]
pub fn SettingsWrapper() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let command = move || match state.cli_args.get().command {
        Some(command) => {
            let c: CommandsEnum = command.clone().into();
            c
        }
        None => CommandsEnum::ClientNode,
    };
    view! {
        <fieldset>
            <div class="mt-4 mb-4 grid grid-cols-1 gap-y-6 sm:grid-cols-3 sm:gap-x-4">
                <ShowCheckBox title=CommandsEnum::ClientNode/>
                <ShowCheckBox title=CommandsEnum::ProxyMaster/>
                <ShowCheckBox title=CommandsEnum::ProxyEndpoint/>
            </div>
        </fieldset>
        <Show
            when=move || command() == CommandsEnum::ClientNode
            fallback=|| {
                view! {}
            }
        >

            <ClientNodeSettingsForm/>
        </Show>
        <Show
            when=move || command() == CommandsEnum::ProxyMaster
            fallback=|| {
                view! {}
            }
        >

            <ProxyMasterSettingsForm/>
        </Show>
        <Show
            when=move || command() == CommandsEnum::ProxyEndpoint
            fallback=|| {
                view! {}
            }
        >

            <ProxyEndpointSettingsForm/>
        </Show>
    }
}
