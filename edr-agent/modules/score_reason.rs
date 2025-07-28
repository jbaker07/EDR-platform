use std::collections::HashMap;

#[derive(Debug)]
pub enum ScoreCategory {
    RemoteShellExecution,
    ProcessInjection,
    SuspiciousNetwork,
    MaliciousUSB,
    PrivilegeEscalation,
    CronInlineShell,
    ContainerCurlScript,
    RootLogon,
    Generic,
}

#[derive(Debug)]
pub struct ScoreReason {
    pub category: ScoreCategory,
    pub message: String,
}

/// Score reason builder from SHAP + GNN + tags (used by trust_hook.rs)
pub fn build_score_reason(shap: f64, gnn: f64, tags: &Vec<String>) -> String {
    let mut parts = Vec::new();

    if shap > 0.5 {
        parts.push(format!("High SHAP score ({:.2}) indicates behavioral anomaly", shap));
    }

    if gnn > 0.5 {
        parts.push(format!("GNN model suggests elevated structural threat (score: {:.2})", gnn));
    }

    if !tags.is_empty() {
        parts.push(format!("Ontology tags triggered: [{}]", tags.join(", ")));
    }

    if parts.is_empty() {
        "No elevated indicators detected.".into()
    } else {
        parts.join(" | ")
    }
}

pub fn get_score_reason(context: &str, metadata: &HashMap<String, String>) -> ScoreReason {
    match context {
        "process_monitor" => {
            if let Some(cmd) = metadata.get("command_line") {
                if cmd.contains("curl") && (cmd.contains("sh") || cmd.contains("|")) {
                    return ScoreReason {
                        category: ScoreCategory::RemoteShellExecution,
                        message: "Command-line execution of remote shell detected.".into(),
                    };
                } else if cmd.contains("--inject") || cmd.contains("reflective") || cmd.contains("shellcode") {
                    return ScoreReason {
                        category: ScoreCategory::ProcessInjection,
                        message: "Potential process injection behavior.".into(),
                    };
                }
            }
            ScoreReason {
                category: ScoreCategory::Generic,
                message: "Unusual process behavior detected.".into(),
            }
        }

        "network_monitor" => {
            if let Some(ip) = metadata.get("destination_ip") {
                if ip.starts_with("185.") || ip.ends_with(".onion") || ip.contains("darknet") {
                    return ScoreReason {
                        category: ScoreCategory::SuspiciousNetwork,
                        message: "Outbound traffic to suspicious or Tor-based address.".into(),
                    };
                }
            }
            ScoreReason {
                category: ScoreCategory::Generic,
                message: "Suspicious outbound network activity.".into(),
            }
        }

        "usb_monitor" => {
            if let Some(vid) = metadata.get("vendor_id") {
                if vid == "1234" || vid.to_lowercase().contains("badusb") {
                    return ScoreReason {
                        category: ScoreCategory::MaliciousUSB,
                        message: "Potential malicious USB device detected.".into(),
                    };
                }
            }
            ScoreReason {
                category: ScoreCategory::Generic,
                message: "Unexpected USB activity.".into(),
            }
        }

        "privilege_monitor" => {
            if let Some(cmd) = metadata.get("sudo_command") {
                if cmd.contains("passwd") || cmd.contains("adduser") || cmd.contains("usermod") {
                    return ScoreReason {
                        category: ScoreCategory::PrivilegeEscalation,
                        message: "Privileged command attempting user or password modification.".into(),
                    };
                }
            }
            ScoreReason {
                category: ScoreCategory::Generic,
                message: "Sudo privilege escalation attempt.".into(),
            }
        }

        "job_sched_monitor" => {
            if let Some(entry) = metadata.get("cron_entry") {
                if entry.contains("bash -c") || entry.contains("wget") {
                    return ScoreReason {
                        category: ScoreCategory::CronInlineShell,
                        message: "Cron job executing inline shell command.".into(),
                    };
                }
            }
            ScoreReason {
                category: ScoreCategory::Generic,
                message: "Unusual scheduled task entry detected.".into(),
            }
        }

        "container_monitor" => {
            if let Some(cmd) = metadata.get("command") {
                if cmd.contains("curl") && cmd.contains("| sh") {
                    return ScoreReason {
                        category: ScoreCategory::ContainerCurlScript,
                        message: "Container spawned with command fetching external script.".into(),
                    };
                }
            }
            ScoreReason {
                category: ScoreCategory::Generic,
                message: "Container launched with non-standard execution pattern.".into(),
            }
        }

        "logon_tracker" => {
            if let Some(user) = metadata.get("user") {
                if user == "root" {
                    return ScoreReason {
                        category: ScoreCategory::RootLogon,
                        message: "Root user session opened unexpectedly.".into(),
                    };
                }
            }
            ScoreReason {
                category: ScoreCategory::Generic,
                message: "User logon activity tracked.".into(),
            }
        }

        _ => ScoreReason {
            category: ScoreCategory::Generic,
            message: "Generic suspicious activity detected.".into(),
        },
    }
}
