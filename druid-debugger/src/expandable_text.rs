#![allow(unused)]
use druid::{
    widget::{prelude::*, Label},
    Color,
};
use druid::{FontDescriptor, FontFamily, Widget};

use crate::data;

pub struct EventWidget {
    inner: Label<()>,
    text: String,
    expanded: bool,
    selected: bool,
}

impl EventWidget {
    pub fn new() -> Self {
        let inner = Label::new("")
            .with_font(FontDescriptor::new(FontFamily::new_unchecked("Fira Code")).with_size(16.0));
        Self {
            inner,
            text: String::new(),
            expanded: false,
            selected: false,
        }
    }

    fn rebuild(&mut self) {
        if !self.expanded {
            let mut lines = self.text.lines();
            let mut first_line = lines.next().unwrap().to_string();
            if lines.next().is_some() {
                first_line.push_str("...}");
            }
            self.inner.set_text(first_line);
        } else {
            self.inner.set_text(self.text.clone());
        }
    }
}

impl Widget<data::Event> for EventWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut data::Event, env: &Env) {
        if let Event::MouseDown(m) = event {
            if m.count == 2 {
                self.expanded = !self.expanded;
                self.rebuild();
                ctx.request_update();
                return;
            } else if m.count == 1 {
                self.selected = !self.selected;
                ctx.request_paint();
            }
        }
        self.inner.event(ctx, event, &mut (), env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &data::Event,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            data.render(&mut self.text);
            self.rebuild();
        }
        self.inner.lifecycle(ctx, event, &(), env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &data::Event,
        data: &data::Event,
        env: &Env,
    ) {
        if !ctx.env_changed() {
            // Data changed
            self.text.clear();
            data.render(&mut self.text);
            self.rebuild();
        }
        self.inner.update(ctx, &(), &(), env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &data::Event,
        env: &Env,
    ) -> Size {
        let mut size = self.inner.layout(ctx, bc, &(), env);
        size.width += 10.;
        size.height += 10.;
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &data::Event, env: &Env) {
        let rect = ctx.size().to_rect();
        if self.expanded {
            ctx.fill(rect, &env.get(druid::theme::BACKGROUND_DARK));
        }
        if self.selected {
            ctx.fill(rect, &Color::AQUA.with_alpha(0.05));
        }
        self.inner.draw_at(ctx, (5., 5.));
    }
}
