use super::{point::Point, segment::Segment};

// General form: ax + by + c = 0
pub struct Line {
    a: i16,
    b: i16,
    c: i16,
}

impl Line {
    pub fn distance_from_point(&self, point: &Point) -> f64 {
        let numerator = (self.a * point.x() as i16 + self.b * point.y() as i16 + self.c).abs() as f64;
        let denominator = ((self.a * self.a) as f64 + (self.b * self.b) as f64).sqrt();
        return numerator / denominator;
    }
}

impl<'p> From<&Segment<'p>> for Line {
    fn from(segment: &Segment) -> Self {
        let x1 = segment.start().x() as i16;
        let y1 = segment.start().y() as i16;
        let x2 = segment.end().x() as i16;
        let y2 = segment.end().y() as i16;
        let a = y2 - y1;
        let b = x1 - x2;
        let c = y1 * (x2 - x1) - x1 * (y2 - y1);
        Self { a, b, c}
    }
}
