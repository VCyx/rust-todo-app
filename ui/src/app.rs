use leptos::*;
use leptos_router::*;
use crate::pages::{home::Home, todo::TodoDetail};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/todos/:id" view=TodoDetail/>
                </Routes>
            </main>
        </Router>
    }
}
