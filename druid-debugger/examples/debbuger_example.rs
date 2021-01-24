use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{AppDelegate, AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

struct Delegate {
    init: bool,
}

impl AppDelegate<HelloState> for Delegate {
    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut HelloState,
        _env: &Env,
        ctx: &mut druid::DelegateCtx,
    ) {
        if !self.init {
            druid_debugger::launch::<HelloState>(ctx);
            self.init = true;
        }
    }
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title("Hello World!")
        .window_size((400.0, 400.0));

    // create the initial app s tate
    let initial_state: HelloState = HelloState {
        name: "World".into(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .delegate(Delegate { init: false })
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| {
        if data.name.is_empty() {
            "Hello anybody!?".to_string()
        } else {
            format!("Hello {}!", data.name)
        }
    })
    .with_text_size(32.0);

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .align_vertical(UnitPoint::CENTER)
}
