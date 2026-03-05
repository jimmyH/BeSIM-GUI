use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="card">
            <p>{ "Test app using Yew to consume the BeSMART REST API." }</p>
        </div>
    }
}
