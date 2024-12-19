use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
}

#[function_component(App)]
fn app() -> Html {
    let items = use_state(|| vec![
        Item { id: 1, name: "Item 1".to_string(), quantity: 10 },
        Item { id: 2, name: "Item 2".to_string(), quantity: 20 },
    ]);

    let add_item = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut new_items = (*items).clone();
            new_items.push(Item { id: new_items.len() as u32 + 1, name: "New Item".to_string(), quantity: 1 });
            items.set(new_items);
        })
    };

    html! {
        <div>
            <h1>{ "Items" }</h1>
            <ul>
                { for items.iter().map(|item| html! {
                    <li>{ format!("{} (Quantity: {})", item.name, item.quantity) }</li>
                }) }
            </ul>
            <button onclick={add_item}>{ "Add Item" }</button>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}