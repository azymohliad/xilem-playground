pub struct Defer<F: FnOnce()> {
    on_drop: Option<F>
}

impl<F: FnOnce()> Defer<F> {
    pub fn new(on_drop: F) -> Self {
        Self { on_drop: Some(on_drop)}
    }
}
impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(on_drop) = self.on_drop.take() {
            (on_drop)();
        }
    }
}
