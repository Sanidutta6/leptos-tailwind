use crate::api::auth_api::{LoginRequest, try_login};
use crate::utils::cookies::set_cookie;
use leptos::{ev::Event, ev::SubmitEvent, prelude::*, task::spawn_local, web_sys};
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct FormData {
    username: String,
    password: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();
    let (form_data, set_form_data) = signal(FormData::default());
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

        spawn_local(async move {
            let login_request = LoginRequest {
                username: current_data.username,
                password: current_data.password,
            };
            let response = try_login(login_request).await.map_err(|e| e.to_string());

            match response {
                Ok(result) => {
                    set_form_data.set(FormData::default());
                    set_cookie("auth_token", result.token);
                    navigate_clone("/dashboard", Default::default());
                }
                Err(error_msg) => {
                    set_err.set(error_msg);
                }
            }
        });
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
                class="mt-8 py-3 w-full cursor-pointer rounded-md bg-indigo-600 text-white transition hover:bg-indigo-700"
            >
                "Login"
            </button>
            <p class="text-center py-8">
                "Don't have an account? " <A href="/signup" attr:class="text-indigo-600 hover:underline">"Sign up"</A>
            </p>
        </form>
    </div>
    }
}
