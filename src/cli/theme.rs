use colored::Color;

pub struct Theme;

impl Theme {
    // Main colors matching error handler
    pub const HIGHLIGHT: Color = Color::Cyan;
    pub const SUCCESS: Color = Color::Green;
    pub const IN_PROGRESS: Color = Color::Yellow;
    pub const ERROR: Color = Color::Red;
    pub const TEXT: Color = Color::BrightWhite;
    pub const DIMMED: Color = Color::White;

    // Progress bar components
    pub const PROGRESS_FILLED: &'static str = "-";
    pub const PROGRESS_ARROW: &'static str = ">>";
    pub const PROGRESS_TRAIL: &'static str = "===";
    pub const PROGRESS_EMPTY: &'static str = "•";

    // Phase indicators
    pub const PHASE_IN_PROGRESS: &'static str = "•";
    pub const PHASE_COMPLETE: &'static str = "✓";

    // Bracket style
    pub const BRACKET_OPEN: &'static str = "[";
    pub const BRACKET_CLOSE: &'static str = "]";
}
