use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use validator::{Validate, ValidationError};

// Custom validation function for ISO 8601 date pattern
fn validate_iso8601(date: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(
        r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$",
    )
    .unwrap();
    if re.is_match(date) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_iso8601_format"))
    }
}

// Custom ISO 8601 date type
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Iso8601Date(#[validate(custom = "validate_iso8601")] pub String);

impl From<String> for Iso8601Date {
    fn from(s: String) -> Self {
        Iso8601Date(s)
    }
}

impl From<&str> for Iso8601Date {
    fn from(s: &str) -> Self {
        Iso8601Date(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    #[validate(length(equal = 2))]
    pub country_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Basics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(email)]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub profiles: Option<Vec<Profile>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkExperience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub start_date: Option<Iso8601Date>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub end_date: Option<Iso8601Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VolunteerExperience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub start_date: Option<Iso8601Date>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub end_date: Option<Iso8601Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Education {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,

    #[serde(rename = "studyType", skip_serializing_if = "Option::is_none")]
    pub study_type: Option<String>,

    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub start_date: Option<Iso8601Date>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub end_date: Option<Iso8601Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub courses: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Award {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub date: Option<Iso8601Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Certificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub date: Option<Iso8601Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Publication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub release_date: Option<Iso8601Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Skill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Language {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluency: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Interest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Reference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub start_date: Option<Iso8601Date>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub end_date: Option<Iso8601Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Main Resume structure following JSON Resume schema
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Resume {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub basics: Option<Basics>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub work: Option<Vec<WorkExperience>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub volunteer: Option<Vec<VolunteerExperience>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub education: Option<Vec<Education>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub awards: Option<Vec<Award>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub certificates: Option<Vec<Certificate>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub publications: Option<Vec<Publication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub skills: Option<Vec<Skill>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub languages: Option<Vec<Language>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub interests: Option<Vec<Interest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub references: Option<Vec<Reference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub projects: Option<Vec<Project>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub meta: Option<Meta>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

impl Resume {
    /// Create a new empty resume
    pub fn new() -> Self {
        Resume {
            schema: None,
            basics: None,
            work: None,
            volunteer: None,
            education: None,
            awards: None,
            certificates: None,
            publications: None,
            skills: None,
            languages: None,
            interests: None,
            references: None,
            projects: None,
            meta: None,
            additional_properties: HashMap::new(),
        }
    }

    /// Validate the entire resume structure
    pub fn validate_resume(&self) -> Result<(), validator::ValidationErrors> {
        self.validate()
    }
}

impl Default for Resume {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso8601_validation() {
        assert!(validate_iso8601("2023").is_ok());
        assert!(validate_iso8601("2023-04").is_ok());
        assert!(validate_iso8601("2023-04-15").is_ok());
        assert!(validate_iso8601("invalid").is_err());
    }

    #[test]
    fn test_resume_creation() {
        let resume = Resume::new();
        assert!(resume.validate_resume().is_ok());
    }

    #[test]
    fn test_email_validation() {
        let mut basics = Basics {
            name: None,
            label: None,
            image: None,
            email: Some("invalid-email".to_string()),
            phone: None,
            url: None,
            summary: None,
            location: None,
            profiles: None,
            additional_properties: HashMap::new(),
        };

        assert!(basics.validate().is_err());

        basics.email = Some("valid@example.com".to_string());
        assert!(basics.validate().is_ok());
    }
}
