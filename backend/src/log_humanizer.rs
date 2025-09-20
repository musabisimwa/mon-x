use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HumanizedLog {
    pub original_message: String,
    pub human_explanation: String,
    pub severity: String,
    pub possible_causes: Vec<String>,
    pub suggested_fixes: Vec<String>,
    pub confidence: f64,
}

pub struct LogHumanizer {
    patterns: HashMap<String, LogPattern>,
    client: Option<reqwest::Client>,
}

#[derive(Clone)]
struct LogPattern {
    regex: Regex,
    explanation: String,
    causes: Vec<String>,
    fixes: Vec<String>,
    severity: String,
}

impl LogHumanizer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Common error patterns
        patterns.insert("connection_refused".to_string(), LogPattern {
            regex: Regex::new(r"(?i)connection.*(refused|failed|timeout)").unwrap(),
            explanation: "Service connection failed - the target service is unreachable".to_string(),
            causes: vec![
                "Service is down or not running".to_string(),
                "Network connectivity issues".to_string(),
                "Firewall blocking connection".to_string(),
                "Wrong port or hostname".to_string(),
            ],
            fixes: vec![
                "Check if target service is running".to_string(),
                "Verify network connectivity with ping/telnet".to_string(),
                "Check firewall rules and port accessibility".to_string(),
                "Validate configuration (hostname, port)".to_string(),
            ],
            severity: "HIGH".to_string(),
        });

        patterns.insert("out_of_memory".to_string(), LogPattern {
            regex: Regex::new(r"(?i)(out.of.memory|oom|memory.*exhausted)").unwrap(),
            explanation: "Application ran out of available memory".to_string(),
            causes: vec![
                "Memory leak in application".to_string(),
                "Insufficient memory allocation".to_string(),
                "High traffic causing memory spike".to_string(),
            ],
            fixes: vec![
                "Increase memory limits/allocation".to_string(),
                "Check for memory leaks in code".to_string(),
                "Implement memory monitoring and alerts".to_string(),
                "Scale horizontally or vertically".to_string(),
            ],
            severity: "CRITICAL".to_string(),
        });

        patterns.insert("database_error".to_string(), LogPattern {
            regex: Regex::new(r"(?i)(database|sql|mysql|postgres|mongo).*(error|failed|timeout)").unwrap(),
            explanation: "Database operation failed".to_string(),
            causes: vec![
                "Database server is down".to_string(),
                "Connection pool exhausted".to_string(),
                "Query timeout or deadlock".to_string(),
                "Invalid SQL syntax".to_string(),
            ],
            fixes: vec![
                "Check database server status".to_string(),
                "Increase connection pool size".to_string(),
                "Optimize slow queries".to_string(),
                "Review and fix SQL syntax".to_string(),
            ],
            severity: "HIGH".to_string(),
        });

        patterns.insert("disk_full".to_string(), LogPattern {
            regex: Regex::new(r"(?i)(disk.*full|no.*space|storage.*full)").unwrap(),
            explanation: "Disk storage is full or nearly full".to_string(),
            causes: vec![
                "Log files consuming too much space".to_string(),
                "Data growth exceeded capacity".to_string(),
                "Temporary files not cleaned up".to_string(),
            ],
            fixes: vec![
                "Clean up old log files and temporary data".to_string(),
                "Implement log rotation".to_string(),
                "Add more storage capacity".to_string(),
                "Set up disk usage monitoring".to_string(),
            ],
            severity: "CRITICAL".to_string(),
        });

        Self {
            patterns,
            client: Some(reqwest::Client::new()),
        }
    }

    pub async fn humanize_log(&self, log_message: &str) -> HumanizedLog {
        // First try pattern matching
        if let Some(result) = self.pattern_match(log_message) {
            return result;
        }

        // Fallback to AI analysis if available
        if let Some(ai_result) = self.ai_analyze(log_message).await {
            return ai_result;
        }

        // Default fallback
        HumanizedLog {
            original_message: log_message.to_string(),
            human_explanation: "Log entry detected - review for potential issues".to_string(),
            severity: "INFO".to_string(),
            possible_causes: vec!["Normal application behavior".to_string()],
            suggested_fixes: vec!["No action required unless part of larger pattern".to_string()],
            confidence: 0.3,
        }
    }

    fn pattern_match(&self, log_message: &str) -> Option<HumanizedLog> {
        for (_, pattern) in &self.patterns {
            if pattern.regex.is_match(log_message) {
                return Some(HumanizedLog {
                    original_message: log_message.to_string(),
                    human_explanation: pattern.explanation.clone(),
                    severity: pattern.severity.clone(),
                    possible_causes: pattern.causes.clone(),
                    suggested_fixes: pattern.fixes.clone(),
                    confidence: 0.9,
                });
            }
        }
        None
    }

    async fn ai_analyze(&self, log_message: &str) -> Option<HumanizedLog> {
        let client = self.client.as_ref()?;
        
        // Try local Ollama/Gemma first
        if let Ok(result) = self.query_local_model(client, log_message).await {
            return Some(result);
        }

        None
    }

    async fn query_local_model(&self, client: &reqwest::Client, log_message: &str) -> Result<HumanizedLog, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Analyze this log message and provide a JSON response with human_explanation, severity (INFO/WARN/HIGH/CRITICAL), possible_causes (array), and suggested_fixes (array):\n\n{}",
            log_message
        );

        let payload = serde_json::json!({
            "model": "gemma2:2b",
            "prompt": prompt,
            "stream": false,
            "format": "json"
        });

        let response = client
            .post("http://localhost:11434/api/generate")
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(response_text) = result["response"].as_str() {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response_text) {
                    return Ok(HumanizedLog {
                        original_message: log_message.to_string(),
                        human_explanation: parsed["human_explanation"].as_str().unwrap_or("AI analysis completed").to_string(),
                        severity: parsed["severity"].as_str().unwrap_or("INFO").to_string(),
                        possible_causes: parsed["possible_causes"].as_array()
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                            .unwrap_or_else(|| vec!["Unknown cause".to_string()]),
                        suggested_fixes: parsed["suggested_fixes"].as_array()
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                            .unwrap_or_else(|| vec!["Review logs for patterns".to_string()]),
                        confidence: 0.8,
                    });
                }
            }
        }

        Err("AI analysis failed".into())
    }
}

// Global instance
use std::sync::LazyLock;
static HUMANIZER: LazyLock<LogHumanizer> = LazyLock::new(|| LogHumanizer::new());

pub async fn humanize_log_message(log_message: &str) -> HumanizedLog {
    HUMANIZER.humanize_log(log_message).await
}
