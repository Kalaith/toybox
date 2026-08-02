//! Best-run records, kept in a slot of their own.
//!
//! A shift ends with a grade and, until this, was immediately forgotten.
//! Nothing carried between runs — tools are earned and lost inside a single
//! shift by design — so a player who cleared 92% of the shop had no way to know
//! next time whether they had beaten it. A score-attack loop with no record is
//! only half a loop.
//!
//! Records live in a separate save slot from the session. The session slot is
//! overwritten by "New Game" every time, which is exactly the moment a record
//! must survive.

use super::{ShiftMode, ShiftSummary};
use serde::{Deserialize, Serialize};

/// What a finished shift is worth, flattened out of `ShiftSummary` so a record
/// stays readable years later without the rest of the session's shape.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ShiftRecord {
    pub toys_shelved: usize,
    pub toy_count: usize,
    pub repairs: u32,
    pub mistakes: u32,
    pub zones_restored: usize,
    pub elapsed_seconds: f32,
    /// Whether the shop was fully restored rather than caught by the clock.
    pub restored: bool,
}

impl ShiftRecord {
    pub fn from_summary(summary: &ShiftSummary, restored: bool) -> Self {
        Self {
            toys_shelved: summary.toys_shelved,
            toy_count: summary.toy_count,
            repairs: summary.repairs,
            mistakes: summary.mistakes,
            zones_restored: summary.zones_restored,
            elapsed_seconds: summary.elapsed_seconds,
            restored,
        }
    }

    /// Is this run better than the one on record?
    ///
    /// Toys shelved first, because that is the job. Then fewer wrong shelves,
    /// then faster — in that order, so a careful slow clear beats a scrappy
    /// quick one, and speed only settles a tie between two equally clean runs.
    /// Deliberately not the grade: the grade is a coarse band and two runs
    /// inside it still have a better and a worse.
    pub fn beats(self, previous: Self) -> bool {
        if self.toys_shelved != previous.toys_shelved {
            return self.toys_shelved > previous.toys_shelved;
        }
        if self.mistakes != previous.mistakes {
            return self.mistakes < previous.mistakes;
        }
        self.elapsed_seconds < previous.elapsed_seconds
    }
}

/// The best run recorded for each way of playing.
///
/// Kept apart rather than pooled: a relaxed run has no deadline, so its time is
/// not comparable with a timed one, and letting an unhurried perfect clear
/// stand as *the* record would leave a Closing Shift player with nothing
/// reachable to chase.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct BestRuns {
    #[serde(default)]
    pub timed: Option<ShiftRecord>,
    #[serde(default)]
    pub relaxed: Option<ShiftRecord>,
}

impl BestRuns {
    pub fn best_for(&self, mode: ShiftMode) -> Option<ShiftRecord> {
        match mode {
            ShiftMode::Timed => self.timed,
            ShiftMode::Relaxed => self.relaxed,
        }
    }

    /// Record `run` if it beats what is stored for `mode`. Returns true when the
    /// record moved, so the score screen can say so.
    pub fn submit(&mut self, mode: ShiftMode, run: ShiftRecord) -> bool {
        let slot = match mode {
            ShiftMode::Timed => &mut self.timed,
            ShiftMode::Relaxed => &mut self.relaxed,
        };
        let improved = slot.is_none_or(|previous| run.beats(previous));
        if improved {
            *slot = Some(run);
        }
        improved
    }
}
