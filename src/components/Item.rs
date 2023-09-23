use leptos::*;
use uuid::Uuid;

#[component]
pub fn Item<F>(id: Uuid, description: String, done: bool, on_toggle_done: F) -> impl IntoView
where
    F: Fn(Uuid) -> () + 'static,
{
    let button_text = if done { "Undone" } else { "Done" };

    let handle_click = move |_| {
        on_toggle_done(id);
    };

    view! {
      <li
        style:border-bottom="1px solid rgba(0, 0, 0, .3)"
        style:margin-bottom="10px"
        style:display="flex"
        style:justify-content="space-between"
      >
        <span style:text-decoration={if done {"line-through"} else {"unset"}}>
          {description}
        </span>

        <button
          style:background-color="transparent"
          style:padding="5px 10px"
          style:border-radius="4px"
          style:margin-bottom="5px"
          on:click=handle_click
        >
          {button_text}
        </button>
      </li>
    }
}
