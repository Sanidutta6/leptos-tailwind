use crate::api::auth_api::{LoginRequest, LoginResponse, try_login};
use crate::components::base::button::{Button, ButtonVariant};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    // 1. Use new_local instead of new to allow non-Send futures (like JsFuture/gloo-net)
    let login_action = Action::new_local(|input: &LoginRequest| {
        let input = input.clone();
        async move { try_login(input).await.map_err(|e| e.to_string()) }
    });

    let login_result = login_action.value();

    view! {
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen p-8">
                <div class="flex flex-col items-center justify-center m-auto gap-4">

                    <h1 class="text-2xl font-bold italic mb-4">"Authentication"</h1>

                    <Button
                        on:click=move |_| {
                            login_action.dispatch(LoginRequest {
                                username: "johnd".to_string(),
                                password: "m38rmF$".to_string(),
                            });
                        }
                        variant=ButtonVariant::Outline
                        attr:disabled=move || login_action.pending().get()
                    >
                        {move || if login_action.pending().get() { "Authenticating..." } else { "Login" }}
                    </Button>

                    <div class="mt-4 p-6 bg-blue-900/50 backdrop-blur-sm rounded-xl min-w-[350px] border border-blue-400/30 shadow-2xl">
                        <h3 class="text-sm font-semibold uppercase tracking-wider text-blue-300 mb-3 border-b border-blue-400/20 pb-2">
                            "Server Response"
                        </h3>

                        {move || match login_result.get() {
                            None => view! {
                                <p class="text-blue-200/50 italic text-sm">"Waiting for login..."</p>
                            }.into_any(),

                            Some(Ok(resp)) => view! {
                                <div class="space-y-2 text-green-400">
                                    <p class="font-medium">"✓ Success"</p>
                                    <code class="text-xs break-all bg-black/20 p-2 block rounded">{resp.token}</code>
                                </div>
                            }.into_any(),

                            Some(Err(err_msg)) => view! {
                                <p class="text-red-400 text-sm italic">"Error: " {err_msg}</p>
                            }.into_any(),
                        }}
                    </div>
                </div>
            </div>
        </main>
    }
}
