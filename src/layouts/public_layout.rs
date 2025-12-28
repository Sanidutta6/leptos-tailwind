use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::ui::{public_footer::PublicFooter, public_navigation::PublicNavigation};

#[component]
pub fn PublicLayout() -> impl IntoView {
    view! {
        <div class="min-h-dvh w-full flex flex-col items-center justify-center">
            <PublicNavigation />
            <main class="flex-1 w-full">
                <Outlet />
            </main>
            <PublicFooter />
        </div>
    }
}
