mod runtime;

use application::{
    command::Command,
    projection::{TaskStatus, TodoList},
};
use framework::*;
use runtime::Runtime;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let input_ref = use_node_ref();
    let runtime = use_memo((), |_| Runtime::default());
    let todo_list = use_state(TodoList::default);

    let fetch = {
        let runtime = runtime.clone();
        let todo_list = todo_list.clone();
        Callback::from(move |_: ()| {
            let runtime = runtime.clone();
            let todo_list = todo_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let query = application::query::GetTodoListQuery {};
                let data = query.execute(runtime.as_ref()).await.unwrap();
                todo_list.set(data);
            })
        })
    };

    {
        let fetch = fetch.clone();
        use_effect_with((), move |_| {
            fetch.emit(());
        });
    }

    let add_task = {
        let fetch = fetch.clone();
        let input_ref = input_ref.clone();
        let runtime = runtime.clone();

        Callback::from(move |_| {
            let fetch = fetch.clone();
            let input_ref = input_ref.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                let name = input.value();
                let res = Command::AddTask { name }.execute(runtime.as_ref()).await;

                if let Err(err) = res {
                    gloo_console::error!(&err.to_string());
                } else {
                    fetch.emit(());
                    input.set_value("");
                }
            });
        })
    };

    let complete_task = {
        let fetch = fetch.clone();
        let runtime = runtime.clone();

        Callback::from(move |index| {
            let fetch = fetch.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let res = Command::CompleteTask { index }
                    .execute(runtime.as_ref())
                    .await;

                if let Err(err) = res {
                    gloo_console::error!(&err.to_string());
                } else {
                    fetch.emit(());
                }
            });
        })
    };

    let remove_task = {
        let fetch = fetch.clone();
        let runtime = runtime.clone();

        Callback::from(move |index| {
            let fetch = fetch.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let res = Command::RemoveTask { index }
                    .execute(runtime.as_ref())
                    .await;

                if let Err(err) = res {
                    gloo_console::error!(&err.to_string());
                } else {
                    fetch.emit(());
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Todo List" }</h1>
            <h2>{ "In Progress" }</h2>
            <ul>
                {todo_list.clone().tasks.iter().filter(|task| task.status == TaskStatus::Created).map(|task| {
                    let task_index = task.index;
                    let task_name = task.name.clone();
                    let complete_task = complete_task.clone().reform(move |_| task_index);
                    let remove_task = remove_task.clone().reform(move |_| task_index);

                    html! {
                        <li key={task_index}>
                            <input type="checkbox" onclick={complete_task} />
                            { &task_name }
                            <a onclick={remove_task}>{ "❌" }</a>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
            <h2>{ "Completed" }</h2>
            {todo_list.clone().tasks.iter().filter(|task| task.status == TaskStatus::Completed).map(|task| html! {
                <li><input type="checkbox" checked={true} disabled={true} /> { &task.name }</li>
            }).collect::<Html>()}
            <input ref={input_ref} type="text" id="task" name="task" />
            <button onclick={add_task}>{ "Add Task" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
