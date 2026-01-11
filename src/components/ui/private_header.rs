use crate::components::base::{button::Button, sidebar::SidebarTrigger};
use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::use_cookie;

#[component]
pub fn PrivateHeader() -> impl IntoView {
    let (_, set_cookie) = use_cookie::<String, FromToStringCodec>("auth_token");

    view! {
        <header class="border-b py-2 px-3 flex items-center justify-between">
            <div class="flex items-center gap-2">
                <SidebarTrigger />
                <p class="text-2xl font-bold">"Brand"</p>
            </div>
            <Button on_click=move || {
                set_cookie.set(None)
            }>"Logout"</Button>
        </header>
    }
}
