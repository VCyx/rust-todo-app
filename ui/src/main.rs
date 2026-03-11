use leptos::*;
use ui::app::{load_initial_state_from_dom, App};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App initial_state=load_initial_state_from_dom()/> })
}
