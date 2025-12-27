use leptos::prelude::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::pages::home::Home;

#[component]
pub fn app() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("home") view=Home/>
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}
