use super::obstacles::Obstacle;

pub struct Map {
    obstacles: Vec<Obstacle>,
}

impl Map {
    pub fn new(obstacles: Vec<Obstacle>) -> Self {
        Self {
            obstacles,
        }
    }

    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }

    pub fn obstacles(&self) -> &[Obstacle] {
        &self.obstacles
    }
}
