use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn PrivateLayout() -> impl IntoView {
    view! {
        <Outlet />
    }
}