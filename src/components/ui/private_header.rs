use leptos::prelude::*;
use crate::components::base::sidebar::SidebarTrigger;

#[component]
pub fn PrivateHeader() -> impl IntoView {
    view! {
        <header>
            <SidebarTrigger />
        </header>
    }
}