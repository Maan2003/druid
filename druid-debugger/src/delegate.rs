use druid::im::Vector;

use druid::{Command, dbg, Event, EventCtx};

use crate::data::{EventData, Item, ItemInner, Screen};
use crate::{data, SELECT_EVENT};
use crate::{data::DebuggerData};

pub struct Delegate;

impl Delegate {
    pub fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut DebuggerData) {}

    pub fn command(&mut self, _ctx: &mut EventCtx, cmd: &Command, data: &mut DebuggerData) {
        if let Some((event_id, widget_id, ev)) = cmd.get(dbg::EVENT).cloned() {
            if data.items.back().map_or(true, |ev| ev.event_id != event_id) {
                data.items.push_back(EventData {
                    idx: data.items.len(),
                    event_id,
                    items: Vector::new(),
                })
            }
            let event_data = data.items.back_mut().unwrap();
            event_data.items.push_back(Item {
                widget_id,
                inner: ItemInner::Event(data::Event { inner: ev }),
            })
        }

        if let Some(idx) = cmd.get(SELECT_EVENT).copied() {
            data.screen = Screen::EventDetails(idx);
        }
        // if let Some((widget_id, name)) = cmd.get(INSPECT_RESPONSE).cloned() {
        //     data.item = Some(DebugItem {
        //         name,
        //         events: Vector::new(),
        //         widget_id,
        //     });
        // }

        // if let (Some((_widget_id, event)), Some(item)) =
        //     (cmd.get(EVENT).cloned(), &mut self.data.item)
        // {
        //     item.events.push_back(data::Event {
        //         inner: event,
        //         selected: false,
        //     });
        //     ctx.children_changed();
        //     // return;
        // }
    }
}
