use yew::AttrValue;

#[derive(Clone, PartialEq)]
pub struct ItemStruct {
    pub id: AttrValue,
    pub name: AttrValue,
    pub prio: i32,
}
