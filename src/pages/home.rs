use crate::components::base::button::{Button, ButtonVariant};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    view! {
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <Button
                        on:click=move |_| set_value.update(|value| *value += 1)
                        variant=ButtonVariant::Destructive
                    >
                        "Increase"
                    </Button>
                    <div class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white".to_string()>
                        {value}
                    </div>
                    <Button
                        on:click=move |_| set_value.update(|value| *value -= 1)
                        variant=ButtonVariant::Outline
                    >
                        "Decrease"
                    </Button>
                </div>
            </div>
        </main>
    }
}
