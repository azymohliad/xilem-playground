use accesskit::Role;
use masonry::{
    kurbo::{Cap, Line, Point, Size, Stroke},
    vello::{
        peniko::{Brush, Color, Fill},
        Scene,
    },
    widget::WidgetMut,
    AccessCtx, AccessEvent, Affine, BoxConstraints, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, PointerEvent, StatusChange, TextEvent, Widget, WidgetId,
};
use smallvec::SmallVec;
use std::sync::Arc;

pub struct BarPlotWidget {
    pub data: Arc<Vec<f64>>,
    pub gap: f64,
    pub bg_brush: Arc<Brush>,
    pub fg_brush: Arc<Brush>,
}

impl Default for BarPlotWidget {
    fn default() -> Self {
        Self {
            data: Arc::new(Vec::new()),
            gap: 0.0,
            bg_brush: Arc::new(Color::rgb8(0x20, 0x20, 0x20).into()),
            fg_brush: Arc::new(Color::rgb8(0xD0, 0xD0, 0xD0).into()),
        }
    }
}

pub trait BarPlotExt {
    fn set_data(&mut self, data: impl Into<Arc<Vec<f64>>>);
}

impl BarPlotExt for WidgetMut<'_, BarPlotWidget> {
    fn set_data(&mut self, data: impl Into<Arc<Vec<f64>>>) {
        self.widget.data = data.into();
        self.ctx.request_paint();
    }
}

impl Widget for BarPlotWidget {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _event: &PointerEvent) {}

    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}

    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle) {}

    fn on_status_change(&mut self, _ctx: &mut LifeCycleCtx, _event: &StatusChange) {}

    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let size = ctx.size();
        let dy = size.height / self.data.iter().copied().reduce(f64::max).unwrap_or(1.0);
        let dx = size.width / self.data.len() as f64;

        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            self.bg_brush.as_ref(),
            Some(Affine::scale_non_uniform(size.width, size.height)),
            &size.to_rect(),
        );
        for (i, &v) in self.data.iter().enumerate() {
            let p1 = Point::new(dx * (i as f64 + 0.5), size.height - v * dy);
            let p2 = Point::new(p1.x, size.height);
            scene.stroke(
                &Stroke::new(1.0f64.max(dx - self.gap)).with_caps(Cap::Butt),
                Affine::IDENTITY,
                self.fg_brush.as_ref(),
                Some(Affine::scale_non_uniform(size.width, size.height)),
                &Line::new(p1, p2),
            );
        }
    }

    fn accessibility_role(&self) -> Role {
        Role::Window
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        ctx.current_node().set_name("Placeholder");
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        SmallVec::new()
    }
}
