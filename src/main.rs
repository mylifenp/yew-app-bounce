use bounce::{use_atom, use_atom_value, Atom, BounceRoot};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::InputEvent;

#[derive(PartialEq, Atom)]
pub struct Username {
    value: String,
}

#[derive(PartialEq, Atom)]
pub struct UserProfile {
    email: String,
    first_name: String,
    last_name: String,
}

impl Default for UserProfile {
    fn default() -> Self {
        UserProfile {
            email: "test@test.de".into(),
            first_name: "John".into(),
            last_name: "Doe".into(),
        }
    }
}

impl Default for Username {
    fn default() -> Self {
        Username {
            value: "John Doe".into(),
        }
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
    <BounceRoot>
        <main>
            <Reader />
            <Setter />
            <div class="mx-auto px-5 container py-10 text-white text-xl grow-1">
                <button class="bg-red-500 rounded-md cursor-pointer" {onclick}>{" + 1"}</button>
                <p class="text-slate-800">{*counter}</p>
            </div>
            <img class="bg-slate-100" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <GetProfile />
        </main>
    </BounceRoot>
    }
}

#[function_component]
fn Reader() -> Html {
    let username = use_atom_value::<Username>();
    html! {<div>{"Hello, "}{&username.value}</div>}
}

#[function_component]
fn Setter() -> Html {
    let username = use_atom::<Username>();
    let on_text_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(Username {
                value: input.value(),
            })
        })
    };

    html! {
        <div class="mb4-4 p-5">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="username">
            {"Username"}
            </label>
            <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" label="username" type="text" oninput={on_text_input} value={username.value.to_string()} />
        </div>
    }
}

#[function_component]
fn GetProfile() -> Html {
    let profile = use_atom_value::<UserProfile>();
    html! {<div>
        <div>{&profile.email}</div>
        <div>{&profile.first_name}</div>
        <div>{&profile.last_name}</div>
    </div>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
