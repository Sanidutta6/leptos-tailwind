# Leptos-Tailwind

A starter/template repository combining the Leptos (Rust) web framework with Tailwind CSS for styling. This project is intended to make it easy to build reactive Rust + WASM frontend apps with Tailwind utility-first styling.

If you're looking for a minimal, modern stack with Rust on the frontend and Tailwind for styles, this repo gives you a solid starting point.

Links
- Leptos: https://github.com/leptos-rs/leptos
- Tailwind CSS: https://tailwindcss.com
- Trunk (recommended dev server / bundler for Leptos WASM): https://trunkrs.dev
- Fake Store: https://fakestoreapi.com/

Features
- Leptos for reactive UI and optional SSR
- Tailwind CSS for utility-first styling
- Example Tailwind setup and build/watch scripts
- Development workflow notes for both WASM (client-side)
- REST Api calling

Prerequisites
- Rust (stable) — install via rustup: https://rustup.rs
- Recommended: trunk (for Leptos WASM dev server) — optional for SSR apps

Quick install tools (examples)
```sh
# Install trunk (optional, recommended for WASM dev)
cargo install --locked trunk

# Add WASM target (for client-side builds)
rustup target add wasm32-unknown-unknown
```

Project layout (typical)
- Cargo.toml — Rust workspace / crate config
- Trunk.toml - For adding tailwind css through trunk support
- public/ — static assets served to browser
- src/
  - main.rs — Leptos app entrypoint
  - app.rs — for defining routes and others
  - components/ — components
  - layouts/ — layouts for Private and Public Routes
  - pages/ — for defining pages
  - api/ — for defining functions for different API points to hit.
- styles/ — Tailwind input CSS (e.g. input.css)

Setup (recommended)
1. Clone the repo
```sh
git clone https://github.com/Sanidutta6/leptos-tailwind.git
cd leptos-tailwind
```

2. Ensure Rust toolchain is installed and the WASM target is added
```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Install trunk (optional but recommended for Leptos WASM)
```sh
cargo install --locked trunk
```

Tailwind setup
Create an input CSS file (example: styles/input.css) with the Tailwind directives:
```css
@import "tailwindcss";
```

Development (WASM client-side with Trunk)
This is the common path for building Leptos apps as a client-side / SPAs using WASM.

1. Run Trunk to serve the app (builds WASM and serves static files)
```sh
trunk serve --open
```

This will:
- Build the Rust -> WASM artifacts
- Rebuild automatically on Rust changes
- Serve the site and reload on changes

Production build (WASM)

1. Build optimized WASM bundle (with trunk)
```sh
trunk build --release
# or if you use another bundler, run your corresponding build command
```

Environment variables
If your app requires environment variables (API URLs, feature flags), prefer:
- .env file for secret variables

Acknowledgements
- Leptos (Rust) — https://github.com/leptos-rs/leptos
- Tailwind CSS — https://tailwindcss.com
- Fake Store — https://fakestoreapi.com/
