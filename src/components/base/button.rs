use leptos::{prelude::*, tachys::html::attribute::any_attribute::AnyAttribute};

/// Variants like in your TS button
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

fn base_classes() -> &'static str {
    "inline-flex items-center justify-center gap-2 whitespace-nowrap \
     rounded-md text-sm font-medium transition-all \
     disabled:pointer-events-none disabled:opacity-50 \
     [&_svg]:pointer-events-none \
     [&_svg:not([class*='size-'])]:size-4 \
     shrink-0 [&_svg]:shrink-0 \
     outline-none \
     focus-visible:border-ring \
     focus-visible:ring-ring/50 \
     focus-visible:ring-[3px] \
     aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 \
     aria-invalid:border-destructive"
}

fn variant_classes(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => {
            "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90"
        }
        ButtonVariant::Destructive => {
            "bg-destructive text-white shadow-xs hover:bg-destructive/90 \
             focus-visible:ring-destructive/20 \
             dark:focus-visible:ring-destructive/40 \
             dark:bg-destructive/60"
        }
        ButtonVariant::Outline => {
            "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground \
             dark:bg-input/30 dark:border-input dark:hover:bg-input/50"
        }
        ButtonVariant::Secondary => {
            "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80"
        }
        ButtonVariant::Ghost => {
            "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50"
        }
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    }
}

fn size_classes(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
        ButtonSize::Sm => "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
        ButtonSize::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
        ButtonSize::Icon => "size-9",
    }
}

fn button_class(variant: ButtonVariant, size: ButtonSize, extra: &str) -> String {
    let mut s = String::new();
    s.push_str(base_classes());
    s.push(' ');
    s.push_str(variant_classes(variant));
    s.push(' ');
    s.push_str(size_classes(size));
    if !extra.is_empty() {
        s.push(' ');
        s.push_str(extra);
    }
    s
}

#[component]
pub fn Button(
    /// Extra classes like `className` in React
    #[prop(optional)]
    class: String,
    /// Variant prop
    #[prop(optional)]
    variant: ButtonVariant,
    /// Size prop
    #[prop(optional)]
    size: ButtonSize,
    /// On Click Callback
    #[prop(into, optional)]
    on_click: Option<Callback<()>>,
    /// Additional Attributes
    #[prop(attrs)]
    attrs: Vec<AnyAttribute>,
    /// Children like `<Button>Click</Button>`
    children: Children,
) -> impl IntoView {
    // Attach interceptor to the <button>, so intercepted attrs go here.
    view! {
        <button
            data-slot="button"
            class=button_class(variant, size, &class)
            on:click=move |_| {
                if let Some(callback) = &on_click {
                    callback.run(());
                }
            }
            {..attrs}
        >
            {children()}
        </button>
    }
}
