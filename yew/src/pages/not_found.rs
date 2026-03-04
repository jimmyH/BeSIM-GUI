use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="card">
            <h2>{"Page not found"}</h2>
        </div>
    }
}
