use std::fmt::Write;

use druid::{im, Data, KeyEvent, Lens, MouseEvent, WidgetId};
use im::Vector;

use druid::EventId;

#[derive(Data, Clone, Lens)]
pub struct DebuggerData {
    pub items: Vector<EventData>,
    pub screen: Screen,
}

#[derive(Data, Clone, Copy)]
pub enum Screen {
    EventSelection,
    EventDetails(usize),
}

#[derive(Data, Clone, Lens)]
pub struct EventData {
    pub idx: usize,
    pub event_id: EventId,
    pub items: Vector<Item>,
}

#[derive(Data, Clone, Lens)]
pub struct Item {
    #[data(same_fn = "PartialEq::eq")]
    pub widget_id: WidgetId,
    pub inner: ItemInner,
}

#[derive(Data, Clone)]
pub enum ItemInner {
    Event(Event),
}

#[derive(Clone, Debug)]
pub struct Event {
    pub inner: druid::Event,
}

impl Data for Event {
    fn same(&self, other: &Self) -> bool {
        let mouse_ev = |a: &MouseEvent, b: &MouseEvent| {
            a.pos == b.pos
                && a.window_pos == b.window_pos
                && a.button == b.button
                && a.buttons == b.buttons
                && a.count == b.count
                && a.focus == b.focus
                && a.mods == b.mods
                && a.wheel_delta == b.wheel_delta
        };
        let key_ev = |a: &KeyEvent, b: &KeyEvent| {
            a.code == b.code
                && a.is_composing == b.is_composing
                && a.key == b.key
                && a.location == b.location
                && a.mods == b.mods
                && a.repeat == b.repeat
                && a.state == b.state
        };
        match (&self.inner, &other.inner) {
            (druid::Event::WindowConnected, druid::Event::WindowConnected) => true,
            (druid::Event::WindowCloseRequested, druid::Event::WindowCloseRequested) => true,
            (druid::Event::WindowDisconnected, druid::Event::WindowDisconnected) => true,
            (druid::Event::WindowSize(a), druid::Event::WindowSize(b)) => a == b,
            (druid::Event::MouseDown(a), druid::Event::MouseDown(b)) => mouse_ev(a, b),
            (druid::Event::MouseUp(a), druid::Event::MouseUp(b)) => mouse_ev(a, b),
            (druid::Event::MouseMove(a), druid::Event::MouseMove(b)) => mouse_ev(a, b),
            (druid::Event::Wheel(a), druid::Event::Wheel(b)) => mouse_ev(a, b),
            (druid::Event::KeyDown(a), druid::Event::KeyDown(b)) => key_ev(a, b),
            (druid::Event::KeyUp(a), druid::Event::KeyUp(b)) => key_ev(a, b),
            (druid::Event::Paste(_), druid::Event::Paste(_)) => true,
            (druid::Event::Zoom(_), druid::Event::Zoom(_)) => true,
            (druid::Event::Timer(_), druid::Event::Timer(_)) => true,
            (druid::Event::AnimFrame(_), druid::Event::AnimFrame(_)) => true,
            (druid::Event::Command(_), druid::Event::Command(_)) => true,
            (druid::Event::Notification(_), druid::Event::Notification(_)) => true,
            (druid::Event::Internal(_), druid::Event::Internal(_)) => true,
            _ => false,
        }
    }
}
impl Event {
    pub fn render(&self, w: &mut String) {
        match &self.inner {
            // druid::Event::WindowSize(_) => {}
            druid::Event::MouseDown(m) => {
                write!(w, "MouseDown ").unwrap();
                show_mouse_event(m, w);
            }
            druid::Event::MouseUp(m) => {
                write!(w, "MouseUp ").unwrap();
                show_mouse_event(m, w);
            }
            druid::Event::MouseMove(m) => {
                write!(w, "MouseMove ").unwrap();
                show_mouse_event(m, w);
            }
            druid::Event::Wheel(m) => {
                write!(w, "Wheel ").unwrap();
                show_mouse_event(m, w);
            }
            // druid::Event::KeyDown(_) => {}
            // druid::Event::KeyUp(_) => {}
            druid::Event::Paste(_) => {
                write!(w, "Paste").unwrap();
            }
            druid::Event::Zoom(z) => {
                write!(w, "Zoom({:.3})", *z).unwrap();
            }
            druid::Event::Timer(_) => {
                write!(w, "Timer").unwrap();
            }
            druid::Event::AnimFrame(_) => {
                write!(w, "{:?}", self).unwrap();
            }
            druid::Event::Command(c) => {
                write!(w, "{:#?}", c).unwrap();
            }
            druid::Event::Notification(n) => {
                write!(w, "{:#?}", n).unwrap();
            }
            druid::Event::Internal(_) => {
                write!(w, "Internal").unwrap();
            }
            druid::Event::WindowConnected => {
                write!(w, "Window Connected").unwrap();
            }
            druid::Event::WindowCloseRequested => {
                write!(w, "Window Connected Requested").unwrap();
            }
            druid::Event::WindowDisconnected => {
                write!(w, "Window Disconnected").unwrap();
            }
            druid::Event::WindowSize(s) => {
                write!(w, "Window Size {:?}", s).unwrap();
            }
            // druid::Event::KeyDown(_) => {}
            // druid::Event::KeyUp(_) => {}
            _ => {
                write!(w, "{:#?}", self).unwrap();
            }
        }
    }
}

fn show_mouse_event(ev: &MouseEvent, w: &mut String) {
    writeln!(w, "{{").unwrap();
    writeln!(w, "\tpos: ({:.3}, {:.3}),", ev.pos.x, ev.pos.y).unwrap();
    writeln!(w, "\tbuttons: {:?},", ev.buttons).unwrap();
    writeln!(w, "\tmods: {:?},", ev.mods).unwrap();
    writeln!(w, "\tfocus: {:?},", ev.focus).unwrap();
    writeln!(w, "\tbutton: {:?},", ev.button).unwrap();
    writeln!(w, "\tcount: {:?},", ev.count).unwrap();
    writeln!(w, "\twheel_delta: {:?},", ev.wheel_delta).unwrap();
    write!(w, "}}").unwrap();
}
