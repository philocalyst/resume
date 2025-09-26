use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use url::Url;

// Custom deserializer for ISO 8601 date pattern
fn deserialize_iso8601<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let re = regex::Regex::new(
        r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$",
    )
    .unwrap();

    if re.is_match(&s) {
        Ok(s)
    } else {
        Err(serde::de::Error::custom("Invalid ISO 8601 date format"))
    }
}

// Custom deserializer for optional ISO 8601 dates
fn deserialize_optional_iso8601<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let re = regex::Regex::new(
                r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$",
            )
            .unwrap();
            if re.is_match(&s) {
                Ok(Some(s))
            } else {
                Err(serde::de::Error::custom("Invalid ISO 8601 date format"))
            }
        }
        None => Ok(None),
    }
}

// Custom deserializer for country codes (ISO-3166-1 ALPHA-2)
fn deserialize_country_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            if s.len() == 2 && s.chars().all(|c| c.is_ascii_uppercase()) {
                Ok(Some(s))
            } else {
                Err(serde::de::Error::custom(
                    "Country code must be 2 uppercase letters (ISO-3166-1 ALPHA-2)",
                ))
            }
        }
        None => Ok(None),
    }
}

// Custom deserializer for email validation
fn deserialize_email<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            // Simple email regex validation
            let email_re = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
            if email_re.is_match(&s) {
                Ok(Some(s))
            } else {
                Err(serde::de::Error::custom("Invalid email format"))
            }
        }
        None => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_country_code")]
    pub country_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_email")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<Profile>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub start_date: Option<String>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub end_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerExperience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub start_date: Option<String>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub end_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub start_date: Option<String>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub end_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub courses: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Award {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub release_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluency: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub start_date: Option<String>,

    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_optional_iso8601")]
    pub end_date: Option<String>,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub basics: Option<Basics>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work: Option<Vec<WorkExperience>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volunteer: Option<Vec<VolunteerExperience>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<Vec<Education>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awards: Option<Vec<Award>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<Skill>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<Language>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<Interest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<Reference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<Project>>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn test_valid_resume_parsing() {
        let json = r#"{
            "basics": {
                "name": "John Doe",
                "email": "john@example.com",
                "location": {
                    "countryCode": "US"
                }
            },
            "work": [{
                "name": "Company",
                "startDate": "2023-01",
                "endDate": "2023-12-31"
            }]
        }"#;

        let resume: Resume = serde_json::from_str(json).unwrap();
        assert_eq!(
            resume.basics.as_ref().unwrap().name.as_ref().unwrap(),
            "John Doe"
        );
    }

    #[test]
    fn test_invalid_email_fails() {
        let json = r#"{
            "basics": {
                "email": "invalid-email"
            }
        }"#;

        let result: Result<Resume, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_date_fails() {
        let json = r#"{
            "work": [{
                "startDate": "invalid-date"
            }]
        }"#;

        let result: Result<Resume, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_country_code_fails() {
        let json = r#"{
            "basics": {
                "location": {
                    "countryCode": "USA"
                }
            }
        }"#;

        let result: Result<Resume, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
