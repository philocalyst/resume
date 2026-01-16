use std::collections::HashMap;
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};

#[cfg(feature = "typst")]
use typst::foundations::{Dict, IntoValue};

// Custom deserializer for ISO 8601 date pattern
#[cfg(feature = "serde")]
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
#[cfg(feature = "serde")]
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
#[cfg(feature = "serde")]
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
#[cfg(feature = "serde")]
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Location {
    pub address: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "postalCode"))]
    pub postal_code: Option<String>,

    pub city: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "countryCode"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_country_code")
    )]
    pub country_code: Option<String>,

    pub region: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Location> for Dict {
    fn from(value: Location) -> Self {
        let mut dict = Dict::new();
        if let Some(address) = value.address {
            dict.insert("address".into(), address.into_value());
        }
        if let Some(postal_code) = value.postal_code {
            dict.insert("postalCode".into(), postal_code.into_value());
        }
        if let Some(city) = value.city {
            dict.insert("city".into(), city.into_value());
        }
        if let Some(country_code) = value.country_code {
            dict.insert("countryCode".into(), country_code.into_value());
        }
        if let Some(region) = value.region {
            dict.insert("region".into(), region.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Location {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Profile {
    pub network: Option<String>,
    pub username: Option<String>,
    pub url: Option<Url>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Profile> for Dict {
    fn from(value: Profile) -> Self {
        let mut dict = Dict::new();
        if let Some(network) = value.network {
            dict.insert("network".into(), network.into_value());
        }
        if let Some(username) = value.username {
            dict.insert("username".into(), username.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Profile {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Basics {
    pub name: Option<String>,
    pub label: Option<String>,
    pub image: Option<Url>,

    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_email"))]
    pub email: Option<String>,

    pub phone: Option<String>,
    pub url: Option<Url>,
    pub summary: Option<String>,
    pub location: Option<Location>,
    pub profiles: Option<Vec<Profile>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Basics> for Dict {
    fn from(value: Basics) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(label) = value.label {
            dict.insert("label".into(), label.into_value());
        }
        if let Some(image) = value.image {
            dict.insert("image".into(), image.to_string().into_value());
        }
        if let Some(email) = value.email {
            dict.insert("email".into(), email.into_value());
        }
        if let Some(phone) = value.phone {
            dict.insert("phone".into(), phone.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(summary) = value.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        if let Some(location) = value.location {
            dict.insert("location".into(), location.into_value());
        }
        if let Some(profiles) = value.profiles {
            let profiles_array: Vec<_> = profiles.into_iter().map(|p| p.into_value()).collect();
            dict.insert("profiles".into(), profiles_array.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Basics {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkExperience {
    pub name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub position: Option<String>,
    pub url: Option<Url>,

    #[cfg_attr(feature = "serde", serde(rename = "startDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub start_date: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "endDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub end_date: Option<String>,

    pub summary: Option<String>,
    pub highlights: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<WorkExperience> for Dict {
    fn from(value: WorkExperience) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(location) = value.location {
            dict.insert("location".into(), location.into_value());
        }
        if let Some(description) = value.description {
            dict.insert("description".into(), description.into_value());
        }
        if let Some(position) = value.position {
            dict.insert("position".into(), position.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(start_date) = value.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = value.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(summary) = value.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        if let Some(highlights) = value.highlights {
            dict.insert("highlights".into(), highlights.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for WorkExperience {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VolunteerExperience {
    pub organization: Option<String>,
    pub position: Option<String>,
    pub url: Option<Url>,

    #[cfg_attr(feature = "serde", serde(rename = "startDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub start_date: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "endDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub end_date: Option<String>,

    pub summary: Option<String>,
    pub highlights: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<VolunteerExperience> for Dict {
    fn from(value: VolunteerExperience) -> Self {
        let mut dict = Dict::new();
        if let Some(organization) = value.organization {
            dict.insert("organization".into(), organization.into_value());
        }
        if let Some(position) = value.position {
            dict.insert("position".into(), position.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(start_date) = value.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = value.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(summary) = value.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        if let Some(highlights) = value.highlights {
            dict.insert("highlights".into(), highlights.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for VolunteerExperience {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Education {
    pub institution: Option<String>,
    pub url: Option<Url>,
    pub area: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "studyType"))]
    pub study_type: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "startDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub start_date: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "endDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub end_date: Option<String>,

    pub score: Option<String>,
    pub courses: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Education> for Dict {
    fn from(value: Education) -> Self {
        let mut dict = Dict::new();
        if let Some(institution) = value.institution {
            dict.insert("institution".into(), institution.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(area) = value.area {
            dict.insert("area".into(), area.into_value());
        }
        if let Some(study_type) = value.study_type {
            dict.insert("studyType".into(), study_type.into_value());
        }
        if let Some(start_date) = value.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = value.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(score) = value.score {
            dict.insert("score".into(), score.into_value());
        }
        if let Some(courses) = value.courses {
            dict.insert("courses".into(), courses.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Education {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Award {
    pub title: Option<String>,

    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub date: Option<String>,

    pub awarder: Option<String>,
    pub summary: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Award> for Dict {
    fn from(value: Award) -> Self {
        let mut dict = Dict::new();
        if let Some(title) = value.title {
            dict.insert("title".into(), title.into_value());
        }
        if let Some(date) = value.date {
            dict.insert("date".into(), date.into_value());
        }
        if let Some(awarder) = value.awarder {
            dict.insert("awarder".into(), awarder.into_value());
        }
        if let Some(summary) = value.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Award {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Certificate {
    pub name: Option<String>,

    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub date: Option<String>,

    pub url: Option<Url>,
    pub issuer: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Certificate> for Dict {
    fn from(value: Certificate) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(date) = value.date {
            dict.insert("date".into(), date.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(issuer) = value.issuer {
            dict.insert("issuer".into(), issuer.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Certificate {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Publication {
    pub name: Option<String>,
    pub publisher: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "releaseDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub release_date: Option<String>,

    pub url: Option<Url>,
    pub summary: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Publication> for Dict {
    fn from(value: Publication) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(publisher) = value.publisher {
            dict.insert("publisher".into(), publisher.into_value());
        }
        if let Some(release_date) = value.release_date {
            dict.insert("releaseDate".into(), release_date.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(summary) = value.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Publication {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Skill {
    pub name: Option<String>,
    pub level: Option<String>,
    pub keywords: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Skill> for Dict {
    fn from(value: Skill) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(level) = value.level {
            dict.insert("level".into(), level.into_value());
        }
        if let Some(keywords) = value.keywords {
            dict.insert("keywords".into(), keywords.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Skill {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Language {
    pub language: Option<String>,
    pub fluency: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Language> for Dict {
    fn from(value: Language) -> Self {
        let mut dict = Dict::new();
        if let Some(language) = value.language {
            dict.insert("language".into(), language.into_value());
        }
        if let Some(fluency) = value.fluency {
            dict.insert("fluency".into(), fluency.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Language {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Interest {
    pub name: Option<String>,
    pub keywords: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Interest> for Dict {
    fn from(value: Interest) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(keywords) = value.keywords {
            dict.insert("keywords".into(), keywords.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Interest {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Reference {
    pub name: Option<String>,
    pub reference: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Reference> for Dict {
    fn from(value: Reference) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(reference) = value.reference {
            dict.insert("reference".into(), reference.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Reference {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Project {
    pub name: Option<String>,
    pub description: Option<String>,
    pub highlights: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(rename = "startDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub start_date: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "endDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub end_date: Option<String>,

    pub url: Option<Url>,
    pub roles: Option<Vec<String>>,
    pub entity: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub project_type: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Project> for Dict {
    fn from(value: Project) -> Self {
        let mut dict = Dict::new();
        if let Some(name) = value.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(description) = value.description {
            dict.insert("description".into(), description.into_value());
        }
        if let Some(highlights) = value.highlights {
            dict.insert("highlights".into(), highlights.into_value());
        }
        if let Some(keywords) = value.keywords {
            dict.insert("keywords".into(), keywords.into_value());
        }
        if let Some(start_date) = value.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = value.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(url) = value.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(roles) = value.roles {
            dict.insert("roles".into(), roles.into_value());
        }
        if let Some(entity) = value.entity {
            dict.insert("entity".into(), entity.into_value());
        }
        if let Some(project_type) = value.project_type {
            dict.insert("type".into(), project_type.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Project {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Meta {
    pub canonical: Option<Url>,
    pub version: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "lastModified"))]
    pub last_modified: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Meta> for Dict {
    fn from(value: Meta) -> Self {
        let mut dict = Dict::new();
        if let Some(canonical) = value.canonical {
            dict.insert("canonical".into(), canonical.to_string().into_value());
        }
        if let Some(version) = value.version {
            dict.insert("version".into(), version.into_value());
        }
        if let Some(last_modified) = value.last_modified {
            dict.insert("lastModified".into(), last_modified.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Meta {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
}

/// Main Resume structure following JSON Resume schema
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resume {
    #[cfg_attr(feature = "serde", serde(rename = "$schema"))]
    pub schema: Option<Url>,

    pub basics: Option<Basics>,
    pub work: Option<Vec<WorkExperience>>,
    pub volunteer: Option<Vec<VolunteerExperience>>,
    pub education: Option<Vec<Education>>,
    pub awards: Option<Vec<Award>>,
    pub certificates: Option<Vec<Certificate>>,
    pub publications: Option<Vec<Publication>>,
    pub skills: Option<Vec<Skill>>,
    pub languages: Option<Vec<Language>>,
    pub interests: Option<Vec<Interest>>,
    pub references: Option<Vec<Reference>>,
    pub projects: Option<Vec<Project>>,
    pub meta: Option<Meta>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl From<Resume> for Dict {
    fn from(value: Resume) -> Self {
        let mut dict = Dict::new();
        if let Some(schema) = value.schema {
            dict.insert("schema".into(), schema.to_string().into_value());
        }
        if let Some(basics) = value.basics {
            dict.insert("basics".into(), basics.into_value());
        }
        if let Some(work) = value.work {
            let work_array: Vec<_> = work.into_iter().map(|w| w.into_value()).collect();
            dict.insert("work".into(), work_array.into_value());
        }
        if let Some(volunteer) = value.volunteer {
            let volunteer_array: Vec<_> = volunteer.into_iter().map(|v| v.into_value()).collect();
            dict.insert("volunteer".into(), volunteer_array.into_value());
        }
        if let Some(education) = value.education {
            let education_array: Vec<_> = education.into_iter().map(|e| e.into_value()).collect();
            dict.insert("education".into(), education_array.into_value());
        }
        if let Some(awards) = value.awards {
            let awards_array: Vec<_> = awards.into_iter().map(|a| a.into_value()).collect();
            dict.insert("awards".into(), awards_array.into_value());
        }
        if let Some(certificates) = value.certificates {
            let certificates_array: Vec<_> =
                certificates.into_iter().map(|c| c.into_value()).collect();
            dict.insert("certificates".into(), certificates_array.into_value());
        }
        if let Some(publications) = value.publications {
            let publications_array: Vec<_> =
                publications.into_iter().map(|p| p.into_value()).collect();
            dict.insert("publications".into(), publications_array.into_value());
        }
        if let Some(skills) = value.skills {
            let skills_array: Vec<_> = skills.into_iter().map(|s| s.into_value()).collect();
            dict.insert("skills".into(), skills_array.into_value());
        }
        if let Some(languages) = value.languages {
            let languages_array: Vec<_> = languages.into_iter().map(|l| l.into_value()).collect();
            dict.insert("languages".into(), languages_array.into_value());
        }
        if let Some(interests) = value.interests {
            let interests_array: Vec<_> = interests.into_iter().map(|i| i.into_value()).collect();
            dict.insert("interests".into(), interests_array.into_value());
        }
        if let Some(references) = value.references {
            let references_array: Vec<_> = references.into_iter().map(|r| r.into_value()).collect();
            dict.insert("references".into(), references_array.into_value());
        }
        if let Some(projects) = value.projects {
            let projects_array: Vec<_> = projects.into_iter().map(|p| p.into_value()).collect();
            dict.insert("projects".into(), projects_array.into_value());
        }
        if let Some(meta) = value.meta {
            dict.insert("meta".into(), meta.into_value());
        }
        dict
    }
}

#[cfg(feature = "typst")]
impl IntoValue for Resume {
    fn into_value(self) -> typst::foundations::Value {
        typst::foundations::Value::Dict(self.into())
    }
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
    #[cfg(feature = "serde")]
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
    #[cfg(feature = "serde")]
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
    #[cfg(feature = "serde")]
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
    #[cfg(feature = "serde")]
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

    #[test]
    #[cfg(feature = "serde")]
    fn test_valid_iso_dates() {
        let json = r#"{
            "work": [{
                "startDate": "2023",
                "endDate": "2023-12"
            }],
            "education": [{
                "startDate": "2020-01-15"
            }]
        }"#;

        let resume: Result<Resume, _> = serde_json::from_str(json);
        assert!(resume.is_ok());
    }

    #[test]
    #[cfg(feature = "typst")]
    fn test_resume_into_dict() {
        let resume = Resume {
            basics: Some(Basics {
                name: Some("Jane Smith".to_string()),
                email: Some("jane@example.com".to_string()),
                location: Some(Location {
                    city: Some("San Francisco".to_string()),
                    country_code: Some("US".to_string()),
                    address: None,
                    postal_code: None,
                    region: None,
                    additional_properties: HashMap::new(),
                }),
                label: None,
                image: None,
                phone: None,
                url: None,
                summary: None,
                profiles: None,
                additional_properties: HashMap::new(),
            }),
            work: Some(vec![WorkExperience {
                name: Some("Tech Corp".to_string()),
                position: Some("Software Engineer".to_string()),
                start_date: Some("2020-01".to_string()),
                end_date: Some("2023-12".to_string()),
                location: None,
                description: None,
                url: None,
                summary: None,
                highlights: None,
                additional_properties: HashMap::new(),
            }]),
            skills: Some(vec![
                Skill {
                    name: Some("Rust".to_string()),
                    level: Some("Expert".to_string()),
                    keywords: Some(vec!["systems".to_string(), "performance".to_string()]),
                    additional_properties: HashMap::new(),
                },
                Skill {
                    name: Some("Python".to_string()),
                    level: Some("Advanced".to_string()),
                    keywords: None,
                    additional_properties: HashMap::new(),
                },
            ]),
            volunteer: None,
            education: None,
            awards: None,
            certificates: None,
            publications: None,
            languages: None,
            interests: None,
            references: None,
            projects: None,
            meta: None,
            schema: None,
            additional_properties: HashMap::new(),
        };

        let dict: Dict = resume.into();

        // Verify the dict contains expected keys
        assert!(dict.get("basics").is_ok());
        assert!(dict.get("work").is_ok());
        assert!(dict.get("skills").is_ok());
    }

    #[test]
    #[cfg(feature = "typst")]
    fn test_location_into_dict() {
        let location = Location {
            address: Some("123 Main St".to_string()),
            postal_code: Some("94102".to_string()),
            city: Some("San Francisco".to_string()),
            country_code: Some("US".to_string()),
            region: Some("CA".to_string()),
            additional_properties: HashMap::new(),
        };

        let dict: Dict = location.into();

        assert!(dict.get("address").is_ok());
        assert!(dict.get("postalCode").is_ok());
        assert!(dict.get("city").is_ok());
        assert!(dict.get("countryCode").is_ok());
        assert!(dict.get("region").is_ok());
    }

    #[test]
    #[cfg(feature = "typst")]
    fn test_work_experience_into_dict() {
        let work = WorkExperience {
            name: Some("Acme Corp".to_string()),
            position: Some("Senior Developer".to_string()),
            start_date: Some("2020-06".to_string()),
            end_date: None,
            location: Some("Remote".to_string()),
            description: Some("Building cool things".to_string()),
            url: None,
            summary: Some("Led development team".to_string()),
            highlights: Some(vec![
                "Improved performance by 50%".to_string(),
                "Mentored 5 developers".to_string(),
            ]),
            additional_properties: HashMap::new(),
        };

        let dict: Dict = work.into();

        assert!(dict.get("name").is_ok());
        assert!(dict.get("position").is_ok());
        assert!(dict.get("startDate").is_ok());
        assert!(dict.get("highlights").is_ok());
    }

    #[test]
    #[cfg(feature = "typst")]
    fn test_empty_resume_into_dict() {
        let resume = Resume::new();
        let dict: Dict = resume.into();

        // Empty resume should create an empty dict
        assert_eq!(dict.len(), 0);
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "typst"))]
    fn test_serde_to_dict_roundtrip() {
        let json = r#"{
            "basics": {
                "name": "Test User",
                "email": "test@example.com"
            },
            "skills": [{
                "name": "Rust",
                "level": "Expert"
            }]
        }"#;

        let resume: Resume = serde_json::from_str(json).unwrap();
        let dict: Dict = resume.into();

        assert!(dict.get("basics").is_ok());
        assert!(dict.get("skills").is_ok());
    }
}
