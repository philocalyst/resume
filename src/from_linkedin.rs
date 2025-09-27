use crate::json_resume::*;
use linkedin_api::*;
use std::collections::HashMap;
use url::Url;

/// Trait for converting LinkedIn data structures to JSON Resume format
pub trait ToJsonResume {
    fn to_json_resume(&self) -> Resume;
}

impl ToJsonResume for linkedin_api::Profile {
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
            resume.work = Some(work_experiences;
        }

        // Convert education
        if !self.education.is_empty() {
            let education_entries: Vec<crate::json_resume_types::Education> = self
                .education
                .iter()
                .map(|edu| edu.to_education())
                .collect();
            resume.education = Some(education_entries);
        }

        // Convert skills
        if !self.skills.is_empty() {
            let skill_entries: Vec<crate::json_resume_types::Skill> =
                self.skills.iter().map(|skill| skill.to_skill()).collect();
            resume.skills = Some(skill_entries);
        }

        resume
    }
}

impl linkedin_api::Profile {
    /// Helper method to get full name
    fn get_full_name(&self) -> Option<String> {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            (None, None) => None,
        }
    }

    /// Helper method to get profile image URL
    fn get_profile_image_url(&self) -> Option<Url> {
        self.profile_picture_original_image
            .as_ref()
            .and_then(|container| container.vector_image.as_ref())
            .and_then(|vector_image| {
                if let (Some(root_url), Some(artifact)) =
                    (&vector_image.root_url, vector_image.artifacts.first())
                {
                    let full_url = format!(
                        "{}{}",
                        root_url,
                        artifact
                            .file_identifying_url_path_segment
                            .as_deref()
                            .unwrap_or("")
                    );
                    Url::parse(&full_url).ok()
                } else {
                    None
                }
            })
    }

    /// Helper method to convert LinkedIn location to JSON Resume location
    fn convert_location(&self) -> Option<crate::json_resume_types::Location> {
        if self.geo_location_name.is_some()
            || self.geo_country_name.is_some()
            || self.address.is_some()
        {
            Some(crate::json_resume_types::Location {
                address: self.address.clone(),
                postal_code: self
                    .location
                    .as_ref()
                    .and_then(|loc| loc.basic_location.as_ref())
                    .and_then(|basic| basic.postal_code.clone()),
                city: self.geo_location_name.clone(),
                country_code: self
                    .location
                    .as_ref()
                    .and_then(|loc| loc.basic_location.as_ref())
                    .and_then(|basic| basic.country_code.clone()),
                region: None, // Not directly available in LinkedIn profile
                additional_properties: HashMap::new(),
            })
        } else {
            None
        }
    }
}

impl linkedin_api::Experience {
    /// Convert LinkedIn Experience to JSON Resume WorkExperience
    fn to_work_experience(&self) -> WorkExperience {
        WorkExperience {
            name: self.company_name.clone(),
            location: None, // Not available in basic experience
            description: self.description.clone(),
            position: self.title.clone(),
            url: None,        // Could be derived from company info if available
            start_date: None, // Not available in basic LinkedIn experience struct
            end_date: None,   // Not available in basic LinkedIn experience struct
            summary: self.description.clone(),
            highlights: self.description.as_ref().map(|desc| vec![desc.clone()]), // Simple conversion, could be enhanced
            additional_properties: HashMap::new(),
        }
    }
}

impl linkedin_api::Education {
    /// Convert LinkedIn Education to JSON Resume Education
    fn to_education(&self) -> crate::json_resume_types::Education {
        crate::json_resume_types::Education {
            institution: self.school_name.clone(),
            url: None, // Not available in basic education
            area: self.field_of_study.clone(),
            study_type: self.degree.clone(),
            start_date: None, // Not available in basic LinkedIn education struct
            end_date: None,   // Not available in basic LinkedIn education struct
            score: None,      // Not available in basic LinkedIn education struct
            courses: None,    // Not available in basic LinkedIn education struct
            additional_properties: HashMap::new(),
        }
    }
}

impl linkedin_api::Skill {
    /// Convert LinkedIn Skill to JSON Resume Skill
    fn to_skill(&self) -> crate::json_resume_types::Skill {
        crate::json_resume_types::Skill {
            name: Some(self.name.clone()),
            level: None,    // Not available in basic LinkedIn skill
            keywords: None, // Could be derived or set to skill name
            additional_properties: HashMap::new(),
        }
    }
}

impl ToJsonResume for ContactInfo {
    fn to_json_resume(&self) -> Resume {
        let mut resume = Resume::new();

        // Create basics with contact info
        let profiles: Option<Vec<crate::json_resume_types::Profile>> =
            if !self.websites.is_empty() || !self.twitter.is_empty() {
                let mut profile_list = Vec::new();

                // Add websites as profiles
                for website in &self.websites {
                    if let Ok(url) = Url::parse(&website.url) {
                        profile_list.push(crate::json_resume_types::Profile {
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
                for twitter_handle in &self.twitter {
                    profile_list.push(crate::json_resume_types::Profile {
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

        let basics = Basics {
            name: None, // Not available in ContactInfo
            label: None,
            image: None,
            email: self.email_address.clone(),
            phone: self.phone_numbers.first().cloned(),
            url: None,
            summary: None,
            location: None,
            profiles,
            additional_properties: HashMap::new(),
        };

        resume.basics = Some(basics);
        resume
    }
}

impl linkedin_api::Experience {
    /// Convert LinkedIn Experience to JSON Resume WorkExperience
    fn to_work_experience(&self) -> WorkExperience {
        WorkExperience {
            name: self.company_name.clone(),
            location: None, // Not available in basic experience
            description: self.description.clone(),
            position: self.title.clone(),
            url: None, // Could be derived from company info if available
            start_date: None, // Not available in basic LinkedIn experience struct
            end_date: None,   // Not available in basic LinkedIn experience struct
            summary: self.description.clone(),
            highlights: self.description
                .as_ref()
                .map(|desc| vec![desc.clone()]), // Simple conversion, could be enhanced
            additional_properties: HashMap::new(),
        }
    }
}

impl linkedin_api::Education {
    /// Convert LinkedIn Education to JSON Resume Education
    fn to_education(&self) -> crate::json_resume_types::Education {
        crate::json_resume_types::Education {
            institution: self.school_name.clone(),
            url: None, // Not available in basic education
            area: self.field_of_study.clone(),
            study_type: self.degree.clone(),
            start_date: None, // Not available in basic LinkedIn education struct
            end_date: None,   // Not available in basic LinkedIn education struct
            score: None,      // Not available in basic LinkedIn education struct
            courses: None,    // Not available in basic LinkedIn education struct
            additional_properties: HashMap::new(),
        }
    }
}

impl linkedin_api::Skill {
    /// Convert LinkedIn Skill to JSON Resume Skill
    fn to_skill(&self) -> crate::json_resume_types::Skill {
        crate::json_resume_types::Skill {
            name: Some(self.name.clone()),
            level: None, // Not available in basic LinkedIn skill
            keywords: None, // Could be derived or set to skill name
            additional_properties: HashMap::new(),
        }
    }
}
