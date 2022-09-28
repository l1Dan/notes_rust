use crate::tools::geometry::{Point, Rect, Size};

#[derive(Debug)]
pub struct View {
    pub frame: Rect,
    pub bounds: Rect,
}

impl View {
    pub fn set_frame(&mut self, frame: Rect) {
        self.frame = frame;
        self.bounds = Rect {
            origin: Point::default(),
            ..self.frame
        }
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.frame = Rect {
            size: bounds.size,
            ..self.frame
        }
    }
}

impl Default for View {
    fn default() -> Self {
        let rect = Rect {
            origin: Point::default(),
            size: Size::default(),
        };
        Self {
            frame: rect,
            bounds: rect,
        }
    }
}
