mod localization;
pub mod misc;
pub mod proc_dir;

#[cfg(feature = "create_report_dialog")]
pub mod create_report_dialog;

#[cfg(feature = "problem_report_dialog")]
pub mod problem_report_dialog;

pub use misc::init;

#[cfg(feature = "with_test")]
pub use misc::init_test;
