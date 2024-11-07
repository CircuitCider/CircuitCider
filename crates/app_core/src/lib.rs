pub mod plugins;

pub const ROOT: &str = "root";
pub const SHADERS: &str = "shaders";

/// Location where the app is being located from. I.E: a subcrate or from main.
pub enum ExecLocation {
    /// This app is being executed from a sub-crate of the project(I.E, in /crates/<crate_name>)
    CRATE,
    /// This app is being executed from main.rs
    MAIN,
}
