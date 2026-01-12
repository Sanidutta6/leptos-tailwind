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
    pub const USER: Icon = Icon {
        path: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
    };

    pub const USER_CIRCLE: Icon = Icon {
        path: "M18.364 5.636a9 9 0 010 12.728m0 0a9 9 0 01-12.728 0M16 12a4 4 0 11-8 0 4 4 0 018 0z",
    };

    pub const USER_SQUARE: Icon = Icon {
        path: "M4 6a2 2 0 012-2h12a2 2 0 012 2v12a2 2 0 01-2 2H6a2 2 0 01-2-2V6zm8 6a3 3 0 100-6 3 3 0 000 6zm0 4a5 5 0 00-5 5h10a5 5 0 00-5-5z",
    };

    pub const PRODUCTS: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4",
    };

    pub const PACKAGE: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4",
    };

    pub const BOX: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4",
    };

    pub const SHOPPING_BAG: Icon = Icon {
        path: "M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z",
    };

    pub const SHOPPING_CART: Icon = Icon {
        path: "M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z",
    };

    pub const CART: Icon = Icon {
        path: "M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z",
    };

    pub const CART_PLUS: Icon = Icon {
        path: "M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0zm11-9h-3m0 0V5m0 3V8m0 0h3",
    };

    pub const CART_MINUS: Icon = Icon {
        path: "M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0zm11-9h-6",
    };

    pub const CART_CHECK: Icon = Icon {
        path: "M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0zm11-10l-3 3m0 0l-2-2m2 2l2-2",
    };

    pub const TAG: Icon = Icon {
        path: "M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z",
    };

    pub const PRICE_TAG: Icon = Icon {
        path: "M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
    };

    pub const INVENTORY: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4m-4-6v6m4-6v6",
    };

    pub const STORE: Icon = Icon {
        path: "M3 3v18h18V3H3zm9 15H6v-4h6v4zm0-6H6V9h6v3zm0-6H6V6h6v3zm6 12h-6v-4h6v4zm0-6h-6V9h6v3zm0-6h-6V6h6v3z",
    };

    pub const GRID: Icon = Icon {
        path: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z",
    };

    pub const LIST: Icon = Icon {
        path: "M4 6h16M4 10h16M4 14h16M4 18h16",
    };

    pub const BAR_CHART: Icon = Icon {
        path: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z",
    };

    pub const PIE_CHART: Icon = Icon {
        path: "M11 3.055A9.001 9.001 0 1020.945 13H11V3.055z M20.488 9H15V3.512A9.025 9.025 0 0120.488 9z",
    };

    pub const DOLLAR_SIGN: Icon = Icon {
        path: "M12 2v20m9-9H3",
    };

    pub const CREDIT_CARD: Icon = Icon {
        path: "M3 10h18M7 15h1m4 0h1m-7 4h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
    };

    pub const RECEIPT: Icon = Icon {
        path: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
    };

    pub const TRUCK: Icon = Icon {
        path: "M9 17a2 2 0 11-4 0 2 2 0 014 0zM19 17a2 2 0 11-4 0 2 2 0 014 0z M13 16V6a1 1 0 00-1-1H4a1 1 0 00-1 1v10a1 1 0 001 1h1m8-1a1 1 0 01-1 1H9m4-1V8a1 1 0 011-1h2.586a1 1 0 01.707.293l3.414 3.414a1 1 0 01.293.707V16a1 1 0 01-1 1h-1m-6-1a1 1 0 001 1h1M5 17a2 2 0 104 0m-4 0a2 2 0 114 0m6 0a2 2 0 104 0m-4 0a2 2 0 114 0",
    };

    pub const PACKAGE_CHECK: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4m5-11l-3 3m0 0l-2-2m2 2l2-2",
    };

    pub const PACKAGE_X: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4m5-11l-3 3m0 0l-2-2m2 2l2-2m-2 2l2 2",
    };

    pub const PACKAGE_PLUS: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4m5-11h-3m0 0V5m0 3V8m0 0h3",
    };

    pub const PACKAGE_MINUS: Icon = Icon {
        path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4m5-11h-6",
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
