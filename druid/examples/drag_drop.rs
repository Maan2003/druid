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

//! An example of drag drop widget.

use druid::widget::{prelude::*, Flex, RawLabel};
use druid::{AppLauncher, Color, LocalizedString, WindowDesc};

struct CustomWidget {
    draging_over: bool,
}

impl Widget<String> for CustomWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, _env: &Env) {
        match event {
            Event::DropEnter => {
                self.draging_over = true;
                let files = ctx.drop_context().unwrap().files();
                if let Some(files) = files {
                    data.clear();
                    data.push_str("oh some files over me:\n");
                    for path in files {
                        data.push_str(&path.into_os_string().into_string().unwrap());
                        data.push('\n');
                    }
                } else {
                    *data = String::from("I only handle files :/");
                }
                ctx.request_paint();
            }
            Event::DropLeave => {
                data.clear();
                self.draging_over = false;
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        let rect = ctx.size().to_rect();
        if self.draging_over {
            ctx.fill(rect, &Color::RED);
        } else {
            ctx.fill(rect, &Color::WHITE);
        }
    }
}

pub fn main() {
    let window = WindowDesc::new(ui()).title(LocalizedString::new("Fancy Colors"));
    AppLauncher::with_window(window)
        .log_to_console()
        .launch("Throw files at the white box".to_string())
        .expect("launch failed");
}

fn ui() -> impl Widget<String> {
    Flex::column()
        .with_child(CustomWidget {
            draging_over: false,
        })
        .with_child(RawLabel::new())
}
