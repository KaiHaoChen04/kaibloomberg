

pub enum BoxLocation{
    TopLeft,
    TopCentre,
    TopRight,
    MiddleLeft,
    MiddleCentre,
    MiddleRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

impl BoxLocation {
    pub fn get_index() -> (usize, usize) {
        match self {
            Self::TopLeft => (0,0),
            Self::TopCentre => (0,1),
            Self::TopRight => (0,2),
            Self::MiddleLeft => (1,0),
            Self::MiddleCentre => (1,1),
            Self::MiddleRight => (1,2),
            Self::BottomLeft => (2,0),
            Self::BottomCentre => (2,1),
            Self::BottomRight => (2,2),
        }
    }
}