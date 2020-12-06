use std::sync::Arc;

use crate::Lens;

pub trait Diffable: 'static {
    type Diff;
    fn apply_diff(&mut self, diff: Self::Diff);
}


pub enum BoolDiff {
    Set(bool),
}

impl Diffable for bool {
    type Diff = BoolDiff;

    fn apply_diff(&mut self, diff: Self::Diff) {
        let BoolDiff::Set(new) = diff;
        *self = new;
    }
}

pub enum ArcStrDiff {
    Set(Arc<str>),
}

impl Diffable for Arc<str> {
    type Diff = ArcStrDiff;

    fn apply_diff(&mut self, diff: Self::Diff) {
        *self = match diff {
            ArcStrDiff::Set(new) => new,
        };
    }
}