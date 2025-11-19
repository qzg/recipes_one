use leptos::*;
use leptos_router::*;
use models::Recipe;

#[component]
pub fn RecipeView() -> impl IntoView {
    let params = use_params_map();
    let (recipe, set_recipe) = create_signal(Option::<Recipe>::None);

    // Fetch recipe when component mounts
    create_effect(move |_| {
        let id = params.get().get("id").cloned().unwrap_or_default();

        spawn_local(async move {
            // TODO: Fetch recipe from API
            // For now, just show loading state
        });
    });

    view! {
        <div class="recipe-view">
            {move || recipe.get().map(|r| view! {
                <div>
                    <h1>{&r.title}</h1>

                    {r.description.as_ref().map(|desc| view! {
                        <p class="description">{desc}</p>
                    })}

                    <div class="recipe-meta">
                        <span>"Serves: "{r.servings}</span>
                        <span>" | Version: "{r.version}</span>
                    </div>

                    <section class="ingredients">
                        <h2>"Ingredients"</h2>
                        <ul>
                            <For
                                each=move || r.ingredients.clone()
                                key=|ing| ing.name.clone()
                                children=move |ing| {
                                    view! {
                                        <li>
                                            {format!("{} {} {}", ing.quantity, ing.unit, ing.name)}
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </section>

                    <section class="steps">
                        <h2>"Instructions"</h2>
                        <ol>
                            <For
                                each=move || r.steps.clone()
                                key=|step| step.text.clone()
                                children=move |step| {
                                    view! {
                                        <li>{&step.text}</li>
                                    }
                                }
                            />
                        </ol>
                    </section>
                </div>
            }).unwrap_or_else(|| view! {
                <div class="loading">"Loading recipe..."</div>
            })}
        </div>
    }
}
