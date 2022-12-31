#[path = "additem/additem.rs"]
mod additem;
#[path = "table/table.rs"]
mod table;

use additem::{item_struct::ItemStruct, AddItem};
use table::Table;
use yew::{
    html,
    prelude::{function_component, Html},
    use_state, AttrValue, Callback, UseStateHandle,
};

#[function_component]
fn App() -> Html {
    let items: UseStateHandle<Vec<ItemStruct>> = use_state(|| vec![]);
    let items_callback_add_item = items.clone();
    let items_inc_prio = items.clone();
    let items_dec_prio = items.clone();
    let items_delete_item = items.clone();

    let callback_add_item: Callback<AttrValue> = Callback::from(move |item_name: AttrValue| {
        let mut new_items: Vec<ItemStruct> = vec![];
        for item in items_callback_add_item.iter() {
            new_items.push(item.clone());
        }
        let new_id = if items_callback_add_item.len() == 0 {
            AttrValue::from("0")
        } else {
            AttrValue::from(
                (new_items[items_callback_add_item.len() - 1]
                    .id
                    .parse::<u32>()
                    .unwrap()
                    + 1)
                .to_string(),
            )
        };
        new_items.push(ItemStruct {
            id: new_id,
            name: item_name,
            prio: 0,
        });
        items_callback_add_item.set(new_items);
    });

    let increment_prio: Callback<AttrValue> = Callback::from(move |item_id: AttrValue| {
        let mut new_items: Vec<ItemStruct> = vec![];
        for item in items_inc_prio.iter() {
            if item.id == item_id {
                new_items.push(ItemStruct {
                    id: item.id.clone(),
                    name: item.name.clone(),
                    prio: item.prio + 1,
                });
            } else {
                new_items.push(item.clone());
            }
        }
        items_inc_prio.set(new_items);
    });

    let decrement_prio: Callback<AttrValue> = Callback::from(move |item_id: AttrValue| {
        let mut new_items: Vec<ItemStruct> = vec![];
        for item in items_dec_prio.iter() {
            if item.id == item_id {
                new_items.push(ItemStruct {
                    id: item.id.clone(),
                    name: item.name.clone(),
                    prio: item.prio - 1,
                });
            } else {
                new_items.push(item.clone());
            }
        }
        items_dec_prio.set(new_items);
    });

    let delete_item: Callback<AttrValue> = Callback::from(move |item_id: AttrValue| {
        let mut new_items: Vec<ItemStruct> = vec![];
        for item in items_delete_item.iter() {
            if item.id != item_id {
                new_items.push(item.clone());
            }
        }
        items_delete_item.set(new_items);
    });

    html! {
        <div>
            <Table { items } { increment_prio } { decrement_prio } { delete_item } />
            <AddItem { callback_add_item } />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
