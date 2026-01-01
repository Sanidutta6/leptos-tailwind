use leptos::prelude::*;

#[component]
pub fn SideNavigation() -> impl IntoView {
    // Signal to handle mobile menu visibility
    let (is_open, set_is_open) = signal(false);

    view! {
        // Mobile Toggle Button (Visible only on small screens)
        <button
            type="button"
            class="lg:hidden fixed top-4 left-4 z-50 p-2 bg-slate-800 text-white rounded-md"
            on:click=move |_| set_is_open.update(|v| *v = !*v)
        >
            <span class="sr-only">"Open sidebar"</span>
            "☰"
        </button>

        // Sidebar Container
        <aside class=move || {
            format!(
                "w-64 h-screen transition-transform border-r border-slate-200 bg-white \
                {} lg:translate-x-0",
                if is_open.get() { "translate-x-0" } else { "-translate-x-full" }
            )
        }>
            <div class="h-full px-3 py-4 overflow-y-auto flex flex-col">
                // Logo / Brand
                <div class="flex items-center ps-2.5 mb-8">
                    <span class="self-center text-xl font-semibold whitespace-nowrap">"My App"</span>
                </div>

                // Navigation Links
                <ul class="space-y-2 font-medium flex-1">
                    <SidebarItem label="Dashboard" icon="🏠" href="#" />
                    <SidebarItem label="Analytics" icon="📈" href="#" />
                    <SidebarItem label="Settings" icon="⚙️" href="#" />
                </ul>

                // Footer / User Section
                <div class="pt-4 mt-4 border-t border-slate-200">
                    <SidebarItem label="Logout" icon="🚪" href="#" />
                </div>
            </div>
        </aside>

        // Backdrop for mobile
        {move || is_open.get().then(|| view! {
            <div
                class="fixed inset-0 z-30 bg-slate-900/50 lg:hidden"
                on:click=move |_| set_is_open.set(false)
            ></div>
        })}
    }
}

#[component]
fn SidebarItem(
    #[prop(into)] label: String,
    #[prop(into)] icon: String,
    #[prop(into)] href: String,
) -> impl IntoView {
    view! {
        <li>
            <a href=href class="flex items-center p-2 text-slate-900 rounded-lg hover:bg-slate-100 group transition-colors">
                <span class="text-xl group-hover:scale-110 transition-transform"> {icon} </span>
                <span class="ms-3">{label}</span>
            </a>
        </li>
    }
}
