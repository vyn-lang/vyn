use crate::utils::Span;

/// Run-length encoded debug information
#[derive(Default, Debug)]
pub struct DebugInfo {
    pub line_changes: Vec<(usize, u32)>,
    pub start_col_changes: Vec<(usize, u32)>,
    pub end_col_changes: Vec<(usize, u32)>,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            line_changes: Vec::new(),
            start_col_changes: Vec::new(),
            end_col_changes: Vec::new(),
        }
    }

    pub fn get_span(&self, ip: usize) -> Span {
        let line = self.find_value(&self.line_changes, ip);
        let start_column = self.find_value(&self.start_col_changes, ip);
        let end_column = self.find_value(&self.end_col_changes, ip);

        Span {
            line,
            start_column,
            end_column,
        }
    }

    fn find_value(&self, changes: &Vec<(usize, u32)>, ip: usize) -> u32 {
        if changes.is_empty() {
            return 0;
        }

        let idx = changes
            .binary_search_by_key(&ip, |&(offset, _)| offset)
            .unwrap_or_else(|i| i.saturating_sub(1));

        changes[idx].1
    }
}
