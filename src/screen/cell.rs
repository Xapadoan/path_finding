use std::fmt::Display;
use colored::Colorize;

#[derive(Clone, Copy)]
pub enum CellDisplay {
    Free,
    Border,
    Obstacle,
    GraphOrigin,
    GraphDestination,
    PathToTest,
    InvalidPath,
    ValidPath,
    SegmentLimit,
}

impl Display for CellDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => write!(f, "{}", "##".white().on_white()),
            Self::Free => write!(f, "  "),
            Self::Obstacle => write!(f, "{}", "##".purple().on_purple()),
            Self::GraphOrigin => write!(f, "{}", "<>".black().on_green()),
            Self::GraphDestination => write!(f, "{}", "<>".white().on_green()),
            Self::PathToTest => write!(f, "{}", "##".blue().on_blue()),
            Self::InvalidPath => write!(f, "{}", "##".red().on_red()),
            Self::ValidPath => write!(f, "{}", "##".green().on_green()),
            Self::SegmentLimit => write!(f, "{}", "##".yellow().on_yellow()),
        }
    }
}
