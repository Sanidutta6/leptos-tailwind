use crate::utils::cookies::get_cookie;
use leptos::prelude::*;
use leptos_router::{NavigateOptions, components::Outlet, hooks::use_navigate};

#[component]
pub fn PrivateLayout() -> impl IntoView {
    let navigate = use_navigate();

    // 1. Create a reactive signal for the token
    // Using Memo ensures the check runs reactively
    let auth_token = Memo::new(move |_| get_cookie("auth_token"));

    // 2. Handle the "Action" (Redirection) in an Effect
    Effect::new(move |_| {
        if auth_token.get().is_none() {
            navigate(
                "/login",
                NavigateOptions {
                    replace: true,
                    ..Default::default()
                },
            );
        }
    });

    // 3. Handle the "UI" (Rendering) using Show
    // This ensures types are compatible and Outlet only renders if authorized
    view! {
        <Show
            when=move || auth_token.get().is_some()
            fallback=|| view! { <div>"Redirecting..."</div> }
        >
            <Outlet />
        </Show>
    }
}
