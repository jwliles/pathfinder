use super::ShellHandler;
use crate::utils::shell::types::{ModificationType, PathModification, ShellType};
use chrono::Local;
use dirs_next;
use regex::Regex;
use std::path::PathBuf;

pub struct GenericHandler {
    config_path: PathBuf,
}

impl GenericHandler {
    pub fn new() -> Self {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        Self {
            config_path: home_dir.join(".profile"),
        }
    }
}

impl ShellHandler for GenericHandler {
    fn get_shell_type(&self) -> ShellType {
        ShellType::Generic
    }

    fn get_config_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    fn parse_path_entries(&self, content: &str) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        let export_regex = Regex::new(r#"export\s+PATH=["']?([^"']+)["']?"#).unwrap();

        for line in content.lines() {
            if let Some(cap) = export_regex.captures(line.trim()) {
                if let Some(paths) = cap.get(1) {
                    for path in paths.as_str().split(':') {
                        let expanded = shellexpand::tilde(path);
                        entries.push(PathBuf::from(expanded.to_string()));
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
        let path_regex = Regex::new(r"(?:export\s+)?PATH=").unwrap();

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
mod generic_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_generic_path_parsing() {
        let handler = GenericHandler::new();
        let content = r#"
# Some config
PATH=/usr/bin:/usr/local/bin
export PATH=/usr/bin:/home/user/bin
"#;

        let entries = handler.parse_path_entries(content);
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().any(|p| p.ends_with("usr/bin")));
        assert!(entries.iter().any(|p| p.ends_with("home/user/bin")));
    }

    #[test]
    fn test_generic_config_update() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".profile");

        let initial_content = r#"
# Initial config
PATH=/usr/bin:/old/path
export PATH=/usr/bin:/another/old/path
"#;

        fs::write(&config_path, initial_content).unwrap();

        let mut handler = GenericHandler::new();
        handler.config_path = config_path.clone();

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        handler.update_config(&new_entries).unwrap();

        let updated_content = fs::read_to_string(&config_path).unwrap();
        assert!(!updated_content.contains("/old/path"));
        assert!(updated_content.contains("export PATH="));
        assert!(updated_content.contains("/usr/local/bin"));
    }
}
