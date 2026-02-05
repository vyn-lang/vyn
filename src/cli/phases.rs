use crate::cli::theme::Theme;
use colored::*;
use indicatif::{ProgressBar as IndicatifBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Tokenizing,
    Parsing,
    StaticEvaluation,
    TypeChecking,
    IRBuilding,
    Compiling,
}

impl Phase {
    fn name(&self) -> &'static str {
        match self {
            Phase::Tokenizing => "Tokenizing",
            Phase::Parsing => "Parsing",
            Phase::StaticEvaluation => "Evaluating Statics",
            Phase::TypeChecking => "Type Checking",
            Phase::IRBuilding => "IR Building",
            Phase::Compiling => "Compiling",
        }
    }

    fn progress_start(&self) -> u64 {
        match self {
            Phase::Tokenizing => 0,
            Phase::Parsing => 16,
            Phase::StaticEvaluation => 32,
            Phase::TypeChecking => 48,
            Phase::IRBuilding => 64,
            Phase::Compiling => 80,
        }
    }

    fn progress_end(&self) -> u64 {
        match self {
            Phase::Tokenizing => 16,
            Phase::Parsing => 32,
            Phase::StaticEvaluation => 48,
            Phase::TypeChecking => 64,
            Phase::IRBuilding => 80,
            Phase::Compiling => 100,
        }
    }
}

pub struct PhaseTracker {
    show_timing: bool,
    quiet: bool,
    slow_mode: bool,
    phase_start: Option<Instant>,
    progress_bar: Option<IndicatifBar>,
}

impl PhaseTracker {
    pub fn new(
        file_name: String,
        show_progress: bool,
        show_timing: bool,
        quiet: bool,
        slow_mode: bool,
    ) -> Self {
        let progress_bar = if show_progress && !quiet {
            let pb = IndicatifBar::new(100);

            let style = ProgressStyle::default_bar()
                .template("{msg}\nCompiling {prefix} [{bar:40.cyan/blue}] {pos}%")
                .unwrap()
                .progress_chars("->=•");

            pb.set_style(style);
            pb.set_prefix(file_name.clone().cyan().to_string());
            Some(pb)
        } else {
            None
        };

        Self {
            show_timing,
            quiet,
            slow_mode,
            phase_start: None,
            progress_bar,
        }
    }

    pub fn start(&mut self) {
        if let Some(pb) = &self.progress_bar {
            pb.set_position(0);
        }
    }

    pub fn begin_phase(&mut self, phase: Phase) {
        if self.quiet {
            return;
        }

        self.phase_start = Some(Instant::now());

        if let Some(pb) = &self.progress_bar {
            let msg = format!(
                "{} {}",
                Theme::PHASE_IN_PROGRESS.color(Theme::IN_PROGRESS),
                phase.name().bright_white()
            );
            pb.set_message(msg);
            pb.set_position(phase.progress_start());
        } else {
            print!(
                "{} {}",
                Theme::PHASE_IN_PROGRESS.color(Theme::IN_PROGRESS),
                phase.name().bright_white()
            );
        }
    }

    pub fn complete_phase(&mut self, phase: Phase) {
        if self.quiet {
            return;
        }

        if let Some(pb) = &self.progress_bar {
            pb.set_position(phase.progress_end());

            if self.show_timing {
                if let Some(start) = self.phase_start {
                    let elapsed = start.elapsed();
                    let msg = format!(
                        "{} {} {}",
                        Theme::PHASE_COMPLETE.color(Theme::SUCCESS),
                        phase.name().bright_white(),
                        format!("({:.2}s)", elapsed.as_secs_f64()).white().dimmed()
                    );
                    pb.set_message(msg);
                }
            }
        } else if self.show_timing {
            if let Some(start) = self.phase_start {
                let elapsed = start.elapsed();
                println!(
                    " {}",
                    format!("({:.2}s)", elapsed.as_secs_f64()).white().dimmed()
                );
            }
        }

        if self.slow_mode {
            thread::sleep(Duration::from_millis(800));
        }
    }

    pub fn finish(&mut self) {
        if self.quiet {
            return;
        }

        if let Some(pb) = &self.progress_bar {
            pb.finish_and_clear();
        }

        println!(
            "{} {}",
            Theme::PHASE_COMPLETE.color(Theme::SUCCESS).bold(),
            "Compilation Complete".bright_white().bold()
        );
    }

    pub fn clear_display(&mut self) {
        if let Some(pb) = &self.progress_bar {
            pb.finish_and_clear();
        }
    }
}
