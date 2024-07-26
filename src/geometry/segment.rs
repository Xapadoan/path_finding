use bresenham::Bresenham;

use super::intersection::Intersection;
use super::point::{CollectionOfPoints, Point};

const DODGE_DISTANCE: u8 = 1;

#[derive(Clone)]
pub struct Segment<'p> {
    start: &'p Point,
    end: &'p Point,
}

pub enum SideOfSegment {
    Left,
    Right,
    Above,
}

pub enum Direction {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl<'p> Segment<'p> {
    pub fn new(start: &'p Point, end: &'p Point) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn start(&self) -> &Point {
        &self.start
    }

    pub fn end(&self) -> &Point {
        &self.end
    }

    pub fn to_bresenham(&self) -> Bresenham {
        Bresenham::new(
            (self.start.x() as isize, self.start.y() as isize),
            (self.end.x() as isize, self.end.y() as isize),
        )
    }

    pub fn side_of_segment(&self, point: &Point) -> SideOfSegment {
        let side_coef =
            (self.end.x() as i16 - self.start.x() as i16) * (point.y() as i16 - self.start.y() as i16) -
            (self.end.y() as i16 - self.start.y() as i16) * (point.x() as i16 - self.start.x() as i16);
        if side_coef > 0 {
            return SideOfSegment::Left;
        } else if side_coef < 0 {
            return SideOfSegment::Right;
        } else {
            return SideOfSegment::Above
        }
    }

    pub fn direction(&self) -> Direction {
        let dy = self.end.y() as i16 - self.start.y() as i16;
        let dx = self.end.x() as i16 - self.start.x() as i16;
        if dy > 0 {
            // left, topleft, top, topright, right
            if dx > 0 {
                // right, topright, top
                if dx > dy {
                    // right, topright
                    if dx > dy * 2 { Direction::Right } else { Direction::TopRight }
                } else {
                    // top, topright
                    if dy > dx * 2 { Direction::Top } else { Direction::TopRight }
                }
            } else {
                // left, topleft, top
                if -dx > dy {
                    // left, topleft
                    if -dx > dy * 2 { Direction::Left } else { Direction::TopLeft }
                } else {
                    // top, topleft
                    if dy > -dx * 2 { Direction::Top } else { Direction::TopLeft }
                }
            }
        } else {
            // left, bottomleft, bottom, bottomright, right
            if dx > 0 {
                //right, bottomrigth, bottom
                if dx > -dy {
                    // right, bottomright
                    if dx > -dy * 2 { Direction::Right } else { Direction::BottomRight }
                } else {
                    // bottom, bottomright
                    if -dy > dx * 2 { Direction::Bottom } else { Direction::BottomRight }
                }
            } else {
                // left, bottomleft, bottom
                if -dx > -dy {
                    // left, bottomleft
                    if -dx > -dy * 2 { Direction::Left } else { Direction::BottomLeft }
                } else {
                    // bottom, bottomleft
                    if -dy > -dx * 2 { Direction::Bottom } else { Direction::BottomLeft }
                }
            }
        }
    }

    pub fn point_left_to_end(&self) -> Point {
        match self.direction() {
            Direction::Bottom => Point::new(self.end.x() + DODGE_DISTANCE, self.end.y()),
            Direction::BottomLeft => Point::new(self.end.x() + DODGE_DISTANCE, self.end.y() - DODGE_DISTANCE),
            Direction::BottomRight => Point::new(self.end.x() + DODGE_DISTANCE, self.end.y() + DODGE_DISTANCE),
            Direction::Top => Point::new(self.end.x() - DODGE_DISTANCE, self.end.y()),
            Direction::TopLeft => Point::new(self.end.x() - DODGE_DISTANCE, self.end.y() - DODGE_DISTANCE),
            Direction::TopRight => Point::new(self.end.x() - DODGE_DISTANCE, self.end.y() + DODGE_DISTANCE),
            Direction::Left => Point::new(self.end.x(), self.end.y() - DODGE_DISTANCE),
            Direction::Right => Point::new(self.end.x(), self.end.y() + DODGE_DISTANCE)
        }
    }

    pub fn point_right_to_end(&self) -> Point {
        match self.direction() {
            Direction::Bottom => Point::new(self.end.x() - DODGE_DISTANCE, self.end.y()),
            Direction::BottomLeft => Point::new(self.end.x() - DODGE_DISTANCE, self.end.y() + DODGE_DISTANCE),
            Direction::BottomRight => Point::new(self.end.x() - DODGE_DISTANCE, self.end.y() - DODGE_DISTANCE),
            Direction::Top => Point::new(self.end.x() + DODGE_DISTANCE, self.end.y()),
            Direction::TopLeft => Point::new(self.end.x() + DODGE_DISTANCE, self.end.y() + DODGE_DISTANCE),
            Direction::TopRight => Point::new(self.end.x() + DODGE_DISTANCE, self.end.y() - DODGE_DISTANCE),
            Direction::Left => Point::new(self.end.x(), self.end.y() + DODGE_DISTANCE),
            Direction::Right => Point::new(self.end.x(), self.end.y() - DODGE_DISTANCE)
        }
    }

    pub fn len(&self) -> f64 {
        let dx = self.end.x() as f64 - self.start.x() as f64;
        let dy = self.end.y() as f64 - self.start.y() as f64;
        return (dx * dx + dy * dy).sqrt();
    }
}

impl<'p> CollectionOfPoints for Segment<'p> {
    fn points(&self) -> Vec<&Point> {
        vec![self.start, self.end]
    }
}

impl<'p> AsSegments for Segment<'p> {}

impl<'p> Intersection for Segment<'p> {}

pub trait AsSegments: CollectionOfPoints {
    fn as_segments(&self) -> Vec<Segment> {
        let mut i = 1;
        let mut segments: Vec<Segment> = vec![];
        let number_of_points = self.points().len();
        while i < number_of_points {
            segments.push(Segment::new(&self.points()[i - 1], &self.points()[i]));
            i += 1;
        }
        segments
    }
}
