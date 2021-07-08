use std::path::PathBuf;

use crate::backend::dnd as backend;
use crate::{FormatId, Modifiers};
use kurbo::Point;

#[derive(Debug, Clone)]
pub struct DropEvent {
    pub modifiers: Modifiers,
    pub position: Point,
}

#[derive(Clone)]
pub struct DropContext(pub(crate) backend::DropContext);

#[derive(Debug)]
pub enum DropAction {
    Copy,
    Move,
}

impl DropContext {
    pub fn deny(&self) {
        self.0.deny()
    }

    pub fn action(&self) -> DropAction {
        self.0.action()
    }

    pub fn set_action(&self, action: DropAction) {
        self.0.set_action(action)
    }

    pub fn get_format(&self, format: FormatId) -> Option<Vec<u8>> {
        self.0.get_format(format)
    }

    pub fn files(&self) -> Option<Vec<PathBuf>> {
        self.0.files()
    }

    pub fn preferred_format(&self, formats: &[FormatId]) -> Option<FormatId> {
        self.0.preferred_format(formats)
    }
}
