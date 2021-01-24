use druid::dbg;
use druid::{DelegateCtx, Selector, WindowDesc};

mod data;
mod delegate;
mod expandable_text;
mod ui;
mod widget;
const SELECT_EVENT: Selector<usize> = Selector::new("druid-debugger.internal.select-event");

pub fn launch<T: druid::Data>(ctx: &mut DelegateCtx) {
    let window = WindowDesc::new(ui::ui_builder::<T>);
    dbg::set_window_id(window.id);
    ctx.new_window(window);
}
