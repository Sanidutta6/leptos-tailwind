use crate::components::base::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::base::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarHeader, SidebarVariant,
};
use crate::components::base::icons::*;
use leptos::prelude::*;
use leptos_router::components::A;

// Navigation item structures
#[derive(Clone)]
pub struct NavItem {
    pub title: &'static str,
    pub icon: Option<Icon>,
    pub url: Option<&'static str>,
    pub permissions: Vec<&'static str>,
    pub items: Option<Vec<NavItem>>,
}

#[derive(Clone)]
pub struct NavGroup {
    pub label: &'static str,
    pub items: Vec<NavItem>,
}

pub struct SidebarNavConfig {
    pub groups: Vec<NavGroup>,
}

impl SidebarNavConfig {
    pub fn new() -> Self {
        Self {
            groups: vec![NavGroup {
                label: "",
                items: vec![
                    NavItem {
                        title: "Home",
                        icon: Some(Icon::HOME.clone()),
                        url: Some("/"),
                        permissions: vec![""],
                        items: None,
                    },
                    NavItem {
                        title: "Create User",
                        icon: Some(Icon::USER.clone()),
                        url: Some("/users"),
                        permissions: vec![""],
                        items: None,
                    },
                    NavItem {
                        title: "Products",
                        icon: Some(Icon::PRODUCTS.clone()),
                        url: None,
                        permissions: vec![""],
                        items: Some(vec![
                            NavItem {
                                title: "products",
                                url: Some("/products"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Create New Product",
                                url: Some("/products/create-new"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Update a Product",
                                url: Some("/products/update-product"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                        ]),
                    },
                ],
            }],
        }
    }
}

#[component]
pub fn SidebarNavigation() -> impl IntoView {
    let nav_config = SidebarNavConfig::new();
    let user_permissions: &[&str] = &[""];

    // Get current path (simplified - you'd use your router's current path)
    let (active_path, _) = signal("/");

    // Helper function to check permissions
    let has_permission = |required_permissions: &[&str], user_permissions: &[&str]| {
        required_permissions
            .iter()
            .all(|permission| user_permissions.contains(permission))
    };

    view! {
        <Sidebar
            collapsible=SidebarCollapsible::Icon
            variant=SidebarVariant::Sidebar
            class="bg-background/80"
        >
            <SidebarHeader>
                <div class="flex items-center p-4 group-data-[collapsible=icon]:px-0 group-data-[collapsible=icon]:ml-1.5">
                    <div class="h-6 w-6 bg-primary rounded flex items-center justify-center">
                        <span class="text-white text-xs font-bold">"M"</span>
                    </div>
                    <span class="ml-2 font-semibold text-sm group-data-[collapsible=icon]:hidden">"Menu"</span>
                </div>
            </SidebarHeader>

            <SidebarContent>
                {nav_config.groups.into_iter().enumerate().map(|(group_index, group)| {
                    let group_class = if group_index == 1 {
                        "group-data-[collapsible=icon]:hidden"
                    } else {
                        ""
                    };

                    view! {
                        <div class=format!("p-4 group-data-[collapsible=icon]:p-0 group-data-[collapsible=icon]:ml-1.5 {}", group_class)>
                            {if !group.label.is_empty() {
                                view! {
                                    <div class="text-xs font-medium text-muted-foreground mb-2">
                                        {group.label}
                                    </div>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}

                            <div class="space-y-1">
                                {group.items.into_iter()
                                    .filter(|item| has_permission(&item.permissions, user_permissions))
                                    .map(|item| {
                                        let item_clone = item.clone();

                                        if let Some(sub_items) = item.items {
                                            // Create a reactive signal for parent active state
                                            let items_clone = sub_items.clone();
                                            let is_parent_active = Signal::derive(move || {
                                                items_clone.iter().any(|sub_item| {
                                                    sub_item.url.as_ref().map_or(false, |u| active_path.get() == *u)
                                                })
                                            });

                                            // Get initial value for default_open
                                            let default_open = is_parent_active.get_untracked();

                                            view! {
                                                <Collapsible default_open=default_open class="group/collapsible ml-0".to_string()>
                                                    <CollapsibleTrigger as_child=true>
                                                        <div class=move || {
                                                            let base_classes = "flex items-center justify-between p-2 rounded-md hover:bg-muted transition-colors cursor-pointer w-full";
                                                            let active_classes = if is_parent_active.get() {
                                                                "bg-muted text-primary font-medium"
                                                            } else {
                                                                "text-foreground"
                                                            };
                                                            format!("{} {}", base_classes, active_classes)
                                                        }>
                                                            <div class="flex items-center gap-2">
                                                                {item_clone.icon.map(|icon| view! { <IconView icon=icon /> })}
                                                                <span class="group-data-[collapsible=icon]:hidden">{item.title}</span>
                                                            </div>
                                                            <IconView
                                                                icon=Icon::CHEVRON_RIGHT.clone()
                                                                class="group-data-[collapsible=icon]:hidden ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"
                                                            />
                                                        </div>
                                                    </CollapsibleTrigger>
                                                    <CollapsibleContent class="group-data-[collapsible=icon]:hidden">
                                                        <div class="ml-4 pl-2 border-l border-border space-y-1 mt-1">
                                                            {sub_items.into_iter()
                                                                .filter(|sub_item| has_permission(&sub_item.permissions, user_permissions))
                                                                .map(|sub_item| {
                                                                    let sub_url = sub_item.url;
                                                                    let is_sub_active = Signal::derive(move || {
                                                                        sub_url.map_or(false, |u| active_path.get() == u)
                                                                    });

                                                                    view! {
                                                                        <A
                                                                            href=sub_item.url.unwrap_or("")
                                                                            attr:class=move || {
                                                                                let base_classes = "flex items-center gap-2 p-2 rounded-md hover:bg-muted transition-colors";
                                                                                let active_classes = if is_sub_active.get() {
                                                                                    "bg-muted text-primary font-medium"
                                                                                } else {
                                                                                    "text-foreground"
                                                                                };
                                                                                format!("{} {}", base_classes, active_classes)
                                                                            }
                                                                        >
                                                                            <IconView icon=Icon::ARROW_RIGHT.clone() />
                                                                            <span>{sub_item.title}</span>
                                                                        </A>
                                                                    }
                                                                }).collect_view()}
                                                        </div>
                                                    </CollapsibleContent>
                                                </Collapsible>
                                            }.into_any()
                                        } else {
                                            let item_url = item.url;
                                            let is_item_active = Signal::derive(move || {
                                                item_url.map_or(false, |u| active_path.get() == u)
                                            });

                                            view! {
                                                <A
                                                    href=item.url.unwrap_or("")
                                                    attr:class=move || {
                                                        let base_classes = "flex items-center gap-2 p-2 rounded-md hover:bg-muted transition-colors";
                                                        let active_classes = if is_item_active.get() {
                                                            "bg-muted text-primary font-medium"
                                                        } else {
                                                            "text-foreground"
                                                        };
                                                        format!("{} {}", base_classes, active_classes)
                                                    }
                                                >
                                                    {item.icon.map(|icon| view! { <IconView icon=icon /> })}
                                                    <span class="group-data-[collapsible=icon]:hidden">{item.title}</span>
                                                </A>
                                            }.into_any()
                                        }
                                    }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </SidebarContent>

            <SidebarFooter>
                <div class="p-4 border-t border-border">
                    <div class="text-xs text-muted-foreground">
                        "Sidebar Footer Content"
                    </div>
                </div>
            </SidebarFooter>
        </Sidebar>
    }
}
