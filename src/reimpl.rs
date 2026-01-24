//! Pipeline pattern for composing commit creation steps.
//!
//! This module provides a functional, composable approach to building commits
//! through a series of steps. Each step receives mutable access to the pipeline
//! and can modify the state and configuration as needed.
//!
//! # Example
//!
//! ```rust,ignore
//! Pipeline::new(config)
//!     .then(|pipeline| {
//!         // Step 1: Do something with pipeline.state and pipeline.config
//!         Ok(())
//!     })?
//!     .then(|pipeline| {
//!         // Step 2: Next operation
//!         Ok(())
//!     })?;
//! ```
//!
//! The pipeline pattern provides:
//! - **Explicit data flow**: State and config are passed through each step
//! - **Error propagation**: Each step returns `Result<(), AppError>`
//! - **Composability**: Steps can be chained with `.then()`
//! - **Type safety**: The compiler ensures proper error handling

use crate::{config::cli::AppConfig, errors::AppError, tui::AppData};

/// The pipeline that carries state and configuration through commit creation steps.
///
/// Each step in the pipeline receives mutable access to this struct and can:
/// - Read and modify the application state (`state`)
/// - Read and modify the configuration (`config`)
/// - Return an error to halt the pipeline
pub struct Pipeline {
    /// The application state, primarily containing the commit being built.
    pub state: AppData,
    /// The application configuration, including user preferences and settings.
    pub config: AppConfig,
}

impl Pipeline {
    /// Creates a new pipeline with the given configuration.
    ///
    /// The state is initialized to its default value.
    pub fn new(config: AppConfig) -> Self {
        Self {
            state: AppData::default(),
            config,
        }
    }
}

impl Pipeline {
    /// Chains a step onto the pipeline.
    ///
    /// The provided closure receives mutable access to the pipeline and can modify
    /// both the state and configuration. If the closure returns an error, the pipeline
    /// stops and the error is propagated.
    ///
    /// # Arguments
    /// * `run` - A closure that performs the step's logic
    ///
    /// # Returns
    /// * `Ok(Pipeline)` - The pipeline to continue chaining
    /// * `Err(AppError)` - If the step encountered an error
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// pipeline.then(|p| {
    ///     p.state.commit.set_title(Some("feat: add feature".to_string()));
    ///     Ok(())
    /// })?
    /// ```
    pub fn then<F>(mut self, run: F) -> Result<Pipeline, AppError>
    where
        F: FnOnce(&mut Self) -> Result<(), AppError>,
    {
        run(&mut self)?;
        Ok(self)
    }

    pub fn if_then<F, D>(mut self, cond: D, run: F) -> Result<Pipeline, AppError>
    where
        D: FnOnce(&mut Self) -> Result<bool, AppError>,
        F: FnOnce(&mut Self) -> Result<(), AppError>,
    {
        if cond(&mut self)? {
            run(&mut self)?;
        }
        Ok(self)
    }
}
