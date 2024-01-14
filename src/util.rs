#[cfg(test)]
pub fn generate_unique_value() -> String {
    use uuid::Uuid;

    format!("Test_{}", Uuid::new_v4())
}