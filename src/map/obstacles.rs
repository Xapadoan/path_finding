use crate::geometry::line::Line;
use crate::geometry::point::{CollectionOfPoints, Point};
use crate::geometry::segment::{AsSegments, Segment, SideOfSegment};

pub struct Obstacle {
    points: Vec<Point>,
}

impl Obstacle {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            points,
        }
    }

    pub fn dodge_points<'a>(&'a self, segment: &Segment) -> (Point, Point) {
        if self.points.len() < 1 {
            panic!("Can't dodge an obstacle that as no points");
        }
        let mut left_side = (&self.points[0], 0.0);
        let mut right_side = (&self.points[0], 0.0);
        let line = Line::from(segment);
        for point in &self.points {
            let d = line.distance_from_point(point);
            match segment.side_of_segment(point) {
                SideOfSegment::Left => {
                    if left_side.1 < d {
                        left_side = (point, d)
                    }
                },
                SideOfSegment::Right => {
                    if right_side.1 < d {
                        right_side = (point, d)
                    }
                },
                SideOfSegment::Above => {
                    if left_side.1 < d {
                        left_side = (point, d)
                    }
                    if right_side.1 < d {
                        right_side = (point, d)
                    }
                }
            }
        }
        let left_escape_point = Segment::new(segment.start(), left_side.0).point_left_to_end();
        let right_escape_point = Segment::new(segment.start(), right_side.0).point_right_to_end();
        (left_escape_point, right_escape_point)
    }
}

impl CollectionOfPoints for Obstacle {
    fn points(&self) -> Vec<&Point> {
        self.points.iter().map(|p| p).collect()
    }
}

impl AsSegments for Obstacle {
    fn as_segments(&self) -> Vec<Segment> {
        let mut i = 1;
        let mut segments: Vec<Segment> = vec![];
        let number_of_points = self.points().len();
        if number_of_points > 2 {
            segments.push(Segment::new(&self.points[0], &self.points[number_of_points - 1]))
        }
        while i < number_of_points {
            segments.push(Segment::new(&self.points[i - 1], &self.points[i]));
            i += 1;
        }
        segments
    }
}
