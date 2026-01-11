use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_router::{NavigateOptions, components::Outlet, hooks::use_navigate};
use leptos_use::use_cookie;

use crate::components::{
    base::sidebar::{SidebarInset, SidebarProvider},
    ui::{private_header::PrivateHeader, sidebar_navigation::SidebarNavigation}
};

#[component]
pub fn PrivateLayout() -> impl IntoView {
    let navigate = use_navigate();
    let (get_cookie, _) = use_cookie::<String, FromToStringCodec>("auth_token");

    Effect::new(move |_| {
        if get_cookie.get().is_none() {
            navigate(
                "/login",
                NavigateOptions {
                    replace: true,
                    ..Default::default()
                },
            );
        }
    });

    view! {
        <Show
            when=move || get_cookie.get().is_some()
            fallback=|| view! { <div>"Redirecting..."</div> }
        >
            <SidebarProvider>
                <SidebarNavigation />
                <SidebarInset>
                    <PrivateHeader />
                    <Outlet />
                </SidebarInset>
            </SidebarProvider>
        </Show>
    }
}
