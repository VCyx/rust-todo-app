use leptos::*;
use crate::api::{self, TodoModel};

#[component]
pub fn Home() -> impl IntoView {
    let (todos_sig, set_todos) = create_signal(Vec::<TodoModel>::new());
    let (input_value, set_input_value) = create_signal(String::new());

    let load_todos = Action::new(move |_: &()| {
        async move {
            match api::list_todos().await {
                Ok(data) => set_todos.set(data),
                Err(e) => logging::error!("Failed to fetch todos: {}", e),
            }
        }
    });

    create_effect(move |_| {
        load_todos.dispatch(());
    });

    let add_todo = Action::new(move |title: &String| {
        let title = title.clone();
        async move {
            if !title.is_empty() {
                match api::create_todo(title).await {
                    Ok(_) => {
                        set_input_value.set(String::new());
                        load_todos.dispatch(());
                    }
                    Err(e) => logging::error!("Failed to create todo: {}", e),
                }
            }
        }
    });

    view! {
        <div class="max-w-2xl mx-auto py-12 px-6">
            <h1 class="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-teal-400 to-emerald-500 mb-8 text-center drop-shadow-sm pb-1">
                "My Todos"
            </h1>
            
            <div class="glass-card mb-8 p-6 transition-all duration-300">
                <form on:submit=move |e| {
                    e.prevent_default();
                    add_todo.dispatch(input_value.get());
                } class="flex gap-4">
                    <input 
                        type="text" 
                        class="input-field flex-grow"
                        placeholder="What needs to be done?"
                        prop:value=move || input_value.get()
                        on:input=move |e| set_input_value.set(event_target_value(&e))
                    />
                    <button type="submit" class="btn-primary whitespace-nowrap">
                        "Add Todo"
                    </button>
                </form>
            </div>

            <div class="space-y-4">
                <For
                    each=move || todos_sig.get()
                    key=|todo| todo.id
                    children=move |todo| {
                        let id = todo.id;
                        view! {
                            <a href=format!("/todos/{}", id) class="block group">
                                <div class="glass-card p-4 transition-all duration-300 hover:scale-[1.02] hover:shadow-teal-500/10 hover:border-teal-500/40 flex items-center justify-between cursor-pointer">
                                    <div class="flex items-center gap-4">
                                        <div class=if todo.completed { "w-6 h-6 rounded-full border-2 border-emerald-500 bg-emerald-500 flex items-center justify-center transition-colors" } else { "w-6 h-6 rounded-full border-2 border-slate-600 group-hover:border-teal-400 transition-colors bg-transparent" }>
                                            {if todo.completed {
                                                view! { <span class="text-white text-xs">"✓"</span> }
                                            } else {
                                                view! { <span class="hidden"></span> }
                                            }}
                                        </div>
                                        <span class=format!("text-lg font-medium transition-colors {}", if todo.completed { "text-slate-500 line-through" } else { "text-slate-200 group-hover:text-teal-50" })>
                                            {todo.title.clone()}
                                        </span>
                                    </div>
                                    <div class="text-teal-400 opacity-0 group-hover:opacity-100 transition-opacity font-semibold">
                                        "Edit →"
                                    </div>
                                </div>
                            </a>
                        }
                    }
                />
            </div>
        </div>
    }
}
