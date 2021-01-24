use super::super::{data::DebuggerData, delegate};
use delegate::Delegate;
use druid::{widget::prelude::*, WidgetPod};

pub struct AppWrapper {
    pub inner: WidgetPod<DebuggerData, Box<dyn Widget<DebuggerData>>>,
    pub data: DebuggerData,
    pub delegate: Delegate,
}

impl<T: Data> Widget<T> for AppWrapper {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, env: &Env) {
        if let Event::Command(cmd) = event {
            self.delegate.command(ctx, cmd, &mut self.data);
        } else {
            self.delegate.event(ctx, event, &mut self.data);
        }
        self.inner.event(ctx, event, &mut self.data, env);
        ctx.request_update();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, &self.data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, _data: &T, env: &Env) {
        self.inner.update(ctx, &self.data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, &self.data, env);
        self.inner.set_origin(ctx, &self.data, env, (0., 0.).into());
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        self.inner.paint(ctx, &self.data, env)
    }
}
