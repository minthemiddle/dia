use crate::core::Core;

pub fn handle_review_command(_core: &Core) -> Result<(), Box<dyn std::error::Error>> {
    println!("Spaced repetition review coming soon!");
    Ok(())
}
