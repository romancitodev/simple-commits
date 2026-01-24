use crate::{config::cli::AppConfig, errors::AppError, reimpl::Pipeline};
use cliclack::{intro, outro};

// Step modules - each contains a function that works with the Pipeline
pub mod body;
pub mod breaking_change;
pub mod commit;
pub mod content;
pub mod emoji;
pub mod exec;
pub mod message;
pub mod scopes;

/// Initializes and runs the interactive commit pipeline.
///
/// This function orchestrates all the steps required to create a conventional commit:
/// 1. Select commit type (feat, fix, etc.)
/// 2. Select scope (optional)
/// 3. Confirm if breaking change
/// 4. Enter breaking change message (if applicable)
/// 5. Select emoji (optional)
/// 6. Enter commit title
/// 7. Enter commit body/description (optional)
/// 8. Enter footer notes (optional)
/// 9. Execute or preview the commit
///
/// # Arguments
/// * `config` - Mutable reference to the application configuration
///
/// # Returns
/// * `Result<(), AppError>` - Success or error during the pipeline execution
pub fn init(config: AppConfig) -> Result<(), AppError> {
    intro("Simple Commit")?;

    Pipeline::new(config)
        .then(commit::select_commit_type)?
        .then(scopes::select_scope)?
        .if_then(
            breaking_change::ask_breaking_change,
            breaking_change::ask_breaking_message,
        )?
        .then(emoji::select_emoji)?
        .then(message::input_title)?
        .then(body::input_body)?
        .then(content::input_footer)?
        .then(exec::execute_commit)?;

    outro(concat!(
        "In case of issues, please report it to https://github.com/romancitodev/simple-commits\n",
        "\u{2764}  Thanks for use this tool!",
    ))?;

    Ok(())
}
