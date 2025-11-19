use leptos::*;
use leptos_router::*;

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        // TODO: Implement actual login API call
        spawn_local(async move {
            // Placeholder
            set_error.set(Some("Login not yet implemented".to_string()));
        });
    };

    view! {
        <div class="login-container">
            <h1>"Login"</h1>

            {move || error.get().map(|err| view! {
                <div class="error">{err}</div>
            })}

            <form on:submit=on_submit>
                <div class="form-group">
                    <label>"Email"</label>
                    <input
                        type="email"
                        placeholder="you@example.com"
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        prop:value=email
                    />
                </div>

                <div class="form-group">
                    <label>"Password"</label>
                    <input
                        type="password"
                        placeholder="••••••••"
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        prop:value=password
                    />
                </div>

                <button type="submit" class="btn-primary">"Login"</button>
            </form>

            <p class="hint">
                "Don't have an account? "
                <a href="/register">"Sign up"</a>
            </p>
        </div>
    }
}
