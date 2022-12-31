use gloo::{console::error, utils::document};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, InputEvent, MouseEvent};
use yew::{
    html,
    prelude::{function_component, Html},
    use_state_eq, AttrValue, Callback, Properties, TargetCast,
};

pub mod item_struct;

#[derive(Properties, PartialEq)]
pub struct AddItemProps {
    pub callback_add_item: Callback<AttrValue>,
}

#[function_component]
pub fn AddItem(props: &AddItemProps) -> Html {
    let new_item_name = use_state_eq(|| AttrValue::from(""));
    let new_item_name_onclick = new_item_name.clone();
    let new_item_name_oninput = new_item_name.clone();
    let new_item_name_html = new_item_name.clone();
    let callback_add_item = props.callback_add_item.clone();

    let onclick = Callback::from(move |e: MouseEvent| {
        if let Some(_target) = e.target_dyn_into::<HtmlElement>() {
            let item_input: HtmlInputElement = document()
                .get_element_by_id("item-input")
                .expect("Missing item-input element")
                .unchecked_into();
            callback_add_item.emit(AttrValue::from(item_input.value().clone()));
            new_item_name_onclick.set(AttrValue::from(""));
            // error!(&item_input);
        }
    });

    let oninput = Callback::from(move |e: InputEvent| {
        if let Some(_target) = e.target_dyn_into::<HtmlElement>() {
            let item_input: HtmlInputElement = document()
                .get_element_by_id("item-input")
                .expect("Missing item-input element")
                .unchecked_into();
            new_item_name_oninput.set(AttrValue::from(item_input.value().clone()));
            error!(&item_input.value());
        }
    });

    html! {
        <div style="display:flex; flex-flow:column">
            { new_item_name_html.to_string() }
            <ui5-input id="item-input" style="min-width: 100%; margin-top: 5px; margin-bottom: 5px;" show-clear-icon="true"
                placeholder={"Add New Task"} value={ new_item_name_html.to_string() } { oninput }></ui5-input>
            <ui5-button id="item-submit-button" design="Emphasized" { onclick } > {"Add Task"} </ui5-button>
        </div>
    }
}
