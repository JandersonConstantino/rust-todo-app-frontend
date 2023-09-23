mod components;

use leptos::{html::Input, *};

use web_sys::SubmitEvent;

#[derive(Clone)]
struct TodoItem {
    description: String,
    done: bool,
}

fn main() {
    let (items, set_items) = create_signal(vec![]);

    let input_value_element: NodeRef<Input> = create_node_ref();
    let form_element: NodeRef<leptos_dom::html::Form> = create_node_ref();

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let typed_value = input_value_element
            .get()
            .expect("need an input element")
            .value();

        set_items.update(move |x| {
            x.push(TodoItem {
                description: typed_value,
                done: false,
            })
        });

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

                <ul
                    style:list-style-type="none"
                    style:padding-inline="10px"
                >
                    {move || items.get().iter().map(|item| view! {
                        <components::Item
                            description=item.description.clone()
                            done=item.done
                        />
                    })
                    .collect_view()}
                </ul>
            </div>
        }
    })
}
