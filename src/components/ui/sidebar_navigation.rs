use crate::components::base::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::base::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarGroupLabel, SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuButtonVariant, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem, SidebarVariant
};
use leptos::prelude::*;
use leptos_router::components::A;

// Define icon struct to match React component
#[derive(Clone)]
pub struct Icon {
    pub path: &'static str,
}

impl Icon {
    pub const HOME: Icon = Icon {
        path: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
    };
    pub const SQUARE_LIBRARY: Icon = Icon {
        path: "M4 19a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4Zm0 0a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2M9 5a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2H9V5Z",
    };
    pub const BINOCULARS: Icon = Icon {
        path: "M12 3v2m0 14v2M3 12h2m14 0h2M5.6 5.6l1.4 1.4M17.4 5.6 16 7m-8 9.2L5.6 18.4M16 17l1.4 1.4M14 12a2 2 0 1 1-4 0 2 2 0 0 1 4 0Z",
    };
    pub const CHEVRON_RIGHT: Icon = Icon {
        path: "M9 18l6-6-6-6",
    };
    pub const ARROW_RIGHT: Icon = Icon {
        path: "M5 12h14m-7-7l7 7-7 7",
    };
}

// Icon component
#[component]
pub fn Icon(#[prop(into)] icon: Icon, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
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
            class=format!("size-4 {}", class)
        >
            <path d={icon.path} />
        </svg>
    }
}

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
                        title: "Installation",
                        icon: Some(Icon::SQUARE_LIBRARY.clone()),
                        url: Some("/installation"),
                        permissions: vec![""],
                        items: None,
                    },
                    NavItem {
                        title: "Components",
                        icon: Some(Icon::BINOCULARS.clone()),
                        url: None,
                        permissions: vec![""],
                        items: Some(vec![
                            NavItem {
                                title: "Accordion",
                                url: Some("/components/accordion"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Alert",
                                url: Some("/components/alert"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Alert Dialog",
                                url: Some("/components/alert-dialog"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Aspect Ratio",
                                url: Some("/components/aspect-ratio"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Avatar",
                                url: Some("/components/avatar"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Badge",
                                url: Some("/components/badge"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Breadcrumb",
                                url: Some("/components/breadcrumb"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Button",
                                url: Some("/components/button"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Card",
                                url: Some("/components/card"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Checkbox",
                                url: Some("/components/checkbox"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Collapsible",
                                url: Some("/components/collapsible"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Dialog",
                                url: Some("/components/dialog"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Drawer",
                                url: Some("/components/drawer"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Dropdown Menu",
                                url: Some("/components/dropdown-menu"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Hover Card",
                                url: Some("/components/hover-card"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Input",
                                url: Some("/components/input"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Label",
                                url: Some("/components/label"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Pagination",
                                url: Some("/components/pagination"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Popover",
                                url: Some("/components/popover"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Progress",
                                url: Some("/components/progress"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Skeleton",
                                url: Some("/components/skeleton"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Radio Group",
                                url: Some("/components/radio-group"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Scroll Area",
                                url: Some("/components/scroll-area"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Select List",
                                url: Some("/components/select-list"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Separator",
                                url: Some("/components/separator"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Slider",
                                url: Some("/components/slider"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Switch",
                                url: Some("/components/switch"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Table",
                                url: Some("/components/table"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Tabs",
                                url: Some("/components/tabs"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Textarea",
                                url: Some("/components/textarea"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Toast",
                                url: Some("/components/toast"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Toggle",
                                url: Some("/components/toggle"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Toggle Group",
                                url: Some("/components/toggle-group"),
                                permissions: vec![""],
                                icon: None,
                                items: None,
                            },
                            NavItem {
                                title: "Tooltip",
                                url: Some("/components/tooltip"),
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
pub fn SideNav() -> impl IntoView {
    let nav_config = SidebarNavConfig::new();
    let user_permissions = vec![""];

    // Get current path (simplified - you'd use your router's current path)
    let (active_path, _) = signal("/");

    // Helper functions
    let is_active = move |url: Option<&str>| {
        if let Some(url) = url {
            active_path.get() == url
        } else {
            false
        }
    };

    let is_active_parent = move |items: &[NavItem]| items.iter().any(|item| is_active(item.url));

    let has_permission = |required_permissions: &[&str], user_permissions: Vec<&str>| {
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
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton>
                                <div class="flex items-center">
                                    // Replace with your logo
                                    <div class="h-6 w-6 bg-primary rounded flex items-center justify-center">
                                        <span class="text-white text-xs font-bold">"M"</span>
                                    </div>
                                    <h1 class="ml-2 text-2xl font-bold">"MPB Portal"</h1>
                                </div>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarHeader>
                <SidebarContent>
                    {nav_config.groups.into_iter().enumerate().map(|(group_index, group)| {
                        let group_class = if group_index == 1 {
                            "group-data-[collapsible=icon]:hidden"
                        } else {
                            ""
                        };

                        view! {
                            <SidebarGroup class=group_class.to_string()>
                                <SidebarGroupLabel>{group.label}</SidebarGroupLabel>
                                <SidebarMenu>
                                    {group.items.into_iter()
                                        .filter(|item| has_permission(&item.permissions, user_permissions))
                                        .map(|item| {
                                            let item_clone = item.clone();
                                            let is_parent_active = item.items.as_ref()
                                                .map(|items| is_active_parent(items))
                                                .unwrap_or(false);

                                            view! {
                                                <SidebarMenuItem>
                                                    {if let Some(sub_items) = item.items {
                                                        let default_open = is_parent_active;

                                                        view! {
                                                            <Collapsible default_open=default_open class="group/collapsible".to_string()>
                                                                <CollapsibleTrigger as_child=true>
                                                                    <SidebarMenuButton
                                                                        is_active=is_parent_active
                                                                        variant=SidebarMenuButtonVariant::Default  // or whichever variant you want
                                                                        size=SidebarMenuButtonSize::Default        // or whichever size you want
                                                                        title=item.title                           // Use the item title for the title attribute
                                                                        // class=move || {
                                                                        //     let extra_class = if is_parent_active {
                                                                        //         "bg-ternary-background text-primary font-medium".to_string()
                                                                        //     } else {
                                                                        //         "".to_string()
                                                                        //     };
                                                                        //     format!("{}", extra_class)
                                                                        // }
                                                                    >
                                                                        {item_clone.icon.map(|icon| view! { <Icon icon=icon /> })}
                                                                        <span>{item.title}</span>
                                                                        <Icon
                                                                            icon=Icon::CHEVRON_RIGHT.clone()
                                                                            class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"
                                                                        />
                                                                    </SidebarMenuButton>
                                                                </CollapsibleTrigger>
                                                                <CollapsibleContent>
                                                                    <SidebarMenuSub>
                                                                        {sub_items.into_iter()
                                                                            .filter(|sub_item| has_permission(&sub_item.permissions, user_permissions))
                                                                            .map(|sub_item| {
                                                                                let is_sub_active = is_active(sub_item.url);

                                                                                view! {
                                                                                    <SidebarMenuSubItem>
                                                                                        <SidebarMenuSubButton
                                                                                            as_child=true
                                                                                            // class=move || {
                                                                                            //     let active_class = if is_sub_active {
                                                                                            //         "bg-ternary-background text-primary font-medium [&_svg]:text-primary"
                                                                                            //     } else {
                                                                                            //         ""
                                                                                            //     };
                                                                                            //     format!("{}", active_class)
                                                                                            // }
                                                                                        >
                                                                                            <A href=sub_item.url.unwrap_or("")>
                                                                                                <Icon icon=Icon::ARROW_RIGHT.clone() />
                                                                                                <span>{sub_item.title}</span>
                                                                                            </A>
                                                                                        </SidebarMenuSubButton>
                                                                                    </SidebarMenuSubItem>
                                                                                }.into_any()
                                                                            }).collect_view()}
                                                                    </SidebarMenuSub>
                                                                </CollapsibleContent>
                                                            </Collapsible>
                                                        }.into_any()
                                                    } else {
                                                        let is_item_active = is_active(item.url);

                                                        view! {
                                                            <SidebarMenuButton
                                                                as_child=true
                                                                // class=move || {
                                                                //     let active_class = if is_item_active {
                                                                //         "bg-ternary-background text-primary font-medium"
                                                                //     } else {
                                                                //         ""
                                                                //     };
                                                                //     format!("{}", active_class)
                                                                // }
                                                            >
                                                                <A href=item.url.unwrap_or("")>
                                                                    {item.icon.map(|icon| view! { <Icon icon=icon /> })}
                                                                    <span>{item.title}</span>
                                                                </A>
                                                            </SidebarMenuButton>
                                                        }.into_any()
                                                    }}
                                                </SidebarMenuItem>
                                            }
                                        }).collect_view()}
                                </SidebarMenu>
                            </SidebarGroup>
                        }
                    }).collect_view()}
                </SidebarContent>
            </Sidebar>
        }
}
