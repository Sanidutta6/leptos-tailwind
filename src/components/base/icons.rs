use leptos::prelude::*;

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
    pub const LOADER_CIRCLE: Icon = Icon {
        path: "M21 12a9 9 0 1 1-6.219-8.56",
    };
}

// Icon component
#[component]
pub fn IconView(
    #[prop(optional)] icon: Option<Icon>,
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
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
            class=format!("size-5 {}", class)
        >
            // Priority 1: Render icon if provided
            {if let Some(icon) = icon {
                view! { <path d={icon.path} /> }.into_any()
            } else if let Some(children) = children {
                // Priority 2: Render children if no icon but children provided
                children().into_any()
            } else {
                // Neither icon nor children provided
                view! {}.into_any()
            }}
        </svg>
    }
}
