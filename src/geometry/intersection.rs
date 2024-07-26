use super::point::Point;
use super::segment::{AsSegments, Segment};

pub trait Intersection: AsSegments {
    fn intersects<T: AsSegments>(&self, obstacle: &T) -> bool {
        for s1 in self.as_segments() {
            for s2 in obstacle.as_segments() {
                if have_intersection(&s1, &s2) {
                    return true
                }
            }
        }
        false
    }
}

fn points_are_ccw(p1: &Point, p2: &Point, p3: &Point) -> bool {
    return (p3.y() as i16 - p1.y() as i16) * (p2.x() as i16 - p1.x() as i16) >
        (p2.y() as i16 - p1.y() as i16) * (p3.x() as i16 - p1.x() as i16);
}

fn have_intersection(s1: &Segment, s2: &Segment) -> bool {
    return (
        points_are_ccw(s1.start(), s2.start(), s2.end()) !=
        points_are_ccw(s1.end(), s2.start(), s2.end())
    ) && (
        points_are_ccw(s1.start(), s1.end(), s2.start()) !=
        points_are_ccw(s1.start(), s1.end(), s2.end())
    );
}
