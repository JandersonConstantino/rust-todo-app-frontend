use leptos::*;

#[component]
pub fn Item(description: String, done: bool) -> impl IntoView {
    let str_is_done = if done { "Finished" } else { "Pendent" };

    view! {
      <li>
        {description} - {str_is_done}
      </li>
    }
}
