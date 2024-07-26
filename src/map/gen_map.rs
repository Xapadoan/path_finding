use crate::geometry::point::Point;

use super::{obstacles::Obstacle, Map};

pub fn gen_map() -> Map {
    let mut map = Map::new(vec![]);

    map.add_obstacle(Obstacle::new(
        [
            Point::new(30, 15),
            Point::new(33, 20),
            Point::new(27, 27),
        ].to_vec()
    ));
    map.add_obstacle(Obstacle::new(
        [
            Point::new(70, 4),
            Point::new(60, 6),
            Point::new(64, 17),
            Point::new(67, 23),
        ].to_vec()
    ));
    map.add_obstacle(Obstacle::new(
        [
            Point::new(44, 2),
            Point::new(46, 7),
            Point::new(52, 11),
            Point::new(56, 4),
        ].to_vec()
    ));

    map
}