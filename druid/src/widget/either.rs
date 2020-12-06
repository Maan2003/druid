// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A widget that switches dynamically between two child views.

use crate::widget::prelude::*;
use crate::{Point, WidgetPod};

/// A widget that switches between two possible child views.
pub struct Either<T: Diffable> {
    closure: Box<dyn Fn(&T, Option<&T::Diff>, &Env) -> bool>,
    true_branch: WidgetPod<T, Box<dyn Widget<T>>>,
    false_branch: WidgetPod<T, Box<dyn Widget<T>>>,
    current: bool,
}

impl<T: Diffable> Either<T> {
    /// Create a new widget that switches between two views.
    ///
    /// The given closure is evaluated on data change. If its value is `true`, then
    /// the `true_branch` widget is shown, otherwise `false_branch`.
    pub fn new(
        closure: impl Fn(&T, Option<&T::Diff>, &Env) -> bool + 'static,
        true_branch: impl Widget<T> + 'static,
        false_branch: impl Widget<T> + 'static,
    ) -> Either<T> {
        Either {
            closure: Box::new(closure),
            true_branch: WidgetPod::new(true_branch).boxed(),
            false_branch: WidgetPod::new(false_branch).boxed(),
            current: false,
        }
    }
}

impl<T: Diffable> Widget<T> for Either<T> {
    fn event(&mut self, ctx: &mut EventCtx<T>, event: &Event, data: &T, env: &Env) {
        if event.should_propagate_to_hidden() {
            self.true_branch.event(ctx, event, data, env);
            self.false_branch.event(ctx, event, data, env);
        } else {
            self.current_widget().event(ctx, event, data, env)
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.current = (self.closure)(data, None, env);
        }

        if event.should_propagate_to_hidden() {
            self.true_branch.lifecycle(ctx, event, data, env);
            self.false_branch.lifecycle(ctx, event, data, env);
        } else {
            self.current_widget().lifecycle(ctx, event, data, env)
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, update: &T::Diff, env: &Env) {
        let current = (self.closure)(old_data, Some(update), env);
        if current != self.current {
            self.current = current;
            ctx.request_layout();
        }
        self.current_widget().update(ctx,old_data, update, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let current_widget = self.current_widget();
        let size = current_widget.layout(ctx, bc, data, env);
        current_widget.set_origin(ctx, data, env, Point::ORIGIN);
        ctx.set_paint_insets(current_widget.paint_insets());
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.current_widget().paint(ctx, data, env)
    }
}

impl<T: Diffable> Either<T> {
    fn current_widget(&mut self) -> &mut WidgetPod<T, Box<dyn Widget<T>>> {
        if self.current {
            &mut self.true_branch
        } else {
            &mut self.false_branch
        }
    }
}
