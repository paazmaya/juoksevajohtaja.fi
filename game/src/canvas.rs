//! Drawing surface abstraction.
//!
//! [`crate::render::Renderer`] draws through this trait instead of `CanvasRenderingContext2d` directly, so the
//! rendering logic runs in native unit tests against a recording canvas. The browser implementation is a thin
//! one-to-one delegation to the Canvas 2D API.

/// A failed canvas call (the browser threw).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasError;

/// The subset of the Canvas 2D API used by the renderer. Method names match `web_sys::CanvasRenderingContext2d`.
pub trait Canvas {
    /// Image type that can be drawn.
    type Image;

    /// Natural size of a fully loaded image, `None` while it is still loading or broken.
    fn image_size(&self, image: &Self::Image) -> Option<(f64, f64)>;

    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn set_fill_style_str(&self, value: &str);
    fn set_stroke_style_str(&self, value: &str);
    fn set_font(&self, value: &str);
    fn set_line_width(&self, value: f64);
    fn fill_text(&self, text: &str, x: f64, y: f64) -> Result<(), CanvasError>;
    fn begin_path(&self);
    fn close_path(&self);
    fn fill(&self);
    fn stroke(&self);
    fn move_to(&self, x: f64, y: f64);
    fn line_to(&self, x: f64, y: f64);
    fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64);
    fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), CanvasError>;
    fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Result<(), CanvasError>;
    fn draw_image_with_html_image_element_and_dw_and_dh(
        &self,
        image: &Self::Image,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) -> Result<(), CanvasError>;
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::{Canvas, CanvasError};
    use web_sys::{CanvasRenderingContext2d as Ctx, HtmlImageElement};

    impl Canvas for Ctx {
        type Image = HtmlImageElement;

        fn image_size(&self, image: &HtmlImageElement) -> Option<(f64, f64)> {
            (image.complete() && image.natural_width() > 0).then(|| {
                (
                    f64::from(image.natural_width()),
                    f64::from(image.natural_height()),
                )
            })
        }
        fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
            Ctx::clear_rect(self, x, y, w, h)
        }
        fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) {
            Ctx::fill_rect(self, x, y, w, h)
        }
        fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) {
            Ctx::stroke_rect(self, x, y, w, h)
        }
        fn set_fill_style_str(&self, value: &str) {
            Ctx::set_fill_style_str(self, value)
        }
        fn set_stroke_style_str(&self, value: &str) {
            Ctx::set_stroke_style_str(self, value)
        }
        fn set_font(&self, value: &str) {
            Ctx::set_font(self, value)
        }
        fn set_line_width(&self, value: f64) {
            Ctx::set_line_width(self, value)
        }
        fn fill_text(&self, text: &str, x: f64, y: f64) -> Result<(), CanvasError> {
            Ctx::fill_text(self, text, x, y).map_err(|_| CanvasError)
        }
        fn begin_path(&self) {
            Ctx::begin_path(self)
        }
        fn close_path(&self) {
            Ctx::close_path(self)
        }
        fn fill(&self) {
            Ctx::fill(self)
        }
        fn stroke(&self) {
            Ctx::stroke(self)
        }
        fn move_to(&self, x: f64, y: f64) {
            Ctx::move_to(self, x, y)
        }
        fn line_to(&self, x: f64, y: f64) {
            Ctx::line_to(self, x, y)
        }
        fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
            Ctx::quadratic_curve_to(self, cpx, cpy, x, y)
        }
        fn arc(
            &self,
            x: f64,
            y: f64,
            radius: f64,
            start: f64,
            end: f64,
        ) -> Result<(), CanvasError> {
            Ctx::arc(self, x, y, radius, start, end).map_err(|_| CanvasError)
        }
        fn arc_to(
            &self,
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            radius: f64,
        ) -> Result<(), CanvasError> {
            Ctx::arc_to(self, x1, y1, x2, y2, radius).map_err(|_| CanvasError)
        }
        fn draw_image_with_html_image_element_and_dw_and_dh(
            &self,
            image: &HtmlImageElement,
            x: f64,
            y: f64,
            w: f64,
            h: f64,
        ) -> Result<(), CanvasError> {
            Ctx::draw_image_with_html_image_element_and_dw_and_dh(self, image, x, y, w, h)
                .map_err(|_| CanvasError)
        }
    }
}
