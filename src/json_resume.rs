use std::collections::HashMap;
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};

#[cfg(feature = "typst")]
use typst::foundations::{Dict, IntoValue, Value};

#[cfg(feature = "serde")]
fn deserialize_iso8601<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Basic regex for YYYY-MM-DD or YYYY-MM or YYYY
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

#[cfg(feature = "serde")]
fn deserialize_email<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
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

// ============================================================================
// ENUMS & SIMPLE TYPES
// ============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LocationType {
    Remote,
    OnSite(String),
    Hybrid {
        #[cfg_attr(feature = "serde", serde(rename = "onSite"))]
        on_site: String,
        description: Option<String>,
    },
}

#[cfg(feature = "typst")]
impl IntoValue for LocationType {
    fn into_value(self) -> Value {
        match self {
            Self::Remote => "Remote".into_value(),
            Self::OnSite(val) => {
                let mut d = Dict::new();
                d.insert("OnSite".into(), val.into_value());
                Value::Dict(d)
            }
            Self::Hybrid {
                on_site,
                description,
            } => {
                let mut content = Dict::new();
                content.insert("onSite".into(), on_site.into_value());
                if let Some(desc) = description {
                    content.insert("description".into(), desc.into_value());
                }
                let mut d = Dict::new();
                d.insert("Hybrid".into(), Value::Dict(content));
                Value::Dict(d)
            }
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Score {
    GPA_Weighted { score: f64, scale: f64 },
    Percentage(u32),
    PassFail(bool),
    LetterGrade(String),
    Custom { score: String, scale: String },
}

#[cfg(feature = "typst")]
impl IntoValue for Score {
    fn into_value(self) -> Value {
        match self {
            Self::GPA_Weighted { score, scale } => {
                let mut content = Dict::new();
                content.insert("score".into(), score.into_value());
                content.insert("scale".into(), scale.into_value());
                let mut d = Dict::new();
                d.insert("GPA_Weighted".into(), Value::Dict(content));
                Value::Dict(d)
            }
            Self::Percentage(val) => {
                let mut d = Dict::new();
                d.insert("Percentage".into(), val.into_value());
                Value::Dict(d)
            }
            Self::PassFail(val) => {
                let mut d = Dict::new();
                d.insert("PassFail".into(), val.into_value());
                Value::Dict(d)
            }
            Self::LetterGrade(val) => {
                let mut d = Dict::new();
                d.insert("LetterGrade".into(), val.into_value());
                Value::Dict(d)
            }
            Self::Custom { score, scale } => {
                let mut content = Dict::new();
                content.insert("score".into(), score.into_value());
                content.insert("scale".into(), scale.into_value());
                let mut d = Dict::new();
                d.insert("Custom".into(), Value::Dict(content));
                Value::Dict(d)
            }
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DegreeType {
    HighSchoolDiploma,
    GED,
    AssociateDegree,
    BachelorDegree,
    MasterDegree,
    MBA,
    JD,
    MD,
    PhD,
    PostDoc,
    Certificate,
    Diploma,
    Bootcamp,
    OnlineCourse,
    Other(String),
}

#[cfg(feature = "typst")]
impl IntoValue for DegreeType {
    fn into_value(self) -> Value {
        match self {
            Self::Other(val) => {
                let mut d = Dict::new();
                d.insert("Other".into(), val.into_value());
                Value::Dict(d)
            }
            _ => format!("{:?}", self).into_value(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Master,
}

#[cfg(feature = "typst")]
impl IntoValue for SkillLevel {
    fn into_value(self) -> Value {
        format!("{:?}", self).into_value()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FluencyLevel {
    Elementary,
    LimitedWorking,
    ProfessionalWorking,
    FullProfessional,
    NativeOrBilingual,
}

#[cfg(feature = "typst")]
impl IntoValue for FluencyLevel {
    fn into_value(self) -> Value {
        format!("{:?}", self).into_value()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LanguageType {
    Afrikaans,
    Albanian,
    Amharic,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bosnian,
    Bulgarian,
    Burmese,
    Catalan,
    Cebuano,
    Chinese,
    Mandarin,
    Cantonese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Galician,
    Georgian,
    German,
    Greek,
    Gujarati,
    HaitianCreole,
    Hausa,
    Hawaiian,
    Hebrew,
    Hindi,
    Hmong,
    Hungarian,
    Icelandic,
    Igbo,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Javanese,
    Kannada,
    Kazakh,
    Khmer,
    Korean,
    Kurdish,
    Kyrgyz,
    Lao,
    Latin,
    Latvian,
    Lithuanian,
    Luxembourgish,
    Macedonian,
    Malagasy,
    Malay,
    Malayalam,
    Maltese,
    Maori,
    Marathi,
    Mongolian,
    Nepali,
    Norwegian,
    Pashto,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romanian,
    Russian,
    Samoan,
    ScottishGaelic,
    Serbian,
    Shona,
    Sindhi,
    Sinhala,
    Slovak,
    Slovenian,
    Somali,
    Spanish,
    Sundanese,
    Swahili,
    Swedish,
    Tagalog,
    Tajik,
    Tamil,
    Telugu,
    Thai,
    Turkish,
    Ukrainian,
    Urdu,
    Uzbek,
    Vietnamese,
    Welsh,
    Xhosa,
    Yiddish,
    Yoruba,
    Zulu,
    Other(String),
}

#[cfg(feature = "typst")]
impl IntoValue for LanguageType {
    fn into_value(self) -> Value {
        match self {
            Self::Other(val) => {
                let mut d = Dict::new();
                d.insert("Other".into(), val.into_value());
                Value::Dict(d)
            }
            _ => format!("{:?}", self).into_value(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProjectType {
    Application,
    Website,
    Library,
    Framework,
    Research,
    OpenSource,
    Conference,
    Talk,
    Presentation,
    Workshop,
    Tutorial,
    Documentation,
    Volunteering,
    Publication,
    Article,
    BlogPost,
    Book,
    Podcast,
    Video,
    Art,
    Music,
    Photography,
    Design,
    Film,
    Theater,
    Dance,
    Writing,
    Poetry,
    Community,
    Activism,
    Fundraising,
    Mentorship,
    Teaching,
    Sports,
    Competition,
    Hackathon,
    Exhibition,
    Performance,
    Installation,
    Sculpture,
    Other(String),
}

#[cfg(feature = "typst")]
impl IntoValue for ProjectType {
    fn into_value(self) -> Value {
        match self {
            Self::Other(val) => {
                let mut d = Dict::new();
                d.insert("Other".into(), val.into_value());
                Value::Dict(d)
            }
            _ => format!("{:?}", self).into_value(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Freelance,
    Internship,
    Apprenticeship,
    Seasonal,
    SelfEmployed,
    Volunteer,
}

#[cfg(feature = "typst")]
impl IntoValue for EmploymentType {
    fn into_value(self) -> Value {
        format!("{:?}", self).into_value()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NetworkType {
    GitHub,
    GitLab,
    Bitbucket,
    LinkedIn,
    Twitter,
    Mastodon,
    Facebook,
    Instagram,
    YouTube,
    Twitch,
    TikTok,
    Reddit,
    StackOverflow,
    HackerNews,
    Medium,
    DevTo,
    Hashnode,
    Substack,
    Personal,
    Portfolio,
    Blog,
    Dribbble,
    Behance,
    Figma,
    CodePen,
    Glitch,
    Replit,
    Observable,
    Kaggle,
    Discord,
    Slack,
    Telegram,
    WhatsApp,
    Signal,
    Email,
    Website,
    Other(String),
}

#[cfg(feature = "typst")]
impl IntoValue for NetworkType {
    fn into_value(self) -> Value {
        match self {
            Self::Other(val) => {
                let mut d = Dict::new();
                d.insert("Other".into(), val.into_value());
                Value::Dict(d)
            }
            _ => format!("{:?}", self).into_value(),
        }
    }
}

// ============================================================================
// STRUCTS
// ============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

#[cfg(feature = "typst")]
impl IntoValue for SemVer {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        dict.insert("major".into(), self.major.into_value());
        dict.insert("minor".into(), self.minor.into_value());
        dict.insert("patch".into(), self.patch.into_value());
        if let Some(prerelease) = self.prerelease {
            dict.insert("prerelease".into(), prerelease.into_value());
        }
        if let Some(build) = self.build {
            dict.insert("build".into(), build.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "postalCode"))]
    pub postal_code: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "countryCode"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_country_code")
    )]
    pub country_code: Option<String>,
}

#[cfg(feature = "typst")]
impl IntoValue for Address {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(street) = self.street {
            dict.insert("street".into(), street.into_value());
        }
        if let Some(city) = self.city {
            dict.insert("city".into(), city.into_value());
        }
        if let Some(region) = self.region {
            dict.insert("region".into(), region.into_value());
        }
        if let Some(postal_code) = self.postal_code {
            dict.insert("postalCode".into(), postal_code.into_value());
        }
        if let Some(country_code) = self.country_code {
            dict.insert("countryCode".into(), country_code.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Profile {
    pub network: Option<NetworkType>,
    pub username: Option<String>,
    pub url: Option<Url>,
}

#[cfg(feature = "typst")]
impl IntoValue for Profile {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(network) = self.network {
            dict.insert("network".into(), network.into_value());
        }
        if let Some(username) = self.username {
            dict.insert("username".into(), username.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PronounSet {
    pub subject: String,
    pub object: String,
    #[cfg_attr(feature = "serde", serde(rename = "possessiveAdj"))]
    pub possessive_adj: String,
    #[cfg_attr(feature = "serde", serde(rename = "possessivePronoun"))]
    pub possessive_pronoun: String,
    pub reflexive: String,
}

#[cfg(feature = "typst")]
impl IntoValue for PronounSet {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        dict.insert("subject".into(), self.subject.into_value());
        dict.insert("object".into(), self.object.into_value());
        dict.insert("possessiveAdj".into(), self.possessive_adj.into_value());
        dict.insert(
            "possessivePronoun".into(),
            self.possessive_pronoun.into_value(),
        );
        dict.insert("reflexive".into(), self.reflexive.into_value());
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pronouns {
    pub primary: Option<PronounSet>,
    pub additional: Vec<PronounSet>,
    pub display: String,
}

#[cfg(feature = "typst")]
impl IntoValue for Pronouns {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(primary) = self.primary {
            dict.insert("primary".into(), primary.into_value());
        }
        let additional_array: Vec<_> = self
            .additional
            .into_iter()
            .map(|p| p.into_value())
            .collect();
        dict.insert("additional".into(), additional_array.into_value());
        dict.insert("display".into(), self.display.into_value());
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Basics {
    pub name: Option<String>,
    pub label: Option<String>,
    pub pronouns: Option<Pronouns>,
    pub image: Option<Url>,

    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_email"))]
    pub email: Option<String>,

    pub phone: Option<String>,
    pub url: Option<Url>,
    pub summary: Option<String>,
    pub location: Option<Address>,
    pub profiles: Option<Vec<Profile>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Basics {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(label) = self.label {
            dict.insert("label".into(), label.into_value());
        }
        if let Some(pronouns) = self.pronouns {
            dict.insert("pronouns".into(), pronouns.into_value());
        }
        if let Some(image) = self.image {
            dict.insert("image".into(), image.to_string().into_value());
        }
        if let Some(email) = self.email {
            dict.insert("email".into(), email.into_value());
        }
        if let Some(phone) = self.phone {
            dict.insert("phone".into(), phone.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(summary) = self.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        if let Some(location) = self.location {
            dict.insert("location".into(), location.into_value());
        }
        if let Some(profiles) = self.profiles {
            let profiles_array: Vec<_> = profiles.into_iter().map(|p| p.into_value()).collect();
            dict.insert("profiles".into(), profiles_array.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Work {
    pub name: Option<String>,
    pub location: Option<LocationType>,
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
    #[cfg_attr(feature = "serde", serde(rename = "employmentType"))]
    pub employment_type: Option<EmploymentType>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Work {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(location) = self.location {
            dict.insert("location".into(), location.into_value());
        }
        if let Some(description) = self.description {
            dict.insert("description".into(), description.into_value());
        }
        if let Some(position) = self.position {
            dict.insert("position".into(), position.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(start_date) = self.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = self.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(summary) = self.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        if let Some(highlights) = self.highlights {
            dict.insert("highlights".into(), highlights.into_value());
        }
        if let Some(employment_type) = self.employment_type {
            dict.insert("employmentType".into(), employment_type.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Volunteer {
    pub organization: Option<String>,
    pub position: Option<String>,
    pub url: Option<Url>,
    pub location: Option<LocationType>,

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
impl IntoValue for Volunteer {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(organization) = self.organization {
            dict.insert("organization".into(), organization.into_value());
        }
        if let Some(position) = self.position {
            dict.insert("position".into(), position.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(location) = self.location {
            dict.insert("location".into(), location.into_value());
        }
        if let Some(start_date) = self.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = self.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(summary) = self.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        if let Some(highlights) = self.highlights {
            dict.insert("highlights".into(), highlights.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Education {
    pub institution: Option<String>,
    pub url: Option<Url>,
    pub area: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "studyType"))]
    pub study_type: Option<DegreeType>,

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

    pub score: Option<Score>,
    pub courses: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Education {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(institution) = self.institution {
            dict.insert("institution".into(), institution.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(area) = self.area {
            dict.insert("area".into(), area.into_value());
        }
        if let Some(study_type) = self.study_type {
            dict.insert("studyType".into(), study_type.into_value());
        }
        if let Some(start_date) = self.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = self.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(score) = self.score {
            dict.insert("score".into(), score.into_value());
        }
        if let Some(courses) = self.courses {
            dict.insert("courses".into(), courses.into_value());
        }
        Value::Dict(dict)
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
impl IntoValue for Award {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(title) = self.title {
            dict.insert("title".into(), title.into_value());
        }
        if let Some(date) = self.date {
            dict.insert("date".into(), date.into_value());
        }
        if let Some(awarder) = self.awarder {
            dict.insert("awarder".into(), awarder.into_value());
        }
        if let Some(summary) = self.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        Value::Dict(dict)
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

    #[cfg_attr(feature = "serde", serde(rename = "expirationDate"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_optional_iso8601")
    )]
    pub expiration_date: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Certificate {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(date) = self.date {
            dict.insert("date".into(), date.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(issuer) = self.issuer {
            dict.insert("issuer".into(), issuer.into_value());
        }
        if let Some(expiration_date) = self.expiration_date {
            dict.insert("expirationDate".into(), expiration_date.into_value());
        }
        Value::Dict(dict)
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
impl IntoValue for Publication {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(publisher) = self.publisher {
            dict.insert("publisher".into(), publisher.into_value());
        }
        if let Some(release_date) = self.release_date {
            dict.insert("releaseDate".into(), release_date.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(summary) = self.summary {
            dict.insert("summary".into(), summary.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Skill {
    pub name: Option<String>,
    pub level: Option<SkillLevel>,
    pub keywords: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Skill {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(level) = self.level {
            dict.insert("level".into(), level.into_value());
        }
        if let Some(keywords) = self.keywords {
            dict.insert("keywords".into(), keywords.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Language {
    pub language: Option<LanguageType>,
    pub fluency: Option<FluencyLevel>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Language {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(language) = self.language {
            dict.insert("language".into(), language.into_value());
        }
        if let Some(fluency) = self.fluency {
            dict.insert("fluency".into(), fluency.into_value());
        }
        Value::Dict(dict)
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
impl IntoValue for Interest {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(keywords) = self.keywords {
            dict.insert("keywords".into(), keywords.into_value());
        }
        Value::Dict(dict)
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
impl IntoValue for Reference {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(reference) = self.reference {
            dict.insert("reference".into(), reference.into_value());
        }
        Value::Dict(dict)
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
    pub project_type: Option<ProjectType>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Project {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(name) = self.name {
            dict.insert("name".into(), name.into_value());
        }
        if let Some(description) = self.description {
            dict.insert("description".into(), description.into_value());
        }
        if let Some(highlights) = self.highlights {
            dict.insert("highlights".into(), highlights.into_value());
        }
        if let Some(keywords) = self.keywords {
            dict.insert("keywords".into(), keywords.into_value());
        }
        if let Some(start_date) = self.start_date {
            dict.insert("startDate".into(), start_date.into_value());
        }
        if let Some(end_date) = self.end_date {
            dict.insert("endDate".into(), end_date.into_value());
        }
        if let Some(url) = self.url {
            dict.insert("url".into(), url.to_string().into_value());
        }
        if let Some(roles) = self.roles {
            dict.insert("roles".into(), roles.into_value());
        }
        if let Some(entity) = self.entity {
            dict.insert("entity".into(), entity.into_value());
        }
        if let Some(project_type) = self.project_type {
            dict.insert("type".into(), project_type.into_value());
        }
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LastModified {
    pub date: String,
    pub time: String,
}

#[cfg(feature = "typst")]
impl IntoValue for LastModified {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        dict.insert("date".into(), self.date.into_value());
        dict.insert("time".into(), self.time.into_value());
        Value::Dict(dict)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Meta {
    pub canonical: Option<Url>,
    pub version: Option<SemVer>,

    #[cfg_attr(feature = "serde", serde(rename = "lastModified"))]
    pub last_modified: Option<LastModified>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[cfg(feature = "typst")]
impl IntoValue for Meta {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(canonical) = self.canonical {
            dict.insert("canonical".into(), canonical.to_string().into_value());
        }
        if let Some(version) = self.version {
            dict.insert("version".into(), version.into_value());
        }
        if let Some(last_modified) = self.last_modified {
            dict.insert("lastModified".into(), last_modified.into_value());
        }
        Value::Dict(dict)
    }
}

// ============================================================================
// MAIN RESUME STRUCT
// ============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resume {
    #[cfg_attr(feature = "serde", serde(rename = "$schema"))]
    pub schema: Option<Url>,

    pub basics: Option<Basics>,
    pub work: Option<Vec<Work>>,
    pub volunteer: Option<Vec<Volunteer>>,
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
impl IntoValue for Resume {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        if let Some(schema) = self.schema {
            dict.insert("schema".into(), schema.to_string().into_value());
        }
        if let Some(basics) = self.basics {
            dict.insert("basics".into(), basics.into_value());
        }
        if let Some(work) = self.work {
            let work_array: Vec<_> = work.into_iter().map(|w| w.into_value()).collect();
            dict.insert("work".into(), work_array.into_value());
        }
        if let Some(volunteer) = self.volunteer {
            let volunteer_array: Vec<_> = volunteer.into_iter().map(|v| v.into_value()).collect();
            dict.insert("volunteer".into(), volunteer_array.into_value());
        }
        if let Some(education) = self.education {
            let education_array: Vec<_> = education.into_iter().map(|e| e.into_value()).collect();
            dict.insert("education".into(), education_array.into_value());
        }
        if let Some(awards) = self.awards {
            let awards_array: Vec<_> = awards.into_iter().map(|a| a.into_value()).collect();
            dict.insert("awards".into(), awards_array.into_value());
        }
        if let Some(certificates) = self.certificates {
            let certificates_array: Vec<_> =
                certificates.into_iter().map(|c| c.into_value()).collect();
            dict.insert("certificates".into(), certificates_array.into_value());
        }
        if let Some(publications) = self.publications {
            let publications_array: Vec<_> =
                publications.into_iter().map(|p| p.into_value()).collect();
            dict.insert("publications".into(), publications_array.into_value());
        }
        if let Some(skills) = self.skills {
            let skills_array: Vec<_> = skills.into_iter().map(|s| s.into_value()).collect();
            dict.insert("skills".into(), skills_array.into_value());
        }
        if let Some(languages) = self.languages {
            let languages_array: Vec<_> = languages.into_iter().map(|l| l.into_value()).collect();
            dict.insert("languages".into(), languages_array.into_value());
        }
        if let Some(interests) = self.interests {
            let interests_array: Vec<_> = interests.into_iter().map(|i| i.into_value()).collect();
            dict.insert("interests".into(), interests_array.into_value());
        }
        if let Some(references) = self.references {
            let references_array: Vec<_> = references.into_iter().map(|r| r.into_value()).collect();
            dict.insert("references".into(), references_array.into_value());
        }
        if let Some(projects) = self.projects {
            let projects_array: Vec<_> = projects.into_iter().map(|p| p.into_value()).collect();
            dict.insert("projects".into(), projects_array.into_value());
        }
        if let Some(meta) = self.meta {
            dict.insert("meta".into(), meta.into_value());
        }
        Value::Dict(dict)
    }
}

impl Default for Resume {
    fn default() -> Self {
        Self {
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

#[cfg(feature = "typst")]
impl From<Address> for Dict {
    fn from(value: Address) -> Self {
        if let Value::Dict(d) = value.into_value() {
            d
        } else {
            Dict::new()
        }
    }
}

#[cfg(feature = "typst")]
impl From<Work> for Dict {
    fn from(value: Work) -> Self {
        if let Value::Dict(d) = value.into_value() {
            d
        } else {
            Dict::new()
        }
    }
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
                pronouns: None,
                location: Some(Address {
                    city: Some("San Francisco".to_string()),
                    country_code: Some("US".to_string()),
                    street: None,
                    postal_code: None,
                    region: None,
                }),
                label: None,
                image: None,
                phone: None,
                url: None,
                summary: None,
                profiles: None,
                additional_properties: HashMap::new(),
            }),
            work: Some(vec![Work {
                name: Some("Tech Corp".to_string()),
                position: Some("Software Engineer".to_string()),
                start_date: Some("2020-01".to_string()),
                end_date: Some("2023-12".to_string()),
                location: None,
                description: None,
                url: None,
                summary: None,
                highlights: None,
                employment_type: None,
                additional_properties: HashMap::new(),
            }]),
            skills: Some(vec![
                Skill {
                    name: Some("Rust".to_string()),
                    level: Some(SkillLevel::Expert),
                    keywords: Some(vec!["systems".to_string(), "performance".to_string()]),
                    additional_properties: HashMap::new(),
                },
                Skill {
                    name: Some("Python".to_string()),
                    level: Some(SkillLevel::Advanced),
                    keywords: None,
                    additional_properties: HashMap::new(),
                },
            ]),
            ..Default::default()
        };

        let dict: Dict = resume.into();

        assert!(dict.get("basics").is_ok());
        assert!(dict.get("work").is_ok());
        assert!(dict.get("skills").is_ok());
    }

    #[test]
    #[cfg(feature = "typst")]
    fn test_location_into_dict() {
        let address = Address {
            street: Some("123 Main St".to_string()),
            postal_code: Some("94102".to_string()),
            city: Some("San Francisco".to_string()),
            country_code: Some("US".to_string()),
            region: Some("CA".to_string()),
        };

        // Note: Using the trait implementation `into_value` which converts to Value::Dict
        let val = address.into_value();
        if let Value::Dict(dict) = val {
            assert!(dict.get("street").is_ok());
            assert!(dict.get("postalCode").is_ok());
            assert!(dict.get("city").is_ok());
            assert!(dict.get("countryCode").is_ok());
            assert!(dict.get("region").is_ok());
        } else {
            panic!("Address should convert to a Dict");
        }
    }

    #[test]
    #[cfg(feature = "typst")]
    fn test_work_experience_into_dict() {
        let work = Work {
            name: Some("Acme Corp".to_string()),
            position: Some("Senior Developer".to_string()),
            start_date: Some("2020-06".to_string()),
            end_date: None,
            location: Some(LocationType::Remote),
            description: Some("Building cool things".to_string()),
            url: None,
            summary: Some("Led development team".to_string()),
            highlights: Some(vec![
                "Improved performance by 50%".to_string(),
                "Mentored 5 developers".to_string(),
            ]),
            employment_type: Some(EmploymentType::FullTime),
            additional_properties: HashMap::new(),
        };

        let val = work.into_value();
        if let Value::Dict(dict) = val {
            assert!(dict.get("name").is_ok());
            assert!(dict.get("position").is_ok());
            assert!(dict.get("startDate").is_ok());
            assert!(dict.get("highlights").is_ok());
        } else {
            panic!("Work should convert to a Dict");
        }
    }

    #[test]
    #[cfg(feature = "typst")]
    fn test_empty_resume_into_dict() {
        let resume = Resume::default();
        let val = resume.into_value();
        if let Value::Dict(dict) = val {
            // Empty resume with Default should have 0 keys (all Nones)
            assert_eq!(dict.len(), 0);
        }
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

        let resume: Resume = serde_json::from_str(json).expect("Failed to deserialize");
        let val = resume.into_value();
        if let Value::Dict(dict) = val {
            assert!(dict.get("basics").is_ok());
            assert!(dict.get("skills").is_ok());
        } else {
            panic!("Resume should convert to a Dict");
        }
    }
}
