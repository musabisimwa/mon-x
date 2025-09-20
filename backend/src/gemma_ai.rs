use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyContext {
    pub logs: Vec<String>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub error_count: u32,
    pub health_status: String,
    pub agent_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIInsight {
    pub analysis: String,
    pub severity: String,
    pub root_cause: String,
    pub suggested_fixes: Vec<String>,
    pub confidence: f64,
}

pub struct GemmaAI {
    model_path: Option<String>,
    initialized: bool,
}

static mut GEMMA_INSTANCE: Option<Mutex<GemmaAI>> = None;

impl GemmaAI {
    pub fn new() -> Self {
        Self { 
            model_path: None,
            initialized: false 
        }
    }

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!(" Initializing GGUF inference engine...");
        
        // Check for llama.cpp binary
        if let Ok(_) = Command::new("llama-cli").arg("--version").output() {
            println!("Found llama-cli binary");
        } else if let Ok(_) = Command::new("./llama.cpp/llama-cli").arg("--version").output() {
            println!("Found local llama-cli binary");
        } else {
            println!("llama-cli not found, will use enhanced fallback");
        }
        
        let model_dir = "src/model/";
        if let Ok(entries) = std::fs::read_dir(model_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("gguf") {
                        println!(" Found GGUF model: {:?}", path);
                        self.model_path = Some(path.to_string_lossy().to_string());
                        self.initialized = true;
                        println!("GGUF model ready for inference");
                        return Ok(());
                    }
                }
            }
        }
        
        println!("AI analysis engine ready (enhanced fallback mode)");
        self.initialized = true;
        Ok(())
    }

    pub fn analyze_anomaly(&self, context: &AnomalyContext) -> AIInsight {
        if let Some(ref model_path) = self.model_path {
            match self.llm_analysis(model_path, context) {
                Ok(insight) => insight,
                Err(e) => {
                    println!("GGUF inference failed: {}, using fallback", e);
                    self.enhanced_fallback_analysis(context)
                }
            }
        } else {
            self.enhanced_fallback_analysis(context)
        }
    }

    fn llm_analysis(&self, model_path: &str, context: &AnomalyContext) -> Result<AIInsight, Box<dyn std::error::Error>> {
        let prompt = self.build_analysis_prompt(context);
        
        // Try llama-cli first
        let output = Command::new("llama-cli")
            .arg("-m").arg(model_path)
            .arg("-p").arg(&prompt)
            .arg("-n").arg("150")
            .arg("--temp").arg("0.7")
            .arg("--top-p").arg("0.9")
            .output()
            .or_else(|_| {
                // Fallback to local binary
                Command::new("./llama.cpp/llama-cli")
                    .arg("-m").arg(model_path)
                    .arg("-p").arg(&prompt)
                    .arg("-n").arg("150")
                    .output()
            })?;

        if output.status.success() {
            let response = String::from_utf8_lossy(&output.stdout);
            Ok(self.parse_llm_response(&response, context))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("llama-cli failed: {}", error).into())
        }
    }

    fn build_analysis_prompt(&self, context: &AnomalyContext) -> String {
        format!(
            "System Monitor AI Analysis:

Application: {}
CPU: {:.1}%
Memory: {:.1}%
Errors: {}
Health: {}
Logs: {}

Analyze this system state and provide structured response:
SEVERITY: [LOW/HIGH/CRITICAL]
ANALYSIS: [brief technical analysis]
ROOT_CAUSE: [primary issue identified]
FIXES: [fix1] | [fix2] | [fix3]
CONFIDENCE: [0.0-1.0]

Response:",
            context.agent_id,
            context.cpu_usage,
            context.memory_usage,
            context.error_count,
            context.health_status,
            context.logs.join("; ")
        )
    }

    fn parse_llm_response(&self, response: &str, context: &AnomalyContext) -> AIInsight {
        let mut severity = "LOW".to_string();
        let mut analysis = String::new();
        let mut root_cause = String::new();
        let mut fixes = Vec::new();
        let mut confidence = 0.8;

        for line in response.lines() {
            let line = line.trim();
            if line.starts_with("SEVERITY:") {
                severity = line.replace("SEVERITY:", "").trim().to_string();
            } else if line.starts_with("ANALYSIS:") {
                analysis = line.replace("ANALYSIS:", "").trim().to_string();
            } else if line.starts_with("ROOT_CAUSE:") {
                root_cause = line.replace("ROOT_CAUSE:", "").trim().to_string();
            } else if line.starts_with("FIXES:") {
                let fixes_line = line.replace("FIXES:", "");
                let fixes_str = fixes_line.trim();
                fixes = fixes_str.split(" | ").map(|s| s.trim().to_string()).collect();
            } else if line.starts_with("CONFIDENCE:") {
                if let Ok(conf) = line.replace("CONFIDENCE:", "").trim().parse::<f64>() {
                    confidence = conf;
                }
            }
        }

        if analysis.is_empty() {
            return self.enhanced_fallback_analysis(context);
        }

        AIInsight {
            analysis: format!("GGUF Model: {}", analysis),
            severity,
            root_cause,
            suggested_fixes: fixes,
            confidence,
        }
    }

    fn enhanced_fallback_analysis(&self, context: &AnomalyContext) -> AIInsight {
        let critical_score = self.calculate_critical_score(context);
        let log_patterns = self.analyze_log_patterns(&context.logs);
        
        let (severity, analysis, root_cause, fixes, confidence) = match critical_score {
            4..=5 => (
                "CRITICAL",
                format!(" CRITICAL: Agent {} cascading failures - CPU {:.1}%, Memory {:.1}%, {} errors. {}", 
                    context.agent_id, context.cpu_usage, context.memory_usage, context.error_count, log_patterns),
                "Complete system resource exhaustion with cascading failures".to_string(),
                vec![
                    " IMMEDIATE: Emergency restart required".to_string(),
                    " IMMEDIATE: Kill runaway processes".to_string(),
                    " URGENT: Implement auto-scaling".to_string(),
                ],
                0.98
            ),
            2..=3 => (
                "HIGH",
                format!("HIGH RISK: Agent {} significant stress - performance degradation imminent. {}", 
                    context.agent_id, log_patterns),
                "High resource utilization causing application instability".to_string(),
                vec![
                    "PRIORITY: Scale resources immediately".to_string(),
                    "NVESTIGATE: Database performance".to_string(),
                    "IMPLEMENT: Circuit breakers".to_string(),
                ],
                0.89
            ),
            _ => (
                "LOW",
                format!("Agent {} stable with minor anomalies. {}", 
                    context.agent_id, if log_patterns.is_empty() { "No issues detected.".to_string() } else { log_patterns }),
                "Normal operational variations".to_string(),
                vec![
                    "ROUTINE: Continue monitoring".to_string(),
                    "REVIEW: Recent changes".to_string(),
                ],
                0.75
            )
        };

        AIInsight {
            analysis,
            severity: severity.to_string(),
            root_cause,
            suggested_fixes: fixes,
            confidence,
        }
    }

    fn calculate_critical_score(&self, context: &AnomalyContext) -> u32 {
        let mut score = 0;
        if context.cpu_usage > 90.0 { score += 1; }
        if context.memory_usage > 85.0 { score += 1; }
        if context.error_count > 15 { score += 1; }
        if context.health_status != "healthy" { score += 1; }
        if context.logs.iter().any(|log| log.contains("FATAL") || log.contains("OutOfMemory")) { score += 1; }
        score
    }

    fn analyze_log_patterns(&self, logs: &[String]) -> String {
        let error_logs: Vec<&String> = logs.iter()
            .filter(|log| log.to_lowercase().contains("error"))
            .collect();

        if error_logs.len() > 3 {
            let mut patterns = Vec::new();
            if error_logs.iter().any(|log| log.contains("connection")) {
                patterns.push("Connection issues");
            }
            if error_logs.iter().any(|log| log.contains("memory")) {
                patterns.push("Memory problems");
            }
            if !patterns.is_empty() {
                format!("Patterns: {}", patterns.join(", "))
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

pub fn get_gemma_instance() -> &'static Mutex<GemmaAI> {
    unsafe {
        GEMMA_INSTANCE.get_or_insert_with(|| Mutex::new(GemmaAI::new()))
    }
}

pub async fn initialize_gemma() -> Result<(), Box<dyn std::error::Error>> {
    let instance = get_gemma_instance();
    let mut gemma = instance.lock().unwrap();
    gemma.initialize().await
}

pub async fn analyze_system_anomaly(context: AnomalyContext) -> AIInsight {
    let instance = get_gemma_instance();
    let analysis = {
        let gemma = instance.lock().unwrap();
        gemma.analyze_anomaly(&context)
    };
    analysis
}
