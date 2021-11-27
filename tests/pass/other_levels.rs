#[log_attributes::trace("called {fn}")]
fn trace() {}

#[log_attributes::debug("called {fn}")]
fn debug() {}

#[log_attributes::info("called {fn}")]
fn info() {}

#[log_attributes::warn("called {fn}")]
fn warn() {}

#[log_attributes::error("called {fn}")]
fn error() {}

fn main() {}
