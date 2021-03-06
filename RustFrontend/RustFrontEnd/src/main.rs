use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0
    });

    let onclick = {
        let state = state.clone(); //shadows the original as we can't overwrite

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 2
            })
        })
    };

    html! {
        <div>
            <button onclick={onclick}>{"+2"}</button>
            <p> { state.value } </p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}