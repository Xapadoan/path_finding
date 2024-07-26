use std::error::Error;
use std::io::stdin;
use std::rc::Rc;
use std::cell::RefCell;

use geometry::{point::Point, Graph};
use map::gen_map;
use screen::Screen;

mod geometry {
    pub mod point;
    pub mod segment;
    mod path;
    mod graph;
    mod intersection;
    pub mod line;

    pub use graph::Graph;
}

mod map {
    mod map;
    mod obstacles;
    mod gen_map;

    pub use map::Map;
    pub use obstacles::Obstacle;
    pub use gen_map::gen_map;
}

mod screen {
    mod screen;
    mod cell;

    pub use screen::Screen;
}

pub fn run () -> Result<(), Box<dyn Error>> {
    let map = gen_map();
    let mut graph = Graph::new(Point::new(5, 32), Point::new(77, 7));

    // I can do it because i know screen wil not be borrowed twice at the same time
    let screen = Rc::new(RefCell::new(Screen::new()));

    let between_steps = |graph: &Graph, title: &str| {
        let screen_clone = Rc::clone(&screen);
        let mut buf = String::new();
        screen_clone.borrow_mut().refresh(&map, graph, title);
        if let Err(_) = stdin().read_line(&mut buf) {
            panic!("io::stdin error")
        }
    };

    between_steps(&graph, "Initial State");

    graph.solve(
        &map,
        |graph| between_steps(graph, "After Tests"),
        |graph| between_steps(graph, "After Filter"),
        |graph| between_steps(graph, "After break"),
    );
    Ok(())
}
