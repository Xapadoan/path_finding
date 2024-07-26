use std::rc::Rc;

use crate::map::Map;

use super::intersection::Intersection;
use super::path::Path;
use super::point::Point;
use super::segment::AsSegments;

pub struct Graph {
    origin: Rc<Point>,
    destination: Rc<Point>,
    valid_paths: Vec<Path>,
    paths_to_test: Vec<Path>,
    invalid_paths: Vec<Path>,
}

impl Graph {
    pub fn new(origin: Point, destination: Point) -> Self {
        let rc_origin = Rc::new(origin);
        let rc_dst = Rc::new(destination);
        let first_path = Path::new(1, vec![Rc::clone(&rc_origin), Rc::clone(&rc_dst)]);
        Self {
            origin: rc_origin,
            destination: rc_dst,
            valid_paths: vec![],
            paths_to_test: vec![first_path],
            invalid_paths: vec![],
        }
    }

    pub fn solve<Ft, Ff, Fb>(
        &mut self,
        map: &Map,
        after_test: Ft,
        after_filter: Ff,
        after_break: Fb,
    )
    where 
        Ft: Fn(&Graph) -> (),
        Ff: Fn(&Graph) -> (),
        Fb: Fn(&Graph) -> ()
    {
        while self.paths_to_test.len() > 0 {
            self.test_paths(map, &after_test);
            self.filter_longer_paths(&after_filter);
            self.break_invalid_paths(map, &after_break);
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn destination(&self) -> &Point {
        &self.destination
    }

    pub fn paths_to_test(&self) -> &Vec<Path> {
        &self.paths_to_test
    }

    pub fn invalid_paths(&self) -> &Vec<Path> {
        &self.invalid_paths
    }

    pub fn valid_paths(&self) -> &Vec<Path> {
        &self.valid_paths
    }

    fn test_paths<F: Fn(&Graph) -> ()>(
        &mut self,
        map: &Map,
        after: F,
    ) {
        if self.paths_to_test.len() < 1 {
            return after(self);
        }
        let path = self.paths_to_test.swap_remove(0);
        if path.is_valid(map) {
            self.valid_paths.push(path);
        } else {
            self.invalid_paths.push(path)
        }
        return self.test_paths(map, after);
    }

    fn break_invalid_paths<F: Fn(&Graph) -> ()>(
        &mut self,
        map: &Map,
        after: F,
    ) {
        if self.invalid_paths.len() < 1 {
            return after(self);
        }
        let path = self.invalid_paths.swap_remove(0);
        for segment in path.as_segments() {
            for obstacle in map.obstacles() {
                if segment.intersects(obstacle) {
                    let (p1, p2) = obstacle.dodge_points(&segment);
                    let rc_p1 = Rc::new(p1);
                    let rc_p2 = Rc::new(p2);
                    let (alt_path1, alt_path2) = path.break_segment(
                        segment.start(),
                        Rc::clone(&rc_p1),
                        Rc::clone(&rc_p2),
                    );
                    // self.all_points.push(rc_p1);
                    self.paths_to_test.push(alt_path1);
                    // self.all_points.push(rc_p2);
                    self.paths_to_test.push(alt_path2);
                    return self.break_invalid_paths(map, after);
                }
            }
        }
        return self.break_invalid_paths(map, after);
    }

    fn filter_longer_paths<F: Fn(&Graph) -> ()>(
        &mut self,
        after: F,
    ) {
        self.remove_long_valid_paths();
        self.remove_long_invalid_paths();
        after(&self);
    }

    fn remove_long_valid_paths(&mut self) {
        if self.valid_paths.len() < 2 {
            return;
        }
        let mut shorter_path = self.valid_paths.swap_remove(0);
        while self.valid_paths.len() > 0 {
            let cmp = self.valid_paths.swap_remove(0);
            if cmp.len() < shorter_path.len() {
                shorter_path = cmp;
            }
        }
        self.valid_paths.push(shorter_path);
        return;
    }

    fn remove_long_invalid_paths(&mut self) {
        if self.valid_paths.len() < 1 {
            return;
        }
        let shorter_path_len = self.valid_paths[0].len();
        let mut i = 0;
        while i < self.invalid_paths.len() {
            if self.invalid_paths[i].len() < shorter_path_len {
                i += 1;
            } else {
                self.invalid_paths.swap_remove(i);
            }
        }
        return;
    }
}
