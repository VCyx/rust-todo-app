use crate::api::{self, TodoModel};
use leptos::*;
use leptos_router::A;

#[component]
pub fn Home() -> impl IntoView {
    let todos = create_blocking_resource(|| (), |_| async { api::list_todos().await });
    let (input_value, set_input_value) = create_signal(String::new());

    let add_todo = Action::new(move |title: &String| {
        let title = title.clone();
        async move {
            let title = title.trim().to_string();
            if title.is_empty() {
                return Ok(None);
            }

            api::create_todo(title).await.map(Some)
        }
    });

    create_effect(move |_| {
        if let Some(result) = add_todo.value().get() {
            match result {
                Ok(Some(_)) => {
                    set_input_value.set(String::new());
                    todos.refetch();
                }
                Ok(None) => {}
                Err(error) => logging::error!("Failed to create todo: {}", error),
            }
        }
    });

    view! {
        <div class="max-w-2xl mx-auto py-12 px-6">
            <h1 class="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-teal-400 to-emerald-500 mb-8 text-center drop-shadow-sm pb-1">
                "My Todos"
            </h1>

            <div class="glass-card mb-8 p-6 transition-all duration-300">
                <form
                    on:submit=move |ev| {
                        ev.prevent_default();
                        add_todo.dispatch(input_value.get_untracked());
                    }
                    class="flex gap-4"
                >
                    <input
                        type="text"
                        class="input-field flex-grow"
                        placeholder="What needs to be done?"
                        prop:value=move || input_value.get()
                        on:input=move |ev| set_input_value.set(event_target_value(&ev))
                    />
                    <button
                        type="submit"
                        class="btn-primary whitespace-nowrap"
                        disabled=move || add_todo.pending().get()
                    >
                        "Add Todo"
                    </button>
                </form>
            </div>

            <Suspense fallback=|| view! { <div class="text-center text-slate-400 py-12 animate-pulse">"Loading todos..."</div> }>
                {move || match todos.get() {
                    Some(Ok(items)) if items.is_empty() => view! {
                        <div class="glass-card p-8 text-center text-slate-400">
                            "No todos yet. Add the first one above."
                        </div>
                    }
                        .into_view(),
                    Some(Ok(items)) => view! {
                        <div class="space-y-4">
                            <For
                                each=move || items.clone()
                                key=|todo| todo.id
                                children=move |todo: TodoModel| {
                                    let id = todo.id;
                                    view! {
                                        <A href=format!("/todos/{}", id) attr:class="block group">
                                            <div class="glass-card p-4 transition-all duration-300 hover:scale-[1.02] hover:shadow-teal-500/10 hover:border-teal-500/40 flex items-center justify-between cursor-pointer">
                                                <div class="flex items-center gap-4">
                                                    <div class=if todo.completed { "w-6 h-6 rounded-full border-2 border-emerald-500 bg-emerald-500 flex items-center justify-center transition-colors" } else { "w-6 h-6 rounded-full border-2 border-slate-600 group-hover:border-teal-400 transition-colors bg-transparent" }>
                                                        {if todo.completed {
                                                            view! { <span class="text-white text-xs">"x"</span> }
                                                        } else {
                                                            view! { <span class="hidden"></span> }
                                                        }}
                                                    </div>
                                                    <span class=format!("text-lg font-medium transition-colors {}", if todo.completed { "text-slate-500 line-through" } else { "text-slate-200 group-hover:text-teal-50" })>
                                                        {todo.title}
                                                    </span>
                                                </div>
                                                <div class="text-teal-400 opacity-0 group-hover:opacity-100 transition-opacity font-semibold">
                                                    "Edit ->"
                                                </div>
                                            </div>
                                        </A>
                                    }
                                }
                            />
                        </div>
                    }
                        .into_view(),
                    Some(Err(error)) => view! {
                        <div class="glass-card p-8 text-center text-rose-400">
                            {format!("Failed to load todos: {}", error)}
                        </div>
                    }
                        .into_view(),
                    None => view! { <div></div> }.into_view(),
                }}
            </Suspense>
        </div>
    }
}
