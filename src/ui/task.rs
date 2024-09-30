use crate::{Actions, AppWindow, OptionItem};
use slint::VecModel;
use std::rc::Rc;

pub fn create_list(ui: &AppWindow) {
    let list = vec![
        OptionItem {
            action: Actions::Rename,
            title: "Rename".into(),
        },
        OptionItem {
            action: Actions::Clone,
            title: "Clone".into(),
        },
        OptionItem {
            action: Actions::Delete,
            title: "Delete".into(),
        },
        OptionItem {
            action: Actions::Move,
            title: "Move".into(),
        },
    ];
    // 为什么一定要使用 Rc?
    let options = Rc::new(VecModel::<OptionItem>::from(list));
    ui.set_option_list(options.into());
}
