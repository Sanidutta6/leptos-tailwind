use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Login() -> impl IntoView {
    view! {
    <div class="flex items-center justify-center w-full px-4 py-8">
        <form class="flex w-full flex-col max-w-1/3">
            <h2 class="text-4xl font-medium text-gray-900">"Sign in"</h2>
            <p class="mt-4 text-base text-gray-500/90">
                "Please enter email and password to access."
            </p>
            <div class="mt-10">
                <label class="font-medium">"Email"</label>
                <input
                    placeholder="Please enter your email"
                    class="mt-2 rounded-md ring ring-gray-200 focus:ring-2 focus:ring-indigo-600 outline-none px-3 py-3 w-full"
                    required
                    type="email"
                    name="email"
                />
            </div>

            <div class="mt-6">
                <label class="font-medium">"Password"</label>
                <input
                    placeholder="Please enter your password"
                    class="mt-2 rounded-md ring ring-gray-200 focus:ring-2 focus:ring-indigo-600 outline-none px-3 py-3 w-full"
                    required
                    type="password"
                    name="password"
                />
            </div>

            <button
                type="submit"
                class="mt-8 py-3 w-full cursor-pointer rounded-md bg-indigo-600 text-white transition hover:bg-indigo-700"
            >
                "Login"
            </button>
            <p class="text-center py-8">
                "Don't have an account? " <A href="/signup" attr:class="text-indigo-600 hover:underline">"Sign up"</A>
            </p>
        </form>
    </div>
    }
}
