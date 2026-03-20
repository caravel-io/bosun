use std::path::Path;

use crate::filesystem::slurp;

use crate::Collector;
use anyhow::Result;
use serde::Serialize;
use serde_json::to_value;

#[derive(Serialize, Debug)]
pub struct OSFacts {
    pub pretty_name: Option<String>,
    pub name: Option<String>,
    pub version_id: Option<String>,
    pub version: Option<String>,
    pub codename: Option<String>,
    pub id: Option<String>,
    pub home_url: Option<String>,
}

pub struct OSComponent;

impl OSComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Collector for OSComponent {
    fn name(&self) -> &'static str {
        "os"
    }

    fn collect(&self) -> Result<serde_json::Value> {
        let lines = get_os_lines()?;
        let of = parse_into_facts(lines)?;
        Ok(to_value(of)?)
    }
}

fn get_os_lines() -> Result<Vec<String>> {
    Ok(slurp(Path::new("/etc/os-release"))?
        .lines()
        .map(|s| s.to_string())
        .collect())
}

fn parse_into_facts(lines: Vec<String>) -> Result<OSFacts> {
    let mut pretty_name = None;
    let mut name = None;
    let mut version_id = None;
    let mut version = None;
    let mut codename = None;
    let mut id = None;
    let mut home_url = None;

    for line in lines {
        let Some((k, v)) = line.split_once("=") else {
            continue;
        };
        match k {
            "PRETTY_NAME" => pretty_name = Some(v.trim_matches('"').to_string()),
            "NAME" => name = Some(v.trim_matches('"').to_string()),
            "VERSION_ID" => version_id = Some(v.trim_matches('"').to_string()),
            "VERSION" => version = Some(v.trim_matches('"').to_string()),
            "VERSION_CODENAME" => codename = Some(v.trim_matches('"').to_string()),
            "ID" => id = Some(v.trim_matches('"').to_string()),
            "HOME_URL" => home_url = Some(v.trim_matches('"').to_string()),
            _ => continue,
        }
    }

    let of = OSFacts {
        pretty_name,
        name,
        version_id,
        version,
        codename,
        id,
        home_url,
    };
    Ok(of)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_lines(s: &str) -> Vec<String> {
        s.lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn test_parse_into_facts() {
        let input = to_lines(
            r#"PRETTY_NAME="Ubuntu 22.04.3 LTS"
NAME="Ubuntu"
VERSION_ID="22.04"
VERSION="22.04.3 LTS (Jammy Jellyfish)"
VERSION_CODENAME=jammy
ID=ubuntu
HOME_URL="https://www.ubuntu.com/""#,
        );
        let facts = parse_into_facts(input).unwrap();
        assert_eq!(facts.pretty_name.as_deref(), Some("Ubuntu 22.04.3 LTS"));
        assert_eq!(facts.name.as_deref(), Some("Ubuntu"));
        assert_eq!(facts.version_id.as_deref(), Some("22.04"));
        assert_eq!(facts.codename.as_deref(), Some("jammy"));
        assert_eq!(facts.id.as_deref(), Some("ubuntu"));
    }

    #[test]
    fn test_parse_into_facts_unknown_keys_ignored() {
        let input = to_lines(
            r#"NAME="Ubuntu"
UNKNOWN_KEY=somevalue
ID=ubuntu"#,
        );
        let facts = parse_into_facts(input).unwrap();
        assert_eq!(facts.name.as_deref(), Some("Ubuntu"));
        assert_eq!(facts.id.as_deref(), Some("ubuntu"));
    }

    #[test]
    fn test_parse_into_facts_missing_fields_are_none() {
        let input = to_lines("ID=ubuntu\n");
        let facts = parse_into_facts(input).unwrap();
        assert_eq!(facts.id.as_deref(), Some("ubuntu"));
        assert!(facts.pretty_name.is_none());
        assert!(facts.name.is_none());
        assert!(facts.version_id.is_none());
        assert!(facts.codename.is_none());
        assert!(facts.home_url.is_none());
    }

    #[test]
    fn test_parse_into_facts_rhel() {
        let input = to_lines(
            r#"NAME="Red Hat Enterprise Linux"
VERSION="8.10 (Ootpa)"
ID="rhel"
ID_LIKE="fedora"
VERSION_ID="8.10"
PLATFORM_ID="platform:el8"
PRETTY_NAME="Red Hat Enterprise Linux 8.10 (Ootpa)"
HOME_URL="https://www.redhat.com/"
VERSION_CODENAME=ootpa"#,
        );
        let facts = parse_into_facts(input).unwrap();
        assert_eq!(
            facts.pretty_name.as_deref(),
            Some("Red Hat Enterprise Linux 8.10 (Ootpa)")
        );
        assert_eq!(facts.name.as_deref(), Some("Red Hat Enterprise Linux"));
        assert_eq!(facts.version_id.as_deref(), Some("8.10"));
        assert_eq!(facts.version.as_deref(), Some("8.10 (Ootpa)"));
        assert_eq!(facts.codename.as_deref(), Some("ootpa"));
        assert_eq!(facts.id.as_deref(), Some("rhel"));
    }

    #[test]
    fn test_parse_into_facts_suse() {
        let input = to_lines(
            r#"NAME="SLES"
VERSION="15-SP5"
VERSION_ID="15.5"
PRETTY_NAME="SUSE Linux Enterprise Server 15 SP5"
ID="sles"
HOME_URL="https://www.suse.com/""#,
        );
        let facts = parse_into_facts(input).unwrap();
        assert_eq!(
            facts.pretty_name.as_deref(),
            Some("SUSE Linux Enterprise Server 15 SP5")
        );
        assert_eq!(facts.name.as_deref(), Some("SLES"));
        assert_eq!(facts.version_id.as_deref(), Some("15.5"));
        assert_eq!(facts.version.as_deref(), Some("15-SP5"));
        assert_eq!(facts.id.as_deref(), Some("sles"));
        // SUSE doesn't ship VERSION_CODENAME
        assert!(facts.codename.is_none());
    }
}
