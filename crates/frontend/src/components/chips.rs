use leptos::*;
use models::UINudge;

#[component]
pub fn ChipsPanel(
    nudges: ReadSignal<Vec<UINudge>>,
) -> impl IntoView {
    view! {
        <div class="chips-panel">
            <For
                each=move || nudges.get()
                key=|nudge| nudge.id.clone()
                children=move |nudge| {
                    view! {
                        <button class="chip" data-priority=nudge.priority>
                            {&nudge.label}
                        </button>
                    }
                }
            />
        </div>
    }
}
