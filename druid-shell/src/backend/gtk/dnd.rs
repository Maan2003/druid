use std::path::PathBuf;

use crate::{dnd::DropAction, FormatId};
use gdk::{Atom, DragAction};

use super::clipboard::Clipboard;

#[derive(Clone)]
pub struct DropContext {
    pub(crate) gtk_ctx: gdk::DragContext,
    pub(crate) clipboard: Clipboard,
    pub(crate) time: u32,
}

impl DropContext {
    pub fn deny(&self) {
        self.gtk_ctx.drag_status(DragAction::empty(), self.time);
    }

    pub fn action(&self) -> DropAction {
        match self.gtk_ctx.get_suggested_action() {
            DragAction::COPY => DropAction::Copy,
            DragAction::MOVE => DropAction::Move,
            _ => DropAction::Copy,
        }
    }

    pub fn set_action(&self, action: DropAction) {
        self.gtk_ctx.drag_status(
            match action {
                DropAction::Copy => DragAction::COPY,
                DropAction::Move => DragAction::MOVE,
            },
            self.time,
        )
    }

    pub fn get_format(&self, format: FormatId) -> Option<Vec<u8>> {
        self.clipboard.get_format(format)
    }

    pub fn files(&self) -> Option<Vec<PathBuf>> {
        let utf8 = self.get_format("text/uri-list")?;
        let string = String::from_utf8(utf8).ok()?;
        Some(
            string
                .lines()
                .map(|p| PathBuf::from(p.strip_prefix("file://").unwrap_or(p)))
                .collect(),
        )
    }

    pub fn preferred_format(&self, formats: &[FormatId]) -> Option<FormatId> {
        let targets = self.gtk_ctx.list_targets();
        let format_atoms = formats
            .iter()
            .map(|fmt| Atom::intern(fmt))
            .collect::<Vec<_>>();
        for atom in targets.iter() {
            if let Some(idx) = format_atoms.iter().position(|fmt| fmt == atom) {
                return Some(formats[idx]);
            }
        }
        None
    }
}
