use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let (recipes, set_recipes) = create_signal(Vec::<models::Recipe>::new());

    // TODO: Fetch recipes from API

    view! {
        <div class="container">
            <h1>"Realtime Recipe Assistant"</h1>

            <div class="webrtc-note">
                <h2>"⚠️ WebRTC Implementation Note"</h2>
                <p>
                    "WebRTC support in Rust WASM (Leptos) is currently experimental. "
                    "While the backend is fully implemented, the frontend WebRTC client "
                    "may require a JavaScript-based implementation (React/Next.js) for production use."
                </p>
                <p>
                    "See README for alternative frontend options."
                </p>
            </div>

            <div class="recipe-list">
                <h2>"Your Recipes"</h2>
                <For
                    each=move || recipes.get()
                    key=|r| r.id
                    children=move |recipe| {
                        view! {
                            <div class="recipe-card">
                                <h3>{&recipe.title}</h3>
                                <p>{recipe.description.as_deref().unwrap_or("No description")}</p>
                                <a href=format!("/recipe/{}", recipe.id)>"View Recipe"</a>
                            </div>
                        }
                    }
                />
            </div>

            <div class="voice-controls">
                <button class="btn-primary">"🎤 Start Voice Session"</button>
                <p class="hint">"Click to start a realtime cooking session"</p>
            </div>
        </div>
    }
}
