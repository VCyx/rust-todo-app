use crate::api::{self, TodoModel};
use leptos::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
use axum::http::StatusCode;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[cfg(feature = "ssr")]
fn set_response_status(status: StatusCode) {
    if let Some(response) = use_context::<ResponseOptions>() {
        response.set_status(status);
    }
}

#[component]
pub fn TodoDetail() -> impl IntoView {
    let params = use_params_map();
    let todo_id = move || params.with(|p| p.get("id").and_then(|id| id.parse::<i32>().ok()));

    #[cfg(feature = "ssr")]
    if todo_id().is_none() {
        set_response_status(StatusCode::NOT_FOUND);
    }

    let todo_resource = create_blocking_resource(todo_id, |id| async move {
        match id {
            Some(id) => api::get_todo(id).await,
            None => Ok(None),
        }
    });

    let (todo_override, set_todo_override) = create_signal(None::<TodoModel>);
    let (edit_title, set_edit_title) = create_signal(None::<String>);

    create_effect(move |_| {
        let _ = todo_id();
        set_todo_override.set(None);
        set_edit_title.set(None);
    });

    let update_action = Action::new(move |(id, title, completed): &(i32, String, bool)| {
        let id = *id;
        let title = title.clone();
        let completed = *completed;

        async move { api::update_todo(id, title, completed).await }
    });

    create_effect(move |_| {
        if let Some(result) = update_action.value().get() {
            match result {
                Ok(todo) => {
                    set_edit_title.set(None);
                    set_todo_override.set(Some(todo));
                }
                Err(error) => logging::error!("Failed to update todo: {}", error),
            }
        }
    });

    let navigate = use_navigate();
    let delete_action = Action::new(move |id: &i32| {
        let id = *id;
        let navigate = navigate.clone();

        async move {
            api::delete_todo(id).await?;
            navigate("/", Default::default());
            Ok::<(), server_fn::ServerFnError>(())
        }
    });

    create_effect(move |_| {
        if let Some(Err(error)) = delete_action.value().get() {
            logging::error!("Failed to delete todo: {}", error);
        }
    });

    let current_todo = move || {
        todo_override.get().or_else(|| match todo_resource.get() {
            Some(Ok(todo)) => todo,
            _ => None,
        })
    };

    let displayed_title = move || {
        edit_title.get().unwrap_or_else(|| {
            current_todo()
                .map(|todo| todo.title)
                .unwrap_or_default()
        })
    };

    view! {
        <div class="max-w-2xl mx-auto py-12 px-6">
            <div class="mb-6">
                <A href="/" attr:class="text-teal-400 hover:text-teal-300 transition-colors font-medium flex items-center gap-2 w-max bg-slate-800/50 px-4 py-2 rounded-full border border-teal-500/20 hover:border-teal-500/50">
                    "<- Back to List"
                </A>
            </div>

            <Suspense fallback=|| view! { <div class="text-center text-slate-400 py-12 animate-pulse">"Loading Todo..."</div> }>
                {move || match todo_resource.get() {
                    Some(Ok(Some(_))) => match current_todo() {
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
                                                prop:value=displayed_title
                                                on:input=move |ev| set_edit_title.set(Some(event_target_value(&ev)))
                                            />
                                        </div>

                                        <div class="flex items-center gap-3">
                                            <button
                                                class=format!("w-6 h-6 rounded border-2 flex items-center justify-center transition-all duration-200 {}", if completed { "bg-emerald-500 border-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.5)]" } else { "border-slate-500 hover:border-teal-400" })
                                                on:click=move |_| {
                                                    update_action.dispatch((current_id, displayed_title(), !completed));
                                                }
                                            >
                                                {if completed {
                                                    view! { <span class="text-white text-xs">"x"</span> }
                                                } else {
                                                    view! { <span class="hidden"></span> }
                                                }}
                                            </button>
                                            <span class="text-slate-300 font-medium">"Mark as completed"</span>
                                        </div>

                                        <div class="pt-6 border-t border-slate-700/50 flex gap-4 mt-8">
                                            <button
                                                class="btn-primary flex-grow text-lg shadow-teal-500/20"
                                                disabled=move || update_action.pending().get()
                                                on:click=move |_| {
                                                    update_action.dispatch((current_id, displayed_title(), completed));
                                                }
                                            >
                                                "Save Changes"
                                            </button>
                                            <button
                                                class="btn-danger flex-grow text-lg shadow-rose-500/20 cursor-pointer"
                                                disabled=move || delete_action.pending().get()
                                                on:click=move |_| {
                                                    delete_action.dispatch(current_id);
                                                }
                                            >
                                                "Delete Todo"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                                .into_view()
                        }
                        None => view! { <div class="text-center text-slate-400 py-12 animate-pulse">"Loading Todo..."</div> }
                            .into_view(),
                    },
                    Some(Ok(None)) => view! { <div class="text-center text-rose-400 py-12 glass-card">"Todo not found."</div> }
                        .into_view(),
                    Some(Err(error)) => view! {
                        <div class="text-center text-rose-400 py-12 glass-card">
                            {format!("Failed to load todo: {}", error)}
                        </div>
                    }
                        .into_view(),
                    None => view! { <div></div> }.into_view(),
                }}
            </Suspense>
        </div>
    }
}
