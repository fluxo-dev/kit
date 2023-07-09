//! Custom build file.

use std::error::Error;

/// Main function for the build process.
fn main() -> Result<(), Box<dyn Error>> {
    lalrpop::process_root()
}
