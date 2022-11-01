// Imports
use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    let on_click = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            })
        })
    };

    let paragraphs = (0..state.value)
        .map(|_| html! { <p class={ "inline" }>{ "" }</p> })
        .collect::<Html>();

    if state.value == 69 {
        html! {
            <div>
                <p>{"Nice!"}</p>
                <button onclick={on_click}>{"Click me!"}</button>
                { paragraphs }
            </div>
        }
    } else {
        html! {
        <div>
            <button onclick={on_click}>{ "Click me!" }</button>
                <p> { state.value } </p>
                { paragraphs }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
