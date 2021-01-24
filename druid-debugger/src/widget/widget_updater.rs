use druid::{widget::Controller, Data, Env, LifeCycle, LifeCycleCtx, Widget};

pub struct WidgetUpdater<T, W> {
    f: Box<dyn FnMut(&mut W, &T)>,
}

impl<T, W> WidgetUpdater<T, W> {
    #[allow(unused)]
    pub fn new(f: impl FnMut(&mut W, &T) + 'static) -> Self {
        Self { f: Box::new(f) }
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for WidgetUpdater<T, W> {
    fn lifecycle(
        &mut self,
        child: &mut W,
        _ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            (self.f)(child, data);
        }
    }
    fn update(
        &mut self,
        child: &mut W,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &T,
        data: &T,
        _env: &druid::Env,
    ) {
        (self.f)(child, data);
    }
}
