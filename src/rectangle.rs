//! Draw rectangle

use current::Modifier;
use internal;
use triangulation;
use Context;
use BackEnd;
use ImageSize;
use Color;

/// The shape of the rectangle
pub enum Shape {
    /// Square corners
    Square,
    /// Round corners
    Round(internal::Radius),
    /// Bevel corners
    Bevel(internal::Radius),
}

/// The border of the rectangle
pub struct Border {
    /// The color of the border
    pub color: internal::Color,
    /// The radius of the border
    pub radius: internal::Radius,
}

/// A filled rectangle
#[deriving(Copy)]
pub struct Rectangle {
    /// The rectangle color
    pub color: internal::Color,
    /// The roundness of the rectangle
    pub shape: Shape,
    /// The border
    pub border: Option<Border>,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub fn new(color: internal::Color) -> Rectangle {
        Rectangle {
            color: color,
            shape: Shape::Square,
            border: None,
        }
    }

    /// Draws the rectangle
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self, 
        rectangle: internal::Rectangle, 
        c: &Context, 
        back_end: &mut B
    ) {
        if self.color[3] != 0.0 {
            back_end.color(self.color);
            match self.shape {
                Shape::Square => {
                    back_end.tri_list(
                        &triangulation::rect_tri_list_xy(c.transform, rectangle),
                    );
                }
                Shape::Round(round_radius) => {
                    triangulation::with_round_rectangle_tri_list(
                        32,
                        c.transform,
                        rectangle,
                        round_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
                Shape::Bevel(bevel_radius) => {
                    triangulation::with_round_rectangle_tri_list(
                        2,
                        c.transform,
                        rectangle,
                        bevel_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
            }
        }
       
        if let Some(Border { color, radius: border_radius }) = self.border {
            if color[3] == 0.0 { return; }
            back_end.color(color);
            match self.shape {
                Shape::Square => {
                    back_end.tri_list(
                        &triangulation::rect_border_tri_list_xy(
                            c.transform, rectangle, border_radius),
                    );
                }
                Shape::Round(round_radius) => {
                    triangulation::with_round_rectangle_border_tri_list(
                        128,
                        c.transform,
                        rectangle,
                        round_radius,
                        border_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
                Shape::Bevel(bevel_radius) => {
                    triangulation::with_round_rectangle_border_tri_list(
                        2,
                        c.transform,
                        rectangle,
                        bevel_radius,
                        border_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
            }
        } 
    }
}

impl Modifier<Rectangle> for Color {
    fn modify(self, r: &mut Rectangle) {
        let Color(val) = self;
        r.color = val;
    }
}

impl Modifier<Rectangle> for Shape {
    fn modify(self, r: &mut Rectangle) {
        r.shape = self;
    }
}

impl Modifier<Rectangle> for Border {
    fn modify(self, r: &mut Rectangle) {
        r.border = Some(self);
    }
}

#[cfg(test)]
mod test {
    use super::Rectangle;
    use super::Shape;
    use super::Border;
    use Color;
    use current::Set;

    #[test]
    fn test_rectangle() {
        let _rectangle = Rectangle::new([1.0, ..4])
            .set(Color([0.0, ..4]))
            .set(Shape::Round(10.0))
            .set(Border { color: [0.0, ..4], radius: 4.0 });
    }
}
