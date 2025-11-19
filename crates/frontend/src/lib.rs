mod components;
mod api;
mod webrtc;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use components::{Home, RecipeView, Login};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="Realtime Recipe Assistant"/>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/login" view=Login/>
                    <Route path="/recipe/:id" view=RecipeView/>
                </Routes>
            </main>
        </Router>
    }
}

/// Hydrate the app when running in the browser
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
