#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zoom {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

impl Zoom {
    const ALL: [Zoom; 5] = [
        Zoom::Contain,
        Zoom::Cover,
        Zoom::Fill,
        Zoom::None,
        Zoom::ScaleDown,
    ];
}

impl Default for Zoom {
    fn default() -> Zoom {
        Zoom::Contain
    }
}

impl std::fmt::Display for Zoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Zoom::Contain => "Contain",
                Zoom::Cover => "Cover",
                Zoom::Fill => "Fill",
                Zoom::None => "None",
                Zoom::ScaleDown => "ScaleDown",
            }
        )
    }
}

