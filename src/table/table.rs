use gloo::dialogs::confirm;
use web_sys::{HtmlElement, MouseEvent};
use yew::{
    html,
    prelude::{function_component, Html},
    AttrValue, Callback, Properties, TargetCast, UseStateHandle,
};

use crate::additem::item_struct::ItemStruct;

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub items: UseStateHandle<Vec<ItemStruct>>,
    pub increment_prio: Callback<AttrValue>,
    pub decrement_prio: Callback<AttrValue>,
    pub delete_item: Callback<AttrValue>,
}

#[function_component]
pub fn Table(props: &TableProps) -> Html {
    let increment_prio_cb = props.increment_prio.clone();
    let decrement_prio_cb = props.decrement_prio.clone();
    let delete_item_cb = props.delete_item.clone();

    let handle_prio_inc = Callback::from(move |e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let item_id_option = target.get_attribute("data-id");
            if let Some(vote_id) = item_id_option {
                increment_prio_cb.emit(AttrValue::from(vote_id));
            }
        }
    });

    let handle_prio_dec = Callback::from(move |e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let item_id_option = target.get_attribute("data-id");
            if let Some(vote_id) = item_id_option {
                decrement_prio_cb.emit(AttrValue::from(vote_id));
            }
        }
    });

    let handle_delete_item = Callback::from(move |e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let item_id_option = target.get_attribute("data-id");
            if let Some(vote_id) = item_id_option {
                let delete_confirmed = confirm("Delete item?");
                if delete_confirmed {
                    delete_item_cb.emit(AttrValue::from(vote_id));
                }
            }
        }
    });

    html! {
        if props.items.len() > 0 {
            <ui5-table id="table">
                <ui5-table-column slot="columns">
                    <span style="line-height: 1.4rem">{ "Task name" }</span>
                </ui5-table-column>
                <ui5-table-column slot="columns" class="todoRightAlignment">
                    <span style="line-height: 1.4rem">{ "Priority" }</span>
                </ui5-table-column>
                <ui5-table-column slot="columns" class="todoWidth64">
                    <span style="line-height: 1.4rem">{ "" }</span>
                </ui5-table-column>
                <ui5-table-column slot="columns" class="todoWidth64">
                    <span style="line-height: 1.4rem">{ "" }</span>
                </ui5-table-column>
            {
                {
                let mut i = -1;
                let mut vec_items = vec![];
                for item in props.items.iter(){
                    vec_items.push(item.clone());
                }
                vec_items.sort_by(|a, b| {b.prio.cmp(&a.prio)});
                vec_items.iter().map(|item|
                {
                    i += 1;
                    html!{
                        <ui5-table-row>
                            <ui5-table-cell>
                                <div style="display: flex">
                                    <ui5-title level="H2">{ item.name.to_string() }</ui5-title>
                                    if i == 0 {
                                        <ui5-badge style="margin-top: 8px; margin-left: 8px;"> { "Priority Task" } </ui5-badge>
                                    }
                                </div>
                            </ui5-table-cell>
                            <ui5-table-cell style="text-align: right">
                                <ui5-title level="H3">{ item.prio }</ui5-title>
                            </ui5-table-cell>
                            <ui5-table-cell style="text-align: right">
                                <ui5-button icon="slim-arrow-up" design="Positive" data-id={ item.id.to_string() } onclick={&handle_prio_inc} />
                                <ui5-button icon="slim-arrow-down" design="Negative" data-id={ item.id.to_string() } onclick={&handle_prio_dec}/>
                            </ui5-table-cell>
                            <ui5-table-cell>
                                <ui5-button icon="delete" design="Transparent" data-id={ item.id.to_string() } onclick={&handle_delete_item}/>
                            </ui5-table-cell>
                        </ui5-table-row>
                    }
                }).collect::<Html>()
                }
            }
            </ui5-table>
        }
        else {
            <ui5-illustrated-message name="EmptyList" />
        }
    }
}
