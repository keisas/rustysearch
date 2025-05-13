use crate::models::Book;
use std::process::Command;

pub async fn compute_relevance_scores(books: &[Book]) -> Result<Vec<f32>, String> {
    let input_json = serde_json::to_string(books).map_err(|e| e.to_string())?;

    let output = Command::new("python3")
        .arg("scripts/calculate_relevance.py")
        .arg(input_json)
        .output()
        .map_err(|e| format!("Python exec error: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
