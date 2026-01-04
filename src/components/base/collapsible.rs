// collapsible.rs
use crate::cn;
use leptos::{prelude::*, tachys::html::attribute::any_attribute::AnyAttribute};

// Context for collapsible state
#[derive(Copy, Clone)]
struct CollapsibleContext {
    is_open: ReadSignal<bool>,
    toggle: Callback<()>,
}

#[component]
pub fn Collapsible(
    #[prop(optional, default = false)] default_open: bool,
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(default_open);

    let toggle = Callback::new(move |_| {
        set_is_open.update(|state| *state = !*state);
    });

    provide_context(CollapsibleContext { is_open, toggle });

    view! {
        <div
            data-slot="collapsible"
            class=cn!("w-full", class)
            data-state={move || if is_open.get() { "open" } else { "closed" }}
            {..attrs}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, default = false)] as_child: bool,
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleContext>();

    view! {
        {match as_child {
            true => view! {
                <>
                    <div
                        on:click=move |_| ctx.toggle.run(())
                        aria-expanded={move || ctx.is_open.get()}
                        data-state={move || if ctx.is_open.get() { "open" } else { "closed" }}
                        class=cn!(
                            "data-[state=open]:bg-accent data-[state=open]:text-accent-foreground",
                            class.clone()
                        )
                        {..attrs.clone()}
                    >
                        {children()}
                    </div>
                </>
            }.into_view(),
            false => view! {
                <>
                    <button
                        data-slot="collapsible-trigger"
                        on:click=move |_| ctx.toggle.run(())
                        class=cn!(
                            "flex w-full items-center gap-2 rounded-md text-sm transition-colors hover:bg-accent hover:text-accent-foreground",
                            "data-[state=open]:bg-accent data-[state=open]:text-accent-foreground",
                            class
                        )
                        aria-expanded={move || ctx.is_open.get()}
                        data-state={move || if ctx.is_open.get() { "open" } else { "closed" }}
                        {..attrs}
                    >
                        {children()}
                    </button>
                </>
            }.into_view(),
        }}
    }
}

#[component]
pub fn CollapsibleContent(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleContext>();

    // We'll use a container with max-height transitions for simplicity
    // For exact height measurements like the React version, we'd need ResizeObserver
    view! {
        <div
            data-slot="collapsible-content"
            class=cn!(
                "overflow-hidden transition-all duration-200",
                class
            )
            style=move || {
                if ctx.is_open.get() {
                    "max-height: 1000px; opacity: 1;"
                } else {
                    "max-height: 0px; opacity: 0;"
                }
            }
            aria-hidden={move || !ctx.is_open.get()}
            {..attrs}
        >
            <div>
                {children()}
            </div>
        </div>
    }
}
