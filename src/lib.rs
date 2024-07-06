//! Random checks for things in Nagios. Mostly implemented in the `bins` directory.

use std::collections::HashMap;

pub mod cli;
pub mod logging;

/// Works out if there's any problematic instances in the stats.
pub fn calculate_bad(stats: &HashMap<String, usize>, ok_status: &[&'static str]) -> usize {
    stats
        .iter()
        .filter_map(|(status, count)| {
            if !ok_status.contains(&status.as_str()) {
                Some(count)
            } else {
                None
            }
        })
        .sum()
}
