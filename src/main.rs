mod components;

use leptos::{html::Input, *};
use web_sys::SubmitEvent;

fn main() {
    let input_value_element: NodeRef<Input> = create_node_ref();
    let form_element: NodeRef<leptos_dom::html::Form> = create_node_ref();

    let (value, set_value) = create_signal(String::from(""));

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let typed_value = input_value_element
            .get()
            .expect("need an input element")
            .value();

        set_value.set(typed_value);

        form_element.get().expect("need an form element").reset();
    };

    mount_to_body(move || {
        view! {
            <div
                style:margin="0 auto"
                style:width="60vw"
                style:padding-top="40px"
            >
                <form
                    on:submit=on_submit
                    node_ref=form_element
                    style:padding-bottom="40px"
                >
                    <components::Input
                        name=String::from("value")
                        placeholder=String::from("Type here...")
                        node_ref=input_value_element
                    />
                </form>

                <span>
                    {move || value.get()}
                </span>
            </div>
        }
    })
}
