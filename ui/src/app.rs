use crate::pages::{home::Home, todo::TodoDetail};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Title text="Todo App"/>
        <Stylesheet id="leptos" href="/input.css"/>
        <Body class="bg-slate-900 text-slate-100 antialiased min-h-screen selection:bg-teal-500 selection:text-white"/>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home ssr=SsrMode::PartiallyBlocked/>
                    <Route path="/todos/:id" view=TodoDetail ssr=SsrMode::PartiallyBlocked/>
                </Routes>
            </main>
        </Router>
    }
}
