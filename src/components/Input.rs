use leptos::*;

#[component]
pub fn Input(
    node_ref: NodeRef<leptos_dom::html::Input>,
    name: String,
    placeholder: String,
) -> impl IntoView {
    view! {
      <input
        placeholder=placeholder
        name=name
        node_ref=node_ref
        style:padding="10px 20px"
        style:width="100%"
      />
    }
}
