//! src/routes/admin/mod.rs

mod dashboard;
mod logout;
mod newsletter;
mod password;

pub use dashboard::admin_dashboard;
pub use logout::*;
pub use newsletter::*;
pub use password::*;
