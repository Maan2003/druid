use std::sync::atomic::{AtomicU64, Ordering};

use crate::{Event, EventId, Selector, WidgetId, WindowId};

static DEBUGGER_WINDOW_ID: AtomicU64 = AtomicU64::new(u64::MAX);

pub fn set_window_id(id: WindowId) {
    DEBUGGER_WINDOW_ID.store(id.0, Ordering::SeqCst);
}

pub fn window_id() -> WindowId {
    WindowId(DEBUGGER_WINDOW_ID.load(Ordering::SeqCst))
}


pub const INSPECT: Selector<()> = Selector::new("druid-debugger.inspect");
pub const INSPECT_RESPONSE: Selector<(WidgetId, String)> =
    Selector::new("druid-debugger.inspect-response");

pub const HIGHLIGHT: Selector<bool> = Selector::new("druid-debugger.highlight");
pub const EVENT: Selector<(EventId, WidgetId, Event)> = Selector::new("druid-debugger.event");
