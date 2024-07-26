use std::borrow::Borrow;
use std::rc::Rc;

use crate::map::Map;

use super::intersection::Intersection;
use super::point::{CollectionOfPoints, Point};
use super::segment::AsSegments;

#[derive(Clone, Debug)]
pub struct Path {
    id: u32,
    points: Vec<Rc<Point>>,
}

impl Path {
    pub fn is_valid(&self, map: &Map) -> bool {
        for obstacle in map.obstacles() {
            if self.intersects(obstacle) {
                return false
            }
        }
        true
    }

    pub fn break_segment(&self, seg_start: &Point, new_left: Rc<Point>, new_right: Rc<Point>) -> (Path, Path) {
        let mut alt_path1 = self.clone();
        alt_path1.id <<= 1;
        let mut alt_path2 = self.clone();
        alt_path2.id <<= 1;
        alt_path2.id |= 1;
        let len = self.points.len();
        let mut i = 0;
        while i < len {
            let p: &Point = self.points[i].borrow();
            if p == seg_start {
                alt_path1.points.insert(i + 1, new_left);
                alt_path2.points.insert(i + 1, new_right);
                return (alt_path1, alt_path2);
            }
            i += 1;
        }
        (alt_path1, alt_path2)
    }

    pub fn len(&self) -> f64 {
        let mut total = 0.0;
        for segment in self.as_segments() {
            total += segment.len();
        }
        total
    }

    // pub fn id(&self) -> u32 {
    //     self.id
    // }

    pub fn new(id: u32, points: Vec<Rc<Point>>) -> Self {
        Self { id, points }
    }
}

impl CollectionOfPoints for Path {
    fn points(&self) -> Vec<&Point> {
        self.points.iter().map(|p| p.borrow()).collect()
    }
}

impl AsSegments for Path {}

impl Intersection for Path {}
