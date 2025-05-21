use std::process::Command;
use crate::services::types::RelevanceScore;

pub async fn compute_relevance_scores(query: &str) -> Result<Vec<RelevanceScore>, String> {
    let output = Command::new("python3")
        .arg("scripts/calculate_relevance.py")
        .arg(query)
        .output()
        .map_err(|e| format!("Python exec error: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}