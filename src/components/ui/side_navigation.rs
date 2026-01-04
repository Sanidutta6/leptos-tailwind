use crate::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader,
    SidebarMenu, SidebarMenuItem, SidebarSide, SidebarTrigger, SidebarVariant,
};
use leptos::prelude::*;

#[derive(Clone)]
struct NavMenuItem {
    id: &'static str,
    label: &'static str,
    icon: &'static str,
    href: &'static str,
    badge: Option<&'static str>,
    group: &'static str,
}

#[component]
pub fn SidebarNavigation() -> impl IntoView {
    let nav_menu_items = vec![
        // Main group
        NavMenuItem {
            id: "dashboard",
            label: "Dashboard",
            icon: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
            href: "/dashboard",
            badge: None,
            group: "main",
        },
        NavMenuItem {
            id: "projects",
            label: "Projects",
            icon: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
            href: "/projects",
            badge: Some("12"),
            group: "main",
        },
        NavMenuItem {
            id: "tasks",
            label: "Tasks",
            icon: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4",
            href: "/tasks",
            badge: Some("5"),
            group: "main",
        },
        NavMenuItem {
            id: "calendar",
            label: "Calendar",
            icon: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
            href: "/calendar",
            badge: None,
            group: "main",
        },
        // Settings group
        NavMenuItem {
            id: "profile",
            label: "Profile",
            icon: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
            href: "/profile",
            badge: None,
            group: "settings",
        },
        NavMenuItem {
            id: "settings",
            label: "Settings",
            icon: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z",
            href: "/settings",
            badge: None,
            group: "settings",
        },
        NavMenuItem {
            id: "help",
            label: "Help & Support",
            icon: "M18.364 5.636l-3.536 3.536m0 5.656l3.536 3.536M9.172 9.172L5.636 5.636m3.536 9.192l-3.536 3.536M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-5 0a4 4 0 11-8 0 4 4 0 018 0z",
            href: "/help",
            badge: None,
            group: "settings",
        },
    ];

    // Group items by their group
    let main_items = nav_menu_items
        .iter()
        .filter(|item| item.group == "main")
        .collect::<Vec<_>>();

    let settings_items = nav_menu_items
        .iter()
        .filter(|item| item.group == "settings")
        .collect::<Vec<_>>();

    view! {
        <Sidebar
            variant=SidebarVariant::Sidebar
            side=SidebarSide::Left
            collapsible=SidebarCollapsible::Offcanvas
            class="bg-sidebar"
        >
            <SidebarHeader class="p-4 border-b border-sidebar-border">
                <div class="flex items-center justify-between">
                    <h2 class="text-lg font-semibold text-sidebar-foreground">
                        "My Application"
                    </h2>
                    <SidebarTrigger class="md:hidden" />
                </div>
                <p class="text-sm text-muted-foreground">
                    "Welcome back, User"
                </p>
            </SidebarHeader>

            <SidebarContent>
                <SidebarGroup>
                    <h3 class="mb-2 px-2 text-xs font-semibold uppercase text-muted-foreground">
                        "Main"
                    </h3>
                    <SidebarMenu>
                        {main_items.into_iter().map(|item| {
                            view! {
                                <SidebarMenuItem class="rounded-md hover:bg-sidebar-accent hover:text-sidebar-accent-foreground">
                                    <a
                                        href={item.href}
                                        class="flex items-center gap-3 px-3 py-2 text-sm transition-colors"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="16"
                                            height="16"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide"
                                        >
                                            <path d={item.icon} />
                                        </svg>
                                        <span class="flex-1">{item.label}</span>
                                        {if let Some(badge) = item.badge {
                                            view! {
                                                <span class="ml-auto text-xs text-muted-foreground">
                                                    {badge}
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </a>
                                </SidebarMenuItem>
                            }
                        }).collect_view()}
                    </SidebarMenu>
                </SidebarGroup>

                <SidebarGroup>
                    <h3 class="mb-2 px-2 text-xs font-semibold uppercase text-muted-foreground">
                        "Settings"
                    </h3>
                    <SidebarMenu>
                        {settings_items.into_iter().map(|item| {
                            view! {
                                <SidebarMenuItem class="rounded-md hover:bg-sidebar-accent hover:text-sidebar-accent-foreground">
                                    <a
                                        href={item.href}
                                        class="flex items-center gap-3 px-3 py-2 text-sm transition-colors"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="16"
                                            height="16"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide"
                                        >
                                            <path d={item.icon} />
                                        </svg>
                                        <span class="flex-1">{item.label}</span>
                                        {if let Some(badge) = item.badge {
                                            view! {
                                                <span class="ml-auto text-xs text-muted-foreground">
                                                    {badge}
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </a>
                                </SidebarMenuItem>
                            }
                        }).collect_view()}
                    </SidebarMenu>
                </SidebarGroup>

                <SidebarGroup class="mt-auto">
                    <SidebarMenu>
                        <SidebarMenuItem class="rounded-md bg-sidebar-accent text-sidebar-accent-foreground">
                            <a
                                href="/upgrade"
                                class="flex items-center gap-3 px-3 py-2 text-sm"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-sparkles">
                                    <path d="m12 3-1.9 5.8a2 2 0 0 1-1.287 1.288L3 12l5.8 1.9a2 2 0 0 1 1.288 1.287L12 21l1.9-5.8a2 2 0 0 1 1.287-1.288L21 12l-5.8-1.9a2 2 0 0 1-1.288-1.287Z"/>
                                    <path d="M5 3v4"/>
                                    <path d="M19 17v4"/>
                                    <path d="M3 5h4"/>
                                    <path d="M17 19h4"/>
                                </svg>
                                <span class="flex-1">"Upgrade to Pro"</span>
                            </a>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroup>
            </SidebarContent>

            <SidebarFooter class="p-4 border-t border-sidebar-border">
                <div class="flex items-center gap-3">
                    <div class="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center">
                        <span class="text-sm font-medium text-primary">"U"</span>
                    </div>
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-medium text-sidebar-foreground truncate">
                            "User Name"
                        </p>
                        <p class="text-xs text-muted-foreground truncate">
                            "user@example.com"
                        </p>
                    </div>
                    <SidebarTrigger class="hidden md:block" />
                </div>
            </SidebarFooter>
        </Sidebar>
    }
}
