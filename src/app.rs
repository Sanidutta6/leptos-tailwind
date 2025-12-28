use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

use crate::layouts::{private_layout::PrivateLayout, public_layout::PublicLayout};
use crate::pages::{dashboard::Dashboard, home::Home, login::Login, not_found::NotFound};

#[component]
pub fn app() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                // 1. Public Routes
                <ParentRoute path=path!("") view=PublicLayout>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/login") view=Login />
                </ParentRoute>

                // 2. Private Routes
                <ParentRoute path=path!("") view=PrivateLayout>
                    <Route path=path!("/dashboard") view=Dashboard />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
