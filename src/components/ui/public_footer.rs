use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn PublicFooter() -> impl IntoView {
    view! {
        <footer class="flex flex-col md:flex-row gap-3 items-center justify-around w-full py-4 text-sm bg-slate-800 text-white/70">
            <p>"Copyright © 2025 Leptos. All rights reserved."</p>
            <div class="flex items-center gap-4">
                <A href="/" attr:class="hover:text-white transition-all">
                    "Contact Us"
                </A>
                <div class="h-8 w-px bg-white/20"></div>
                <A href="/" attr:class="hover:text-white transition-all">
                    "Privacy Policy"
                </A>
            </div>
        </footer>
    }
}
