use super::ShellHandler;
use crate::utils::shell::types::{ModificationType, PathModification, ShellType};
use chrono::Local;
use dirs_next;
use regex::Regex;
use std::path::PathBuf;

pub struct KshHandler {
    config_path: PathBuf,
}

impl KshHandler {
    pub fn new() -> Self {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        Self {
            config_path: home_dir.join(".kshrc"),
        }
    }

    fn get_fallback_paths(&self) -> Vec<PathBuf> {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        vec![home_dir.join(".profile"), home_dir.join(".ksh_profile")]
    }
}

impl ShellHandler for KshHandler {
    fn get_shell_type(&self) -> ShellType {
        ShellType::Ksh
    }

    fn get_config_path(&self) -> PathBuf {
        // Check for fallback paths if .kshrc doesn't exist
        if !self.config_path.exists() {
            for path in self.get_fallback_paths() {
                if path.exists() {
                    return path;
                }
            }
        }
        self.config_path.clone()
    }

    fn parse_path_entries(&self, content: &str) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        let mut seen_paths = std::collections::HashSet::new();
        let export_regex =
            Regex::new(r#"(?:export|typeset -x)\s+PATH=["']?([^"']+)["']?"#).unwrap();

        for line in content.lines() {
            let line = line.trim();

            if let Some(cap) = export_regex.captures(line) {
                if let Some(paths) = cap.get(1) {
                    for path in paths.as_str().split(':') {
                        // Skip variables like $PATH
                        if path.starts_with('$') {
                            continue;
                        }
                        let expanded = shellexpand::tilde(path);
                        let path_buf = PathBuf::from(expanded.to_string());
                        if seen_paths.insert(path_buf.clone()) {
                            entries.push(path_buf);
                        }
                    }
                }
            }
        }

        entries
    }

    fn format_path_export(&self, entries: &[PathBuf]) -> String {
        let paths = entries
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(":");

        format!(
            "\n# Updated by pathmaster on {}\nexport PATH=\"{}\"\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            paths
        )
    }

    fn detect_path_modifications(&self, content: &str) -> Vec<PathModification> {
        let mut modifications = Vec::new();
        let path_regex = Regex::new(r"(export\s+PATH=|typeset\s+-x\s+PATH=)").unwrap();

        for (idx, line) in content.lines().enumerate() {
            if path_regex.is_match(line) {
                modifications.push(PathModification {
                    line_number: idx + 1,
                    content: line.to_string(),
                    modification_type: ModificationType::Assignment,
                });
            }
        }

        modifications
    }

    fn update_path_in_config(&self, content: &str, entries: &[PathBuf]) -> String {
        let modifications = self.detect_path_modifications(content);

        let mut updated_content = content
            .lines()
            .enumerate()
            .filter(|(idx, _)| !modifications.iter().any(|m| m.line_number == idx + 1))
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");

        updated_content.push_str(&self.format_path_export(entries));

        updated_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_ksh_path_handling() {
        let handler = KshHandler::new();
        let content = r#"
# Some config
typeset -x PATH=/usr/local/bin:/usr/bin
export PATH=$PATH:/home/user/bin
"#;

        let entries = handler.parse_path_entries(content);
        println!("Found entries: {:?}", entries); // Add debug output
        assert_eq!(entries.len(), 3, "Expected 3 unique paths");
        assert!(entries.iter().any(|p| p.ends_with("usr/bin")));
        assert!(entries.iter().any(|p| p.ends_with("usr/local/bin")));
        assert!(entries.iter().any(|p| p.ends_with("home/user/bin")));
    }

    #[test]
    fn test_ksh_config_update() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".kshrc");

        let initial_content = r#"
# Initial config
typeset -x PATH=/usr/bin:/old/path
"#;

        fs::write(&config_path, initial_content).unwrap();

        let mut handler = KshHandler::new();
        handler.config_path = config_path.clone();

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        handler.update_config(&new_entries).unwrap();

        let updated_content = fs::read_to_string(&config_path).unwrap();
        assert!(!updated_content.contains("/old/path"));
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
    }
}
