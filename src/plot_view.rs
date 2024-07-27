use masonry::vello::peniko::Brush;
use std::sync::Arc;
use xilem::{
    core::{DynMessage, MessageResult, Mut, View, ViewId},
    Pod, ViewCtx,
};

use crate::plot_widget::{BarPlotExt, BarPlotWidget};
pub fn bar_plot(data: impl Into<Arc<Vec<f64>>>) -> BarPlot {
    BarPlot {
        data: data.into(),
        gap: 0.0,
        bg_brush: None,
        fg_brush: None,
    }
}

pub struct BarPlot {
    data: Arc<Vec<f64>>,
    gap: f64,
    bg_brush: Option<Arc<Brush>>,
    fg_brush: Option<Arc<Brush>>,
}

impl BarPlot {
    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    pub fn background(mut self, brush: impl Into<Brush>) -> Self {
        self.bg_brush = Some(Arc::new(brush.into()));
        self
    }

    pub fn foreground(mut self, brush: impl Into<Brush>) -> Self {
        self.fg_brush = Some(Arc::new(brush.into()));
        self
    }
}

impl<State, Action> View<State, Action, ViewCtx> for BarPlot {
    type Element = Pod<BarPlotWidget>;
    type ViewState = ();

    fn build(&self, _ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let mut widget = BarPlotWidget {
            data: self.data.clone(),
            gap: self.gap,
            ..Default::default()
        };
        if let Some(brush) = &self.bg_brush {
            widget.bg_brush = brush.clone();
        }
        if let Some(brush) = &self.fg_brush {
            widget.fg_brush = brush.clone();
        }
        (Pod::new(widget), ())
    }

    fn rebuild<'el>(
        &self,
        prev: &Self,
        (): &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'el, Self::Element>,
    ) -> Mut<'el, Self::Element> {
        if prev.data != self.data {
            element.set_data(self.data.clone());
            ctx.mark_changed();
        }
        element
    }

    fn teardown(&self, (): &mut Self::ViewState, _: &mut ViewCtx, _: Mut<'_, Self::Element>) {}

    fn message(
        &self,
        (): &mut Self::ViewState,
        _id_path: &[ViewId],
        message: DynMessage,
        _app_state: &mut State,
    ) -> MessageResult<Action> {
        MessageResult::Stale(message)
    }
}
