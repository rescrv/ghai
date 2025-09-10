use policyai::PolicyType;

////////////////////////////////////////////// policy //////////////////////////////////////////////

pub const POLICY: &str = r#"type ghai::Policy {
    action: ["mark-read", "mark-unread"] @ highest wins,
    priority: ["low", "medium", "high"] @ highest wins,
    label: [string],
}
"#;

pub fn get_policy_type() -> PolicyType {
    PolicyType::parse(POLICY).unwrap()
}

///////////////////////////////////////////// Decision /////////////////////////////////////////////

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Decision {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub label: Vec<String>,
}

/////////////////////////////////////////////// tests //////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_parses_successfully() {
        let policy_type = PolicyType::parse(POLICY);
        assert!(policy_type.is_ok());
    }

    #[test]
    fn get_policy_type_returns_valid_policy() {
        let _policy_type = get_policy_type();
    }
}
