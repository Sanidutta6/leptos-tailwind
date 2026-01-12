use leptos::{ev, logging::log, prelude::*, tachys::html::attribute::any_attribute::AnyAttribute};
use leptos_router::components::A;

use super::button::{Button, ButtonSize, ButtonVariant};
use crate::cn;

// For SidebarProvider
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SidebarState {
    Expanded,
    Collapsed,
}

#[derive(Copy, Clone)]
pub struct SidebarContext {
    pub state: Signal<SidebarState>, // Changed to Signal
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

// Components
#[component]
pub fn SidebarProvider(
    #[prop(optional, default = true)] default_open: bool,
    #[prop(optional)] open: Option<ReadSignal<bool>>,
    #[prop(optional)] set_open: Option<WriteSignal<bool>>,
    #[prop(optional, default = String::from(""))] class: String,
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

    // Use the passed-in signal if available, otherwise the local one
    let open_signal = open.unwrap_or(is_open);

    // Create a reactive state signal that derives from open_signal
    let state = Signal::derive(move || {
        if open_signal.get() {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    });

    // 3. Provide context using the struct
    provide_context(SidebarContext {
        state,
        open: open_signal,
        set_open: change_open,
        toggle_sidebar,
    });

    view! {
        <div
            data-slot="sidebar-wrapper"
            style=("--sidebar-width", "16rem")
            style=("--sidebar-width-icon", "3rem")
            class=cn!(
                "group/sidebar-wrapper has-data-[variant=inset]:bg-sidebar flex min-h-svh w-full",
                class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Sidebar(
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<AnyAttribute>, // Captures ...props
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
        SidebarCollapsible::Icon => "icon",
        SidebarCollapsible::Offcanvas => "offcanvas",
        SidebarCollapsible::None => "none",
    };

    view! {
        <div
            class="group peer text-sidebar-foreground hidden md:block"
            data-state=move || if ctx.state.get() == SidebarState::Collapsed { "collapsed" } else { "expanded" }
            data-collapsible=move || if ctx.state.get() == SidebarState::Collapsed { collapsible_str } else { "" }
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
    #[prop(optional)] class: String,
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

                // log
                log!("sidebar state: {:#?} {:#?}", ctx.state.get_untracked(), ctx.open.get_untracked());
            }
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-panel-left-icon lucide-panel-left"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/></svg>
            <span class="sr-only">"Toggle Sidebar"</span>
        </Button>
    }
}

#[component]
pub fn SidebarInset(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <main
            data-slot="sidebar-inset"
            class=cn!(
              "bg-background relative flex w-full flex-1 flex-col",
              "md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow-sm md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2",
              class
            )
        >
            {children()}
        </main>
    }
}

#[component]
pub fn SidebarHeader(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-header"
            data-sidebar="header"
            class=cn!("flex flex-col gap-2 p-2", class)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooter(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-footer"
            data-sidebar="footer"
            class=cn!("flex flex-col gap-2 p-2", class)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarContent(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-content"
            data-sidebar="content"
            class=cn!("flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden", class)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroup(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-group"
            data-sidebar="group"
            class=cn!("relative flex w-full min-w-0 flex-col p-2", class)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenu(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu"
            data-sidebar="menu"
            class=cn!("flex w-full min-w-0 flex-col gap-1", class)
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuItem(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-item"
            data-sidebar="menu-item"
            class=cn!("group/menu-item relative", class)
        >
            {children()}
        </li>
    }
}

// SidebarGroupLabel, SidebarMenuButton, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem
#[component]
pub fn SidebarGroupLabel(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="sidebar-group-label"
            data-sidebar="group-label"
            class=cn!(
                "cn-sidebar-group-label flex shrink-0 items-center outline-hidden [&>svg]:shrink-0",
                class
            )
            {..attributes}
        >
            {children()}
        </div>
    }
}

// Variants for SidebarMenuButton
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Subtle,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

// Context to pass classes from parent to child when as_child=true
#[derive(Clone, Copy)]
pub struct SidebarMenuButtonClass {
    pub get_class: fn() -> String,
}

#[component]
pub fn SidebarMenuButton(
    #[prop(optional, default = Signal::derive(|| false))] is_active: Signal<bool>,
    #[prop(optional, default = SidebarMenuButtonVariant::Default)]
    variant: SidebarMenuButtonVariant,
    #[prop(optional, default = SidebarMenuButtonSize::Default)] size: SidebarMenuButtonSize,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] class: String,
    #[prop(optional)] as_child: bool,
    #[prop(attrs)] attributes: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    // Generate variant and size classes
    let variant_class = match variant {
        SidebarMenuButtonVariant::Default => {
            "bg-transparent hover:bg-accent hover:text-accent-foreground"
        }
        SidebarMenuButtonVariant::Outline => {
            "border border-input bg-transparent hover:bg-accent hover:text-accent-foreground"
        }
        SidebarMenuButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        SidebarMenuButtonVariant::Subtle => "text-muted-foreground hover:text-foreground",
    };

    let size_class = match size {
        SidebarMenuButtonSize::Default => "h-9 px-3 py-2",
        SidebarMenuButtonSize::Sm => "h-8 px-2 text-xs",
        SidebarMenuButtonSize::Lg => "h-10 px-4",
        SidebarMenuButtonSize::Icon => "h-9 w-9",
    };

    let base_classes = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 gap-2";

    if as_child {
        // For asChild pattern, provide a Signal that children can access
        let class_signal = Signal::derive(move || {
            cn!(
                base_classes,
                variant_class,
                size_class,
                if is_active.get() {
                    "bg-accent text-accent-foreground"
                } else {
                    ""
                },
                class.clone()
            )
        });

        provide_context(class_signal);

        let children_vec = children();
        view! {
            {children_vec}
        }
        .into_any()
    } else {
        view! {
            <button
                attr:data-slot="sidebar-menu-button"
                attr:data-sidebar="menu-button"
                attr:data-size={move || format!("{:?}", size).to_lowercase()}
                attr:data-active={move || if is_active.get() { "true" } else { "false" }}
                title=title
                class=move || cn!(
                    base_classes,
                    variant_class,
                    size_class,
                    if is_active.get() { "bg-accent text-accent-foreground" } else { "" },
                    class.clone()
                )
                {..attributes}
            >
                {children()}
            </button>
        }
        .into_any()
    }
}

#[component]
pub fn SidebarMenuSub(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <ul
            data-slot="sidebar-menu-sub"
            data-sidebar="menu-sub"
            class=cn!("cn-sidebar-menu-sub flex min-w-0 flex-col", class)
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuSubButton(
    #[prop(optional, default = false)] is_active: bool,
    #[prop(optional, default = "md")] size: &'static str, // "sm" or "md"
    #[prop(optional, into)] class: String,
    #[prop(optional)] as_child: bool,
    #[prop(attrs)] attributes: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let size_class = match size {
        "sm" => "text-xs",
        "md" => "text-sm",
        _ => "text-sm",
    };

    let active_class = if is_active {
        "bg-accent text-accent-foreground"
    } else {
        ""
    };

    let base_classes = "cn-sidebar-menu-sub-button flex min-w-0 -translate-x-px items-center overflow-hidden outline-hidden group-data-[collapsible=icon]:hidden disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&>span:last-child]:truncate [&>svg]:shrink-0";

    if as_child {
        // For asChild pattern
        let children_vec = children();
        view! {
            <div
                data-slot="sidebar-menu-sub-button"
                data-sidebar="menu-sub-button"
                data-size=size
                data-active={if is_active { "true" } else { "false" }}
                class=cn!(base_classes, size_class, active_class, class)
                {..attributes}
            >
                {children_vec}
            </div>
        }
        .into_any()
    } else {
        view! {
            <A
                href=""
                attr:data-slot="sidebar-menu-sub-button"
                attr:data-sidebar="menu-sub-button"
                attr:data-size=size
                attr:data-active={if is_active { "true" } else { "false" }}
                attr:class=cn!(base_classes, size_class, active_class, class)
                {..attributes}
            >
                {children()}
            </A>
        }
        .into_any()
    }
}

#[component]
pub fn SidebarMenuSubItem(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <li
            data-slot="sidebar-menu-sub-item"
            data-sidebar="menu-sub-item"
            class=cn!("group/menu-sub-item relative", class)
        >
            {children()}
        </li>
    }
}
