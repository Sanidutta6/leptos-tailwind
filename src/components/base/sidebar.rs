use leptos::prelude::*;

use super::button::{Button, ButtonSize, ButtonVariant};
use crate::cn;

// For SidebarProvider
#[derive(Copy, Clone)]
pub struct SidebarContext {
    pub state: String,
    pub open: ReadSignal<bool>,
    pub set_open: Callback<bool>,
    pub toggle_sidebar: Callback<()>,
}

// For Sidebar
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

#[component]
pub fn SidebarProvider(
    #[prop(optional, default = true)] default_open: bool,
    #[prop(optional)] open: Option<ReadSignal<bool>>,
    #[prop(optional)] set_open: Option<WriteSignal<bool>>,
    children: Children,
) -> impl IntoView {
    // 1. Create local state signals
    let (is_open, set_is_open) = signal(default_open);

    // 2. Define the logic as closures
    let change_open = Callback::new(move |value: bool| {
        set_is_open.set(value);
        // If an external setter was provided, call it
        if let Some(external_setter) = set_open {
            external_setter.set(value);
        }
    });

    let toggle_sidebar = Callback::new(move |_| {
        change_open.run(!is_open.get());
    });

    // In case if reactive state is needed.
    // let state = move || if ctx.open.get() { "expanded" } else { "collapsed" };
    let state = if open.unwrap_or(is_open).get() == true {
        "expanded"
    } else {
        "collapsed"
    };

    // 3. Provide context using the struct
    provide_context(SidebarContext {
        // Use the passed-in signal if available, otherwise the local one
        state: state.to_string(),
        open: open.unwrap_or(is_open),
        set_open: change_open,
        toggle_sidebar,
    });

    view! {
        {children()}
    }
}

#[component]
pub fn Sidebar(
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Attributes, // Captures ...props
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();

    // 1. Map Enums to strings for data attributes
    let side_str = match side {
        SidebarSide::Left => "left",
        SidebarSide::Right => "right",
    };
    let variant_str = match variant {
        SidebarVariant::Sidebar => "sidebar",
        SidebarVariant::Floating => "floating",
        SidebarVariant::Inset => "inset",
    };
    let collapsible_str = match collapsible {
        SidebarCollapsible::Offcanvas => "offcanvas",
        SidebarCollapsible::Icon => "icon",
        SidebarCollapsible::None => "none",
    };

    view! {
        <div
            class="group peer text-sidebar-foreground hidden md:block"
            data-state=ctx.state
            data-collapsible=collapsible_str
            data-variant=variant_str
            data-side=side_str
            data-slot="sidebar"
            {..attributes}
        >
            // Sidebar Gap (Desktop)
            <div
                data-slot="sidebar-gap"
                class=cn!(
                    "relative w-(--sidebar-width) bg-transparent transition-[width] duration-200 ease-linear",
                    "group-data-[collapsible=offcanvas]:w-0",
                    "group-data-[side=right]:rotate-180",
                    if variant == SidebarVariant::Floating || variant == SidebarVariant::Inset {
                        "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]"
                    } else {
                        "group-data-[collapsible=icon]:w-(--sidebar-width-icon)"
                    }
                )
            />

            // Sidebar Container
            <div
                data-slot="sidebar-container"
                class=cn!(
                    "fixed inset-y-0 z-10 hidden h-svh w-(--sidebar-width) transition-[left,right,width] duration-200 ease-linear md:flex",
                    if side == SidebarSide::Left {
                        "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]"
                    } else {
                        "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]"
                    },
                    if variant == SidebarVariant::Floating || variant == SidebarVariant::Inset {
                        "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
                    } else {
                        "group-data-[collapsible=icon]:w-(--sidebar-width-icon) group-data-[side=left]:border-r group-data-[side=right]:border-l"
                    },
                    class.clone() // user-provided class
                )
            >
                <div
                    data-sidebar="sidebar"
                    data-slot="sidebar-inner"
                    class="bg-sidebar group-data-[variant=floating]:border-sidebar-border flex h-full w-full flex-col group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:shadow-sm"
                >
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SidebarTrigger(
    #[prop[optional]] class: String,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();

    view! {
        <Button
            variant=ButtonVariant::Ghost
            size=ButtonSize::Icon
            class=cn!("size-7", class)
            on:click=move |ev| {
                ev.prevent_default();
                // 1. Execute the sidebar toggle logic
                ctx.toggle_sidebar.run(());

                // 2. Execute the optional user-provided callback if it exists
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
        >
            <span class="sr-only">"Toggle Sidebar"</span>
        </Button>
    }
}

#[component]
pub fn SidebarHeader(#[prop[optional]] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-header"
            data-sidebar="header"
            class=cn!("flex flex-col gap-2 p-2", class)
        />
    }
}

#[component]
pub fn SidebarFooter(#[prop[optional]] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-footer"
            data-sidebar="footer"
            class=cn!("flex flex-col gap-2 p-2", class)
        />
    }
}

#[component]
pub fn SidebarContent(#[prop[optional]] class: String) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-content"
            data-sidebar="content"
            class=cn!("flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden", class)
        />
    }
}

#[component]
pub fn SidebarGroup(#[prop[optional]] class: String) -> impl IntoView {
  view! {
        <div
            data-slot="sidebar-group"
            data-sidebar="group"
            class=cn!("relative flex w-full min-w-0 flex-col p-2", class)
        />
    }
}