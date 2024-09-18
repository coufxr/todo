use slint::{PlatformError, VecModel};
use std::rc::Rc;

mod utils;
use crate::utils::datetimex::today;

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let todo_model = Rc::new(VecModel::<TodoItem>::from(vec![]));

    let app = AppWindow::new()?;

    let today = today();
    app.set_dt_text(today.into());

    app.on_todo_add({
        let todo_model = todo_model.clone();
        move |text| {
            todo_model.push(TodoItem {
                checked: false,
                title: text,
            });
        }
    });

    app.set_todo_list(todo_model.clone().into());
    app.run()
}
