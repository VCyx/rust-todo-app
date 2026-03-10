use leptos::*;
use leptos_router::*;
use crate::api::{self, TodoModel};

#[component]
pub fn TodoDetail() -> impl IntoView {
    let params = use_params_map();
    let id_str = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let id = move || id_str().parse::<i32>().unwrap_or(0);

    let (todo_sig, set_todo) = create_signal(None::<TodoModel>);
    let (is_loading, set_is_loading) = create_signal(true);
    let (edit_title, set_edit_title) = create_signal(String::new());

    let load_todo = Action::new(move |id: &i32| {
        let id_val = *id;
        async move {
            set_is_loading.set(true);
            match api::get_todo(id_val).await {
                Ok(data) => {
                    set_edit_title.set(data.title.clone());
                    set_todo.set(Some(data));
                }
                Err(e) => logging::error!("Failed to fetch todo: {}", e),
            }
            set_is_loading.set(false);
        }
    });

    create_effect(move |_| {
        let current_id = id();
        if current_id != 0 {
            load_todo.dispatch(current_id);
        }
    });

    let update_action = Action::new(move |(id, title, completed): &(i32, String, bool)| {
        let id_val = *id;
        let title_val = title.clone();
        let completed_val = *completed;
        async move {
            match api::update_todo(id_val, title_val, completed_val).await {
                Ok(data) => set_todo.set(Some(data)),
                Err(e) => logging::error!("Failed to update todo: {}", e),
            }
        }
    });

    let delete_action = Action::new(move |id: &i32| {
        let id_val = *id;
        let navigate = use_navigate();
        async move {
            match api::delete_todo(id_val).await {
                Ok(_) => {
                    navigate("/", Default::default());
                }
                Err(e) => logging::error!("Failed to delete todo: {}", e),
            }
        }
    });

    view! {
        <div class="max-w-2xl mx-auto py-12 px-6">
            <div class="mb-6">
                <a href="/" class="text-teal-400 hover:text-teal-300 transition-colors font-medium flex items-center gap-2 w-max bg-slate-800/50 px-4 py-2 rounded-full border border-teal-500/20 hover:border-teal-500/50">
                    "← Back to List"
                </a>
            </div>

            <Show
                when=move || !is_loading.get()
                fallback=|| view! { <div class="text-center text-slate-400 py-12 animate-pulse">"Loading Todo..."</div> }
            >
                {move || match todo_sig.get() {
                    Some(todo) => {
                        let current_id = todo.id;
                        let completed = todo.completed;
                        view! {
                            <div class="glass-card p-8 animate-[fadeIn_0.5s_ease-out]">
                                <h1 class="text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-white to-slate-400 mb-6 drop-shadow-sm">"Edit Todo"</h1>
                                
                                <div class="space-y-6">
                                    <div class="flex flex-col gap-2">
                                        <label class="text-sm text-slate-400 font-medium ml-1">"Title"</label>
                                        <input 
                                            type="text" 
                                            class="input-field text-lg"
                                            prop:value=move || edit_title.get()
                                            on:input=move |e| set_edit_title.set(event_target_value(&e))
                                        />
                                    </div>

                                    <div class="flex items-center gap-3">
                                        <button 
                                            class=format!("w-6 h-6 rounded border-2 flex items-center justify-center transition-all duration-200 {}", if completed { "bg-emerald-500 border-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.5)]" } else { "border-slate-500 hover:border-teal-400" })
                                            on:click=move |_| {
                                                update_action.dispatch((current_id, edit_title.get(), !completed));
                                            }
                                        >
                                            {if completed {
                                                view! { <span class="text-white text-xs">"✓"</span> }
                                            } else {
                                                view! { <span class="hidden"></span> }
                                            }}
                                        </button>
                                        <span class="text-slate-300 font-medium">"Mark as completed"</span>
                                    </div>

                                    <div class="pt-6 border-t border-slate-700/50 flex gap-4 mt-8">
                                        <button 
                                            class="btn-primary flex-grow text-lg shadow-teal-500/20"
                                            on:click=move |_| {
                                                update_action.dispatch((current_id, edit_title.get(), completed));
                                            }
                                        >
                                            "Save Changes"
                                        </button>
                                        <button 
                                            class="btn-danger flex-grow text-lg shadow-rose-500/20 cursor-pointer"
                                            on:click=move |_| {
                                                delete_action.dispatch(current_id);
                                            }
                                        >
                                            "Delete Todo"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }
                    None => view! { <div class="text-center text-rose-400 py-12 glass-card">"Todo not found."</div> }.into_view()
                }}
            </Show>
        </div>
    }
}
