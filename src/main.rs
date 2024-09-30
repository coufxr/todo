use slint::{PlatformError, VecModel};
use std::rc::Rc;

mod controllers;
mod models;
mod ui;
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

    ui::task::create_list(&app);

    app.on_option_action(move |action| match action {
        Actions::Rename => {
            println!("修改")
        }
        Actions::Clone => {
            println!("克隆")
        }
        Actions::Delete => {
            println!("删除")
        }
        Actions::Move => {
            println!("移动")
        }
    });

    app.run()
}
