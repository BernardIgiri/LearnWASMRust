#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    input: String,
    edit_input: String,
    todos: Vec<Todo>,
}

struct Todo {
    text: String,
    edit: bool,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    StartEditing(usize),
    FinishEditing(usize),
    UpdateEdit(String),
    RemoveAll,
    Nothing,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: vec![],
            input: "".to_string(),
            edit_input: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let t = Todo {
                    text: self.input.clone(),
                    edit: false,
                };
                self.todos.push(t);
                self.input = "".to_string();
            }
            Msg::Update(s) => {
                self.input = s;
            }
            Msg::Remove(i) => {
                self.todos.remove(i);
            }
            Msg::RemoveAll => {
                self.todos = vec![];
            }
            Msg::UpdateEdit(s) => {
                self.edit_input = s;
            }
            Msg::StartEditing(i) => {
                let todo = self.todos.get_mut(i).unwrap();
                if !todo.edit {
                    todo.edit = true;
                    self.edit_input = todo.text.clone();
                }
            }
            Msg::FinishEditing(i) => {
                let todo = self.todos.get_mut(i).unwrap();
                if todo.edit {
                    todo.edit = false;
                    todo.text = self.edit_input.clone();
                }
            }
            Msg::Nothing => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let view_todo_edit = |(i, todo): (usize, &Todo)| {
            if todo.edit {
                html! {
                    <label><input type="text"
                        value=&self.edit_input,
                        oninput=self.link.callback(|e: InputData| Msg::UpdateEdit(e.value)),
                        onkeypress=self.link.callback(move |e: KeyboardEvent| {
                            if e.key() == "Enter" {Msg::FinishEditing(i)} else {Msg::Nothing}
                        }),
                        />
                    </label>
                }
            } else {
                html! {
                    <label > {format!("{} ", &todo.text)}
                    </label>
                }
            }
        };
        let view_todo = |(i, todo): (usize, &Todo)| {
            html! {
                <li ondblclick=self.link.callback(move |_| Msg::StartEditing(i)),>
                    { view_todo_edit((i, &todo))}
                    <button title="Delete".to_string(), onclick=self.link.callback(move |_| Msg::Remove(i)),>{"X"}</button>
                </li>
            }
        };
        html! {
            <>
                <div>
                    <h1>{"My Todo App"}</h1>
                    <input
                        placeholder="what do you want to do?",
                        value=&self.input,
                        oninput=self.link.callback(|e: InputData| Msg::Update(e.value)),
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" {Msg::Add} else {Msg::Nothing}
                        }),/>
                </div>
                <div>
                    <button onclick=self.link.callback(|_| Msg::RemoveAll), >{"Delete all Todos!"}</button>
                </div>
                <div>
                    <ul>
                    {for self.todos.iter().enumerate().map(view_todo)}
                    </ul>
                </div>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
