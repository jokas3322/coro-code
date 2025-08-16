//! Execution trajectory recording and replay

pub mod entry;
pub mod recorder;

pub use entry::{EntryType, TrajectoryEntry};
pub use recorder::TrajectoryRecorder;
