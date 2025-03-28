use std::io::Result;
use std::io::Stderr;
use std::io::Write;

use tracing_subscriber::filter::Directive;
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

use crate::utils::progress::GLOBAL_PROGRESS_MANAGER;

/// Initializes the logger with the specified directive and environment variable.
///
/// # Arguments
///
/// * `directive` - A logging directive that controls the log level and filtering rules.
/// * `env_var` - The environment variable used to override log filtering rules.
pub fn initialize_logger(directive: impl Into<Directive>, env_var: impl Into<String>) {
    fmt()
        .with_env_filter(
            EnvFilter::builder().with_default_directive(directive.into()).with_env_var(env_var.into()).from_env_lossy(),
        )
        .with_writer(LoggerWriter::stderr)
        .with_target(cfg!(debug_assertions))
        .without_time()
        .compact()
        .init();
}

/// A writer that allows feedback output to be redirected to the specified writer,
/// supporting logging while suspending progress bars for clear output.
struct LoggerWriter<W: Write> {
    writer: W,
}

impl LoggerWriter<Stderr> {
    /// Creates a new `LoggerWriter` that writes to standard error (`stderr`).
    pub fn stderr() -> Self {
        Self { writer: std::io::stderr() }
    }
}

impl<W: Write> Write for LoggerWriter<W> {
    /// Writes a buffer to the internal writer, suspending the progress bars during the operation.
    ///
    /// # Arguments
    ///
    /// * `buf` - The data to write.
    ///
    /// # Returns
    ///
    /// The number of bytes written.
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        GLOBAL_PROGRESS_MANAGER.suspend(|| self.writer.write(buf))
    }

    /// Flushes the internal writer, ensuring all data is written out, while suspending the progress bars.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    fn flush(&mut self) -> Result<()> {
        GLOBAL_PROGRESS_MANAGER.suspend(|| self.writer.flush())
    }
}
