use crate::cli::theme::Theme;
use colored::*;

pub struct ProgressBar {
    width: usize,
    show_percentage: bool,
}

impl ProgressBar {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            show_percentage: true,
        }
    }

    pub fn render(&self, progress: f32) -> String {
        let progress = progress.clamp(0.0, 1.0);
        let filled_width = (self.width as f32 * progress) as usize;

        let mut bar = String::new();

        // Opening bracket
        bar.push_str(&Theme::BRACKET_OPEN.white().dimmed().to_string());

        // Progress visualization
        if filled_width == 0 {
            bar.push_str(
                &Theme::PROGRESS_EMPTY
                    .repeat(self.width)
                    .white()
                    .dimmed()
                    .to_string(),
            );
        } else if filled_width >= self.width {
            // Completely filled
            bar.push_str(
                &Theme::PROGRESS_FILLED
                    .repeat(self.width)
                    .color(Theme::HIGHLIGHT)
                    .to_string(),
            );
        } else {
            // Partially filled with arrow
            let arrow_len = 2; // ">>"
            let trail_len = 3.min(filled_width.saturating_sub(arrow_len)); // "===" trail
            let filled_len = filled_width.saturating_sub(trail_len + arrow_len);

            // Filled portion (--------)
            if filled_len > 0 {
                bar.push_str(
                    &Theme::PROGRESS_FILLED
                        .repeat(filled_len)
                        .color(Theme::HIGHLIGHT)
                        .to_string(),
                );
            }

            // Trail portion (===)
            if trail_len > 0 {
                bar.push_str(
                    &Theme::PROGRESS_TRAIL
                        .repeat(trail_len)
                        .color(Theme::IN_PROGRESS)
                        .to_string(),
                );
            }

            // Arrow (>>)
            bar.push_str(
                &Theme::PROGRESS_ARROW
                    .color(Theme::IN_PROGRESS)
                    .bold()
                    .to_string(),
            );

            // Empty portion (•••••••••••••••••••)
            let empty_len = self.width - filled_width;
            if empty_len > 0 {
                bar.push_str(
                    &Theme::PROGRESS_EMPTY
                        .repeat(empty_len)
                        .white()
                        .dimmed()
                        .to_string(),
                );
            }
        }

        // Closing bracket
        bar.push_str(&Theme::BRACKET_CLOSE.white().dimmed().to_string());

        if self.show_percentage {
            let percentage = format!(" {}%", (progress * 100.0) as u8);
            bar.push_str(&percentage.white().dimmed().to_string());
        }

        bar
    }
}
