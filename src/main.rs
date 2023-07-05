use leptos::*;

fn main() {
    mount_to_body(|cx| view! {cx,<App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! { cx,
        <button on:click=move |_| {set_count(3);}>
            "Click me: "
            {move || count.get()}
        </button>
        <button on:click= move |_| {set_count(5);}>
        "Again click me:"
        {move || count.get()}
        </button>
    }
}