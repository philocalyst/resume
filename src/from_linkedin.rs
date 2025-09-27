use crate::json_resume::*;
use linkedin_api::types;
use std::collections::HashMap;
use url::Url;

/// Trait for converting LinkedIn data structures to JSON Resume format
pub trait ToJsonResume {
    fn to_json_resume(&self) -> Resume;
}

impl ToJsonResume for linkedin_api::types::Profile {
    fn to_json_resume(&self) -> Resume {
        let mut resume = Resume::new();

        // Convert basics section
        let mut basics = Basics {
            name: self.get_full_name(),
            label: self.headline.clone(),
            image: self.get_profile_image_url(),
            email: None, // Not available in basic profile
            phone: None, // Not available in basic profile
            url: None,   // Could be constructed from public profile URL
            summary: self.summary.clone(),
            location: self.convert_location(),
            profiles: None, // Could be populated with LinkedIn profile info
            additional_properties: HashMap::new(),
        };

        // Set the basics
        resume.basics = Some(basics);

        // Convert work experience
        if !self.experience.is_empty() {
            let work_experiences: Vec<WorkExperience> = self
                .experience
                .iter()
                .map(|exp| exp.to_work_experience())
                .collect();
            resume.work = Some(work_experiences);
        }

        // Convert education
        if !self.education.is_empty() {
            let education_entries: Vec<crate::json_resume::Education> = self
                .education
                .iter()
                .map(|edu| edu.to_education())
                .collect();
            resume.education = Some(education_entries);
        }

        // Convert skills
        if !self.skills.is_empty() {
            let skill_entries: Vec<crate::json_resume::Skill> =
                self.skills.iter().map(|skill| skill.to_skill()).collect();
            resume.skills = Some(skill_entries);
        }

        resume
    }
}

fn to_jsonresume_basics(contact_info: &types::ContactInfo) -> Basics {
    // Create basics with contact info
    let profiles: Option<Vec<crate::json_resume::Profile>> =
        if !contact_info.websites.is_empty() || !contact_info.twitter.is_empty() {
            let mut profile_list = Vec::new();

            // Add websites as profiles
            for website in &contact_info.websites {
                if let Ok(url) = Url::parse(&website.url) {
                    profile_list.push(crate::json_resume::Profile {
                        network: website
                            .label
                            .clone()
                            .or_else(|| Some("Website".to_string())),
                        username: None,
                        url: Some(url),
                        additional_properties: HashMap::new(),
                    });
                }
            }

            // Add Twitter profiles
            for twitter_handle in &contact_info.twitter {
                profile_list.push(crate::json_resume::Profile {
                    network: Some("Twitter".to_string()),
                    username: Some(twitter_handle.clone()),
                    url: Url::parse(&format!("https://twitter.com/{}", twitter_handle)).ok(),
                    additional_properties: HashMap::new(),
                });
            }

            if profile_list.is_empty() {
                None
            } else {
                Some(profile_list)
            }
        } else {
            None
        };

    Basics {
        name: None, // Not available in ContactInfo
        label: None,
        image: None,
        email: contact_info.email_address.clone(),
        phone: contact_info.phone_numbers.first().cloned(),
        url: None,
        summary: None,
        location: None,
        profiles,
        additional_properties: HashMap::new(),
    }
}

/// Convert LinkedIn Experience to JSON Resume WorkExperience
fn to_jsonresume_work_experience(work_experience: &types::Experience) -> WorkExperience {
    WorkExperience {
        name: work_experience.company_name.clone(),
        location: None, // Not available in basic experience
        description: work_experience.description.clone(),
        position: work_experience.title.clone(),
        url: None,        // Could be derived from company info if available
        start_date: None, // Not available in basic LinkedIn experience struct
        end_date: None,   // Not available in basic LinkedIn experience struct
        summary: work_experience.description.clone(),
        highlights: work_experience
            .description
            .as_ref()
            .map(|desc| vec![desc.clone()]), // Simple conversion, could be enhanced
        additional_properties: HashMap::new(),
    }
}

/// Convert LinkedIn Education to JSON Resume Education
fn to_jsonresume_education(education: &types::Education) -> crate::json_resume::Education {
    crate::json_resume::Education {
        institution: education.school_name.clone(),
        url: None, // Not available in basic education
        area: education.field_of_study.clone(),
        study_type: education.degree.clone(),
        start_date: None, // Not available in basic LinkedIn education struct
        end_date: None,   // Not available in basic LinkedIn education struct
        score: None,      // Not available in basic LinkedIn education struct
        courses: None,    // Not available in basic LinkedIn education struct
        additional_properties: HashMap::new(),
    }
}

/// Convert LinkedIn Skill to JSON Resume Skill
fn to_jsonresume_skill(skill: &types::Skill) -> crate::json_resume::Skill {
    crate::json_resume::Skill {
        name: Some(skill.name.clone()),
        level: None,    // Not available in basic LinkedIn skill
        keywords: None, // Could be derived or set to skill name
        additional_properties: HashMap::new(),
    }
}
