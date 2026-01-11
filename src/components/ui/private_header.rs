use crate::components::base::sidebar::SidebarTrigger;
use leptos::prelude::*;

#[component]
pub fn PrivateHeader() -> impl IntoView {
    view! {
        <header class="border-b py-2">
            <div class="flex items-center gap-2">
                <SidebarTrigger />
                <p class="text-2xl font-bold">"Brand"</p>
            </div>
        </header>
    }
}
