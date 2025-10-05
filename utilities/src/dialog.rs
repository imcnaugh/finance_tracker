use dialoguer::Confirm;

/// Prompt the user to confirm an action.
/// Returns true if the user confirms (answers yes/y), false otherwise.
pub fn prompt_confirm(prompt: &str) -> bool {
    Confirm::new()
        .with_prompt(prompt)
        .default(false)
        .interact()
        .unwrap_or(false)
}
