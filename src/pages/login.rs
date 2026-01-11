use crate::api::auth_api::{LoginRequest, try_login};
use crate::components::base::icons::*;
use leptos::{
    ev::{Event, SubmitEvent},
    prelude::*,
    server::codee::string::FromToStringCodec,
    task::spawn_local,
    web_sys,
};
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use leptos_use::use_cookie;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct FormData {
    username: String,
    password: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();
    let (_, set_cookie) = use_cookie::<String, FromToStringCodec>("auth_token");
    let (form_data, set_form_data) = signal(FormData::default());
    let (is_submitting, set_is_submitting) = signal(false);
    let (err, set_err) = signal(String::new());

    let handle_input_change = move |ev: Event| {
        let name = event_target::<web_sys::HtmlInputElement>(&ev).name();
        let value = event_target_value(&ev);

        let mut data = form_data.get();
        match name.as_str() {
            "username" => data.username = value,
            "password" => data.password = value,
            _ => {}
        }
        set_form_data.set(data);
    };

    let handle_form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let current_data = form_data.get();
        let navigate_clone = navigate.clone();
        set_is_submitting.set(true);

        spawn_local(async move {
            let login_request = LoginRequest {
                username: current_data.username,
                password: current_data.password,
            };
            let response = try_login(login_request).await.map_err(|e| e.to_string());

            match response {
                Ok(result) => {
                    set_form_data.set(FormData::default());
                    set_cookie.set(Some(result.token));
                    navigate_clone("/dashboard", Default::default());
                }
                Err(error_msg) => {
                    set_err.set(error_msg);
                }
            }
        });
        set_is_submitting.set(false);
    };

    view! {
    <div class="flex items-center justify-center w-full px-4 py-8">
        <form
            class="flex w-full flex-col max-w-1/3"
            on:submit=handle_form_submit
        >
            <h2 class="text-4xl font-medium text-gray-900">"Sign in"</h2>
            <p class="mt-4 text-base text-gray-500/90">
                "Please enter email and password to access."
            </p>
            {move || match err.get() {
                err_msg if !err_msg.is_empty() => view! {
                    <p class="text-red-400 text-sm italic">"Error: " {err_msg}</p>
                }.into_any(),
                _ => view! { <></> }.into_any(),
            }}
            <div class="mt-10">
                <label class="font-medium">"Email"</label>
                <input
                    placeholder="Please enter your username"
                    class="mt-2 rounded-md ring ring-gray-200 focus:ring-2 focus:ring-indigo-600 outline-none px-3 py-3 w-full"
                    required
                    type="text"
                    name="username"
                    id="username"
                    prop:value = move || form_data.get().username
                    on:input = handle_input_change
                />
            </div>

            <div class="mt-6">
                <label class="font-medium">"Password"</label>
                <input
                    placeholder="Please enter your password"
                    class="mt-2 rounded-md ring ring-gray-200 focus:ring-2 focus:ring-indigo-600 outline-none px-3 py-3 w-full"
                    required
                    type="password"
                    name="password"
                    id="password"
                    prop:value = move || form_data.get().password
                    on:input = handle_input_change
                    autocomplete
                />
            </div>

            <button
                type="submit"
                disabled=is_submitting
                class="mt-8 py-3 w-full cursor-pointer rounded-md bg-indigo-600 text-white transition hover:bg-indigo-700 flex items-center justify-center gap-2 disabled:bg-indigo-300 disabled:cursor-not-allowed"
            >
                {move || if is_submitting.get() {
                    view! {
                        <IconView
                            icon=Icon::LOADER_CIRCLE.clone()
                            class="animate-spin"
                        />
                    }
                } else {
                    view! {
                        <IconView>
                            <path d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z"/>
                            <circle cx="16.5" cy="7.5" r=".5" fill="currentColor"/>
                        </IconView>
                    }
                }}
                "Login"
            </button>
            <p class="text-center py-8">
                "Don't have an account? " <A href="/signup" attr:class="text-indigo-600 hover:underline">"Sign up"</A>
            </p>
        </form>
    </div>
    }
}
