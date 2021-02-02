use crate::data::{EventData, Filter, Item, ItemInner, Screen};
use crate::SELECT_EVENT;
use crate::{data::DebuggerData, delegate::Delegate, widget::AppWrapper};
use druid::widget::{Button, Checkbox, Controller, ViewSwitcher};
use druid::{
    im::Vector,
    widget::{CrossAxisAlignment, Flex, Label, Scroll},
    Widget, WidgetExt, WidgetPod,
};
use druid::{lens::Index, widget::TextBox};
use druid::{Env, LensExt, LifeCycle, LifeCycleCtx};
use druid_simple_table::Table;

pub fn ui_builder<T: druid::Data>() -> impl Widget<T> {
    AppWrapper {
        inner: WidgetPod::new(ui().boxed()),
        data: DebuggerData {
            screen: Screen::EventSelection,
            all_items: Vector::new(),
            filter: Default::default(),
        },
        delegate: Delegate::default(),
    }
}

fn ui() -> impl Widget<DebuggerData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::column()
                .with_child(
                    TextBox::new()
                        .with_placeholder("Widget Ids ignored")
                        .fix_width(300.)
                        .lens(Filter::widget_ids),
                )
                .with_default_spacer()
                .with_child(
                    Flex::row()
                        .with_child(Checkbox::new("Timer").lens(Filter::timer))
                        .with_child(Checkbox::new("Mouse").lens(Filter::mouse))
                        .with_child(Checkbox::new("Keyboard").lens(Filter::key))
                        .with_child(Checkbox::new("Window").lens(Filter::window))
                        .with_child(Checkbox::new("Animation").lens(Filter::anim))
                        .with_child(Checkbox::new("Internal").lens(Filter::internal))
                        .with_child(Checkbox::new("Requests").lens(Filter::requests)).center(),
                )
                .with_default_spacer()
                .lens(DebuggerData::filter),
        )
        .with_default_spacer()
        .with_flex_child(
            ViewSwitcher::new(
                |data: &DebuggerData, _| data.screen,
                |_, data: &DebuggerData, _| match data.screen {
                    Screen::EventSelection => event_selection().boxed(),
                    Screen::EventDetails(a) => event_details()
                        .lens(DebuggerData::all_items.then(Index::new(a)))
                        .boxed(),
                },
            ),
            1.0,
        )
        .padding(10.)
}

fn event_selection() -> impl Widget<DebuggerData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Select an Event to inspect"))
        .with_default_spacer()
        .with_flex_child(Scroll::new(event_slc()), 1.0)
}

fn event_slc() -> impl Widget<DebuggerData> {
    Table::new()
        .col(Label::new("Event Id").padding(5.), || {
            Label::dynamic(|ev: &EventData, _env| format!("{}", ev.event_id.as_raw())).on_click(
                |ctx, ev, _| {
                    ctx.submit_command(SELECT_EVENT.with(ev.idx));
                },
            )
        })
        .col(Label::new("Kind").padding(5.), || {
            Label::dynamic(|ev: &Item, _env| {
                let mut s = String::new();
                match &ev.inner {
                    ItemInner::Event(ev) => ev.render(&mut s),
                    ItemInner::Command(_) => s = "Command Sent".into(),
                }
                s
            })
            .lens(EventData::all_items.then(Index::new(0)))
        })
        .border(druid::theme::BORDER_LIGHT, 1.0)
        .lens(DebuggerData::all_items)
}

fn event_details() -> impl Widget<EventData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Button::new("Back").on_click(|ctx, _, _| {
            ctx.submit_command(super::BACK_HOME);
        }))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                Table::new()
                    .col(Label::new("Widget Id").padding(5.), || {
                        Label::dynamic(|ev: &Item, _| format!("{:?}", ev.widget_id)).controller(
                            OnHover(|ctx: &mut LifeCycleCtx, it: &Item, hovered, _env: &Env| {
                                ctx.submit_command(
                                    druid::dbg::HIGHLIGHT.with(hovered).to(it.widget_id),
                                );
                            }),
                        )
                    })
                    .col(Label::new("Details").padding(5.), || {
                        Label::dynamic(|data: &Item, _| match &data.inner {
                            ItemInner::Event(ev) => format!("{:#?}", ev.inner),
                            ItemInner::Command(cmd) => format!("{:#?}", cmd.inner),
                        })
                    })
                    .border(druid::theme::BORDER_LIGHT, 1.0)
                    .lens(EventData::all_items),
            ),
            1.0,
        )
}

struct OnHover<F>(F);

impl<T, W, F> Controller<T, W> for OnHover<F>
where
    W: Widget<T>,
    F: FnMut(&mut LifeCycleCtx, &T, bool, &Env),
{
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        match event {
            LifeCycle::HotChanged(hovered) => {
                (self.0)(ctx, data, *hovered, env);
            }
            _ => {}
        }
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &T,
        data: &T,
        env: &druid::Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}
