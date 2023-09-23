mod components;

use leptos::{html::Input, *};
use uuid::Uuid;
use web_sys::SubmitEvent;

#[derive(Clone)]
struct TodoItem {
    id: Uuid,
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
                id: Uuid::new_v4(),
                description: typed_value,
                done: false,
            })
        });

        form_element.get().expect("need an form element").reset();
    };

    let on_toggle_done = move |id: Uuid| {
        let updated: Vec<TodoItem> = items
            .get()
            .into_iter()
            .map(|mut item| {
                if item.id == id {
                    item.done = !item.done;
                }
                item
            })
            .collect();

        set_items.set(updated);
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
                            id=item.id
                            description=item.description.clone()
                            done=item.done
                            on_toggle_done=on_toggle_done
                        />
                    })
                    .collect_view()}
                </ul>
            </div>
        }
    })
}
