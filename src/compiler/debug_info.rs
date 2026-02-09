use crate::utils::Span;

/*
 * Run-Length Encoded debug information mapping bytecode offsets to source spans
 *
 * Stores runs of (offset, count, span) where:
 * - offset: starting bytecode offset
 * - count: number of consecutive bytes with the same span
 * - span: the source location (line, start_column, end_column)
 *
 * Example: If bytes 0-4 all map to line 1:5-10, we store one entry instead of 5.
 *
 * -- Entry method: `.add_span()` to record, `.get_span()` to retrieve
 * -- Compression: Automatic RLE compression for consecutive identical spans
 * */
#[derive(Debug, Clone, Default)]
pub struct DebugInfo {
    /// RLE-compressed span entries: (start_offset, run_length, span)
    runs: Vec<(usize, usize, Span)>,
}

impl DebugInfo {
    /*
     * Creates a new empty DebugInfo
     *
     * -- Arguments: [none]
     * -- Return value: DebugInfo instance
     * */
    pub fn new() -> Self {
        Self::default()
    }

    /*
     * Add a span for a bytecode offset
     *
     * Automatically compresses consecutive identical spans into runs.
     * If the offset is consecutive and has the same span as the last run,
     * it extends the run instead of creating a new entry.
     *
     * -- Arguments: [&mut self],
     *               offset - bytecode offset
     *               span - source location
     * -- Return value: void
     *
     * -- Example:
     * # add_span(0, Span{1,0,10})
     * # add_span(1, Span{1,0,10})
     * # add_span(2, Span{1,0,10})
     * # Results in one run: (0, 3, Span{1,0,10})
     * */
    pub fn add_span(&mut self, offset: usize, span: Span) {
        // Check if we can extend the last run
        if let Some((start_offset, run_length, last_span)) = self.runs.last_mut() {
            let expected_offset = *start_offset + *run_length;

            // If this offset is consecutive and has the same span, extend the run
            if offset == expected_offset && *last_span == span {
                *run_length += 1;
                return;
            }
        }

        // Otherwise, start a new run
        self.runs.push((offset, 1, span));
    }

    /*
     * Get the span for a given bytecode offset
     *
     * Uses binary search to efficiently find the run containing the offset.
     *
     * -- Arguments: [&self], offset - bytecode offset to look up
     * -- Return value: Span - source location for this offset
     *                  Returns Span::default() if offset not found
     * */
    pub fn get_span(&self, offset: usize) -> Span {
        // Binary search to find the run containing this offset
        match self.runs.binary_search_by(|(start, len, _)| {
            if offset < *start {
                std::cmp::Ordering::Greater
            } else if offset >= *start + *len {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }) {
            Ok(idx) => self.runs[idx].2,
            Err(_) => Span::default(), // Offset not found, return default
        }
    }

    /*
     * Get the number of compressed runs
     *
     * -- Arguments: [&self]
     * -- Return value: usize - number of RLE runs stored
     *
     * -- Notes:
     * # Useful for debugging and measuring compression effectiveness
     * */
    pub fn num_runs(&self) -> usize {
        self.runs.len()
    }

    /*
     * Get compression ratio
     *
     * -- Arguments: [&self]
     * -- Return value: f64 - ratio of original entries to compressed runs
     *
     * -- Notes:
     * # Higher values indicate better compression
     * # Example: 100.0 means 100 entries compressed into 1 run
     * */
    pub fn compression_ratio(&self) -> f64 {
        let total_entries: usize = self.runs.iter().map(|(_, len, _)| len).sum();
        if self.runs.is_empty() {
            1.0
        } else {
            total_entries as f64 / self.runs.len() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_compression() {
        let mut debug_info = DebugInfo::new();
        let span1 = Span {
            line: 1,
            start_column: 0,
            end_column: 10,
        };
        let span2 = Span {
            line: 2,
            start_column: 5,
            end_column: 15,
        };

        // Add 5 consecutive bytes with same span
        for i in 0..5 {
            debug_info.add_span(i, span1);
        }

        // Should compress to 1 run
        assert_eq!(debug_info.num_runs(), 1);
        assert_eq!(debug_info.runs[0], (0, 5, span1));

        // Add 3 bytes with different span
        for i in 5..8 {
            debug_info.add_span(i, span2);
        }

        // Should have 2 runs now
        assert_eq!(debug_info.num_runs(), 2);
        assert_eq!(debug_info.runs[1], (5, 3, span2));

        // Test retrieval
        assert_eq!(debug_info.get_span(2), span1);
        assert_eq!(debug_info.get_span(6), span2);
    }

    #[test]
    fn test_compression_ratio() {
        let mut debug_info = DebugInfo::new();
        let span = Span {
            line: 1,
            start_column: 0,
            end_column: 10,
        };

        // Add 100 consecutive bytes with same span
        for i in 0..100 {
            debug_info.add_span(i, span);
        }

        // Should compress 100 entries into 1 run
        assert_eq!(debug_info.num_runs(), 1);
        assert_eq!(debug_info.compression_ratio(), 100.0);
    }
}
