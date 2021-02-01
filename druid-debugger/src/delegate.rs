use druid::{im::Vector, EventId, Selector, UpdateCtx, WidgetId};

use druid::{dbg, Command, Event, EventCtx};

use crate::data::DebuggerData;
use crate::data::{EventData, Item, ItemInner, Screen};
use crate::{data, SELECT_EVENT};

use super::data::Filter;
use druid::Data;

#[derive(Default, Debug)]
pub struct Delegate {
    filter: Option<Filter>,
}

const REFILTER: Selector<()> = Selector::new("druid-debugger.refilter");

impl Delegate {
    pub fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut DebuggerData) {}

    pub fn update(&mut self, ctx: &mut UpdateCtx, data: &DebuggerData) {
        if self
            .filter
            .as_ref()
            .map_or(false, |x| !x.same(&data.filter))
        {
            ctx.submit_command(REFILTER);
        }
        self.filter = Some(data.filter.clone());
    }

    fn filter_item(&mut self, filter: &data::Filter, item: &Item) -> bool {
        match &item.inner {
            ItemInner::Event(e) => match e.inner {
                Event::WindowConnected => filter.window,
                Event::WindowCloseRequested => filter.window,
                Event::WindowDisconnected => filter.window,
                Event::WindowSize(_) => filter.window,
                Event::MouseDown(_) => filter.mouse,
                Event::MouseUp(_) => filter.mouse,
                Event::MouseMove(_) => filter.mouse_move,
                Event::Wheel(_) => filter.mouse,
                Event::KeyDown(_) => filter.key,
                Event::KeyUp(_) => filter.key,
                Event::Paste(_) => filter.key,
                Event::Zoom(_) => filter.key,
                Event::Timer(_) => filter.timer,
                Event::AnimFrame(_) => filter.anim,
                Event::Command(_) => true,
                Event::Notification(_) => true,
                Event::Internal(_) => filter.internal,
            },
            ItemInner::Command(_) => true,
        }
    }

    fn re_filter(&mut self, data: &mut DebuggerData) {
        // match &data.screen {
        //     Screen::EventSelection => {
        //         let items = data.all_items.clone();
        //         data.all_items.clear();

        //         let filter = &data.filter;
        //         let filtered = items.iter().filter(|e| {
        //             e.all_items
        //                 .iter()
        //                 .any(|item| self.filter_item(filter, item))
        //         });

        //         data.all_items.extend(filtered.cloned());
        //     }
        //     Screen::EventDetails(idx) => {
        //         let item = &mut data.all_items[*idx];
        //         let filter = &data.filter;

        //         let filtered = item
        //             .all_items
        //             .iter()
        //             .filter(|e| self.filter_item(filter, e));

        //         item.items.extend(filtered.cloned());
        //     }
        // }
    }

    fn add_item(
        &mut self,
        data: &mut DebuggerData,
        event_id: EventId,
        widget_id: WidgetId,
        item: ItemInner,
    ) {
        let item = Item {
            widget_id,
            inner: item,
        };
        let show = self.filter_item(&data.filter, &item);
        if !show {
            return;
        }

        if data
            .all_items
            .back()
            .map_or(true, |ev| ev.event_id != event_id)
        {
            let event_data = EventData {
                idx: data.all_items.len(),
                event_id,
                all_items: Vector::new(),
            };
            data.all_items.push_back(event_data);
        }

        let event_data = data.all_items.back_mut().unwrap();
        event_data.all_items.push_back(item);
    }

    pub fn command(&mut self, _ctx: &mut EventCtx, cmd: &Command, data: &mut DebuggerData) {
        if let Some((event_id, widget_id, ev)) = cmd.get(dbg::EVENT).cloned() {
            self.add_item(
                data,
                event_id,
                widget_id,
                ItemInner::Event(data::Event { inner: ev }),
            );
        }

        if let Some((event_id, widget_id, cmd)) = cmd.get(dbg::COMMAND).cloned() {
            self.add_item(
                data,
                event_id,
                widget_id,
                ItemInner::Command(data::Command { inner: cmd }),
            );
        }

        if let Some(idx) = cmd.get(SELECT_EVENT).copied() {
            data.screen = Screen::EventDetails(idx);
        }
        if cmd.is(super::BACK_HOME) {
            data.screen = Screen::EventSelection;
        }

        if cmd.is(REFILTER) {
            self.re_filter(data);
        }
    }
}
