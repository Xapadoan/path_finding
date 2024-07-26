use std::fmt::Display;

use crate::geometry::segment::{AsSegments, Segment};
use crate::geometry::Graph;
use crate::map::{Map, Obstacle};

use super::cell::CellDisplay;

const SCREEN_WIDTH: usize = 85;
const SCREEN_HEIGHT: usize = 35;

pub struct Screen {
    points: [[CellDisplay; SCREEN_WIDTH]; SCREEN_HEIGHT]
}

impl Screen {
    pub fn new() -> Self {
        let mut screen = Self {
            points: [[CellDisplay::Free; SCREEN_WIDTH]; SCREEN_HEIGHT],
        };
        screen.clear();
        screen
    }

    pub fn refresh(&mut self, map: &Map, graph: &Graph, title: &str) {
        self.clear();
        self.draw_map(&map);
        self.draw_graph(&graph);
        println!("{} {title}", self);
    }

    fn clear(&mut self) {
        self.points = [[CellDisplay::Free; SCREEN_WIDTH]; SCREEN_HEIGHT];
        self.points[0] = [CellDisplay::Border; SCREEN_WIDTH];
        self.points[SCREEN_HEIGHT - 1] = [CellDisplay::Border; SCREEN_WIDTH];
        
        for line in 1..SCREEN_HEIGHT - 1 {
            self.points[line][0] = CellDisplay::Border;
            self.points[line][SCREEN_WIDTH - 1] = CellDisplay::Border;
        }
    }

    fn draw_map(&mut self, map: &Map) {
        for obstacle in map.obstacles() {
            self.draw_obstacle(obstacle);
        }
    }

    fn draw_graph(&mut self, graph: &Graph) {
        for path in graph.paths_to_test() {
            for segment in path.as_segments() {
                self.draw_segment(&segment, CellDisplay::PathToTest);
            }
        }
        for path in graph.invalid_paths() {
            for segment in path.as_segments() {
                self.draw_segment(&segment, CellDisplay::InvalidPath);
            }
        }
        for path in graph.valid_paths() {
            for segment in path.as_segments() {
                self.draw_segment(&segment, CellDisplay::ValidPath);
            }
        }

        let origin = graph.origin();
        let destination = graph.destination();
        self.points[origin.y() as usize][origin.x() as usize] = CellDisplay::GraphOrigin;
        self.points[destination.y() as usize][destination.x() as usize] = CellDisplay::GraphDestination;
    }

    fn draw_obstacle(&mut self, obstacle: &Obstacle) {
        for segment in obstacle.as_segments() {
            self.draw_segment(&segment, CellDisplay::Obstacle);
        }
    }

    fn draw_segment(&mut self, segment: &Segment, kind: CellDisplay) {
        for (x, y) in segment.to_bresenham() {
            self.points[segment.start().y() as usize][segment.start().x() as usize] = CellDisplay::SegmentLimit;
            self.points[y as usize][x as usize] = kind;
            self.points[segment.end().y() as usize][segment.end().x() as usize] = CellDisplay::SegmentLimit;
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.points {
            for col in line {
                write!(f, "{}", col)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}
