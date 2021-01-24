use druid::lens::Index;
use druid::widget::{Controller, ViewSwitcher};
use druid::{dbg, Env, LensExt, LifeCycle, LifeCycleCtx};
use druid::{
    im::Vector,
    widget::{CrossAxisAlignment, Flex, Label, List, Scroll},
    Widget, WidgetExt, WidgetPod,
};

use crate::data::{EventData, Item, ItemInner, Screen};
use crate::SELECT_EVENT;
use crate::{data::DebuggerData, delegate::Delegate, widget::AppWrapper};

pub fn ui_builder<T: druid::Data>() -> impl Widget<T> {
    AppWrapper {
        inner: WidgetPod::new(ui().boxed()),
        data: DebuggerData {
            items: Vector::new(),
            screen: Screen::EventSelection,
        },
        delegate: Delegate,
    }
}

fn ui() -> impl Widget<DebuggerData> {
    ViewSwitcher::new(
        |data: &DebuggerData, _| data.screen,
        |_, data: &DebuggerData, _| match data.screen {
            Screen::EventSelection => event_selection().boxed(),
            Screen::EventDetails(a) => event_details()
                .lens(DebuggerData::items.then(Index::new(a)))
                .boxed(),
        },
    )
    .padding(10.)
}

fn event_selection() -> impl Widget<DebuggerData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Select an Event to inspect"))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(List::new(|| {
                Label::dynamic(|event: &EventData, _| {
                    format!("Events #{}", event.event_id.as_raw())
                })
                .on_click(|ctx, event: &mut EventData, _| {
                    ctx.submit_command(SELECT_EVENT.with(event.idx));
                })
            }))
            .vertical()
            .expand_width()
            .lens(DebuggerData::items),
            1.0,
        )
}

fn event_details() -> impl Widget<EventData> {
    List::new(show_event).lens(EventData::items)
}

fn show_event() -> impl Widget<Item> {
    Label::dynamic(|item: &Item, _| match &item.inner {
        ItemInner::Event(e) => {
            let mut s = String::new();
            e.render(&mut s);
            format!(
                "{:?} recieved Event {}",
                item.widget_id,
                s.split("{").next().unwrap()
            )
        }
    })
    .controller(OnHover(
        |ctx: &mut LifeCycleCtx, data: &Item, hovered, _env: &Env| {
            dbg!();
            ctx.submit_command(dbg::HIGHLIGHT.with(hovered).to(data.widget_id));
        },
    ))
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
// fn selector_page() -> impl Widget<()> {
//     Button::new("Inspect")
//         .on_click(|ctx, _, _| ctx.submit_command(INSPECT.to(Target::Global)))
//         .center()
// }

// fn widget_page() -> impl Widget<DebugItem> {
//     let heading = Label::new("Events").with_text_size(18.);

//     Flex::column()
//         .must_fill_main_axis(true)
//         .cross_axis_alignment(CrossAxisAlignment::Start)
//         .with_child(
//             Label::dynamic(|data: &DebugItem, _| data.name.clone())
//                 .with_text_size(24.0)
//                 .center(),
//         )
//         .with_spacer(20.)
//         .with_child(heading)
//         .with_default_spacer()
//         .with_flex_child(
//             Scroll::new(List::new(event).with_spacing(5.))
//                 .vertical()
//                 .expand()
//                 .lens(DebugItem::events),
//             1.0,
//         )
//         .padding(15.)
// }

// fn event() -> impl Widget<Event> {
//     EventWidget::new().expand_width()
// }

// fn get_color(ev: &Event) -> Color {
//     match ev.inner {
//         druid::Event::WindowConnected
//         | druid::Event::WindowCloseRequested
//         | druid::Event::WindowDisconnected
//         | druid::Event::WindowSize(_) => Color::PURPLE,

//         druid::Event::MouseDown(_)
//         | druid::Event::MouseUp(_)
//         | druid::Event::MouseMove(_)
//         | druid::Event::Wheel(_) => Color::YELLOW,
//         druid::Event::KeyDown(_) | druid::Event::KeyUp(_) | druid::Event::Paste(_) => Color::AQUA,
//         _ => Color::FUCHSIA,
//     }
// }
