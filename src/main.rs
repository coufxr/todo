use slint::Model;
use std::rc::Rc;

slint::include_modules!();

pub struct State {
    pub main_window: AppWindow,
    pub todo_model: Rc<slint::VecModel<TodoItem>>,
}

thread_local! {
    static STATE : core::cell::RefCell<Option<State>> = Default::default();
}

pub fn main() {
    let state = init();
    let main_window = state.main_window.clone_strong();
    STATE.with(|ui| *ui.borrow_mut() = Some(state));
    main_window.run().unwrap();
}

fn init() -> State {
    let todo_model = Rc::new(slint::VecModel::<TodoItem>::from(vec![]));

    let main_window = AppWindow::new().unwrap();

    main_window.on_todo_added({
        let todo_model = todo_model.clone();
        move |text| {
            todo_model.push(TodoItem {
                checked: false,
                title: text,
            })
        }
    });
    main_window.on_remove_done({
        let todo_model = todo_model.clone();
        move || {
            let mut offset = 0;
            for i in 0..todo_model.row_count() {
                if todo_model.row_data(i - offset).unwrap().checked {
                    todo_model.remove(i - offset);
                    offset += 1;
                }
            }
        }
    });

    let weak_window = main_window.as_weak();
    main_window.on_popup_confirmed(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });

    {
        let weak_window = main_window.as_weak();
        let todo_model = todo_model.clone();
        main_window.window().on_close_requested(move || {
            let window = weak_window.unwrap();

            if todo_model.iter().any(|t| !t.checked) {
                window.invoke_show_confirm_popup();
                slint::CloseRequestResponse::KeepWindowShown
            } else {
                slint::CloseRequestResponse::HideWindow
            }
        });
    }

    main_window.set_show_header(true);
    main_window.set_todo_model(todo_model.clone().into());
    State {
        main_window,
        todo_model,
    }
}
