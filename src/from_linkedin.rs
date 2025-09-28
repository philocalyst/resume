use crate::json_resume::*;
use linkedin_api::types;
use std::collections::HashMap;
use time::Month;
use url::Url;

/// Trait for converting LinkedIn data structures to JSON Resume format
pub trait ToJsonResume {
    fn to_json_resume(&self) -> Resume;
}

impl ToJsonResume for types::ProfileView {
    fn to_json_resume(&self) -> Resume {
        let mut resume = Resume::new();

        // Set JSON Resume schema
        resume.schema = Some(
            Url::parse(
                "https://raw.githubusercontent.com/jsonresume/resume-schema/v1.0.0/schema.json",
            )
            .unwrap(),
        );

        // Convert basics from profile
        resume.basics = Some(to_jsonresume_basics(&self.profile));

        // Convert work experience from position views
        let mut work_experiences = Vec::new();

        // Add from position groups
        for position_group in &self.position_group_view.elements {
            for position in &position_group.positions {
                work_experiences.push(to_jsonresume_work_experience(position));
            }
        }

        // Add from individual positions
        for position in &self.position_view.elements {
            work_experiences.push(to_jsonresume_work_experience(position));
        }

        if !work_experiences.is_empty() {
            resume.work = Some(work_experiences);
        }

        // Convert education
        if !self.education_view.elements.is_empty() {
            let education_entries: Vec<crate::json_resume::Education> = self
                .education_view
                .elements
                .iter()
                .map(|edu| to_jsonresume_education(edu))
                .collect();
            resume.education = Some(education_entries);
        }

        // Convert skills
        if !self.skill_view.elements.is_empty() {
            let skill_entries: Vec<crate::json_resume::Skill> = self
                .skill_view
                .elements
                .iter()
                .map(|skill| to_jsonresume_skill(skill))
                .collect();
            resume.skills = Some(skill_entries);
        }

        // Convert languages
        if !self.language_view.elements.is_empty() {
            let language_entries: Vec<crate::json_resume::Language> = self
                .language_view
                .elements
                .iter()
                .map(|lang| to_jsonresume_language(lang))
                .collect();
            resume.languages = Some(language_entries);
        }

        // Convert volunteer experience
        if !self.volunteer_experience_view.elements.is_empty() {
            let volunteer_entries: Vec<crate::json_resume::VolunteerExperience> = self
                .volunteer_experience_view
                .elements
                .iter()
                .map(|vol| to_jsonresume_volunteer(vol))
                .collect();
            resume.volunteer = Some(volunteer_entries);
        }

        // Convert awards/honors
        if !self.honor_view.elements.is_empty() {
            let award_entries: Vec<crate::json_resume::Award> = self
                .honor_view
                .elements
                .iter()
                .map(|honor| to_jsonresume_award(honor))
                .collect();
            resume.awards = Some(award_entries);
        }

        // Convert certifications
        if !self.certification_view.elements.is_empty() {
            let cert_entries: Vec<crate::json_resume::Certificate> = self
                .certification_view
                .elements
                .iter()
                .map(|cert| to_jsonresume_certificate(cert))
                .collect();
            resume.certificates = Some(cert_entries);
        }

        // Convert publications
        if !self.publication_view.elements.is_empty() {
            let pub_entries: Vec<crate::json_resume::Publication> = self
                .publication_view
                .elements
                .iter()
                .filter_map(|pub_val| to_jsonresume_publication(pub_val))
                .collect();
            if !pub_entries.is_empty() {
                resume.publications = Some(pub_entries);
            }
        }

        // Convert projects
        if !self.project_view.elements.is_empty() {
            let project_entries: Vec<crate::json_resume::Project> = self
                .project_view
                .elements
                .iter()
                .filter_map(|proj_val| to_jsonresume_project(proj_val))
                .collect();
            if !project_entries.is_empty() {
                resume.projects = Some(project_entries);
            }
        }

        // Convert interests from volunteer causes
        if !self.volunteer_cause_view.elements.is_empty() {
            let interest_entries: Vec<crate::json_resume::Interest> = self
                .volunteer_cause_view
                .elements
                .iter()
                .map(|cause| crate::json_resume::Interest {
                    name: Some(cause.cause_name.clone()),
                    keywords: Some(vec![cause.cause_type.clone()]),
                    additional_properties: HashMap::new(),
                })
                .collect();
            resume.interests = Some(interest_entries);
        }

        // Add test scores as additional certificates
        if !self.test_score_view.elements.is_empty() {
            let mut existing_certs = resume.certificates.unwrap_or_default();
            let test_cert_entries: Vec<crate::json_resume::Certificate> = self
                .test_score_view
                .elements
                .iter()
                .map(|test| to_jsonresume_test_score_as_certificate(test))
                .collect();
            existing_certs.extend(test_cert_entries);
            resume.certificates = Some(existing_certs);
        }

        // Convert courses as additional education entries
        if !self.course_view.elements.is_empty() {
            let mut existing_education = resume.education.unwrap_or_default();
            let course_entries: Vec<crate::json_resume::Education> = self
                .course_view
                .elements
                .iter()
                .map(|course| to_jsonresume_course_as_education(course))
                .collect();
            existing_education.extend(course_entries);
            resume.education = Some(existing_education);
        }

        // Set meta information
        resume.meta = Some(crate::json_resume::Meta {
            canonical: None,
            version: Some("1.0.0".to_string()),
            last_modified: Some(chrono::Utc::now().to_rfc3339()),
            additional_properties: HashMap::new(),
        });

        resume
    }
}

fn to_jsonresume_basics(profile: &types::Profile) -> Basics {
    // Build location from profile geo data
    let location = if profile.geo_country_name.is_some()
        || profile.geo_location_name.is_some()
        || profile.address.is_some()
    {
        Some(crate::json_resume::Location {
            address: profile.address.as_ref().map(|addr| addr.raw.clone()),
            postal_code: profile
                .geo_location
                .as_ref()
                .and_then(|geo| geo.postal_code.clone())
                .or_else(|| {
                    profile
                        .address
                        .as_ref()
                        .and_then(|addr| addr.postal_code.clone())
                }),
            city: profile
                .address
                .as_ref()
                .and_then(|addr| addr.city.clone())
                .or_else(|| profile.geo_location_name.clone()),
            country_code: profile.geo_country_name.as_ref().and_then(|country| {
                // Convert country name to country code if possible
                // This is a simplified conversion - you'd want a proper country name->code mapping
                match country.as_str() {
                    "United States" => Some("US".to_string()),
                    "Canada" => Some("CA".to_string()),
                    "United Kingdom" => Some("GB".to_string()),
                    "Germany" => Some("DE".to_string()),
                    "France" => Some("FR".to_string()),
                    _ => None,
                }
            }),
            region: profile.address.as_ref().and_then(|addr| addr.state.clone()),
            additional_properties: HashMap::new(),
        })
    } else {
        None
    };

    // Build profiles array
    let mut profiles = Vec::new();

    // Add LinkedIn profile
    if let Some(profile_id) = profile.get_profile_id() {
        profiles.push(crate::json_resume::Profile {
            network: Some("LinkedIn".to_string()),
            username: Some(profile_id.clone()),
            url: Url::parse(&format!("https://www.linkedin.com/in/{}", profile_id)).ok(),
            additional_properties: HashMap::new(),
        });
    }

    Basics {
        name: profile.get_full_name(),
        label: profile.headline.clone(),
        image: profile.get_profile_image_url(),
        email: None, // Will be filled from contact info if available
        phone: None, // Will be filled from contact info if available
        url: profile
            .get_profile_id()
            .and_then(|id| Url::parse(&format!("https://www.linkedin.com/in/{}", id)).ok()),
        summary: profile.summary.clone(),
        location,
        profiles: if profiles.is_empty() {
            None
        } else {
            Some(profiles)
        },
        additional_properties: HashMap::new(),
    }
}

fn to_jsonresume_work_experience(experience: &types::Experience) -> WorkExperience {
    let (start_date, end_date) = convert_time_period(&experience.time_period);

    // Parse highlights from description
    let (summary, highlights) = parse_description_and_highlights(&experience.description);

    WorkExperience {
        name: experience.company_name.clone(),
        location: experience
            .location_name
            .clone()
            .or_else(|| experience.geo_location_name.clone())
            .or_else(|| experience.region.clone()),
        description: experience.description.clone(),
        position: experience.title.clone(),
        url: experience
            .company
            .as_ref()
            .and_then(|company| company.mini_company.as_ref())
            .and_then(|mini| mini.universal_name.as_ref())
            .and_then(|name| {
                Url::parse(&format!("https://www.linkedin.com/company/{}", name)).ok()
            }),
        start_date,
        end_date,
        summary,
        highlights,
        additional_properties: {
            let mut props = HashMap::new();
            if let Some(company) = &experience.company {
                if let Some(employee_range) = &company.employee_count_range {
                    props.insert(
                        "companySize".to_string(),
                        serde_json::Value::String(format!(
                            "{}-{}",
                            employee_range.start, employee_range.end
                        )),
                    );
                }
                if !company.industries.is_empty() {
                    props.insert(
                        "industries".to_string(),
                        serde_json::Value::Array(
                            company
                                .industries
                                .iter()
                                .map(|i| serde_json::Value::String(i.clone()))
                                .collect(),
                        ),
                    );
                }
            }
            if let Some(logo_url) = experience.get_company_logo_url() {
                props.insert(
                    "companyLogo".to_string(),
                    serde_json::Value::String(logo_url.to_string()),
                );
            }
            props
        },
    }
}

fn to_jsonresume_education(education: &types::Education) -> crate::json_resume::Education {
    let (start_date, end_date) = convert_time_period(&education.time_period);

    // Parse activities into courses
    let activities = education.get_activities_list();
    let courses = if activities.is_empty() {
        None
    } else {
        Some(activities)
    };

    // Combine description and honors
    let mut description_parts = Vec::new();
    if let Some(desc) = &education.description {
        description_parts.push(desc.clone());
    }
    if let Some(honors) = &education.honors {
        if !honors.is_empty() {
            description_parts.push(format!("Honors: {}", honors.join(", ")));
        }
    }
    if let Some(test_scores) = &education.test_scores {
        if !test_scores.is_empty() {
            description_parts.push(format!("Test Scores: {}", test_scores.join(", ")));
        }
    }

    crate::json_resume::Education {
        institution: education.school_name.clone(),
        url: education.school.as_ref().and_then(|school| {
            Url::parse(&format!(
                "https://www.linkedin.com/school/{}",
                school
                    .entity_urn
                    .clone()
                    .unwrap()
                    .split(':')
                    .last()
                    .unwrap_or("")
            ))
            .ok()
        }),
        area: education.field_of_study.clone(),
        study_type: education.degree_name.clone(),
        start_date,
        end_date,
        score: education.grade.clone(),
        courses,
        additional_properties: {
            let mut props = HashMap::new();
            if let Some(logo_url) = education.get_school_logo_url() {
                props.insert(
                    "schoolLogo".to_string(),
                    serde_json::Value::String(logo_url.to_string()),
                );
            }
            if !description_parts.is_empty() {
                props.insert(
                    "description".to_string(),
                    serde_json::Value::String(description_parts.join("\n")),
                );
            }
            if let Some(activities) = &education.activities {
                props.insert(
                    "activities".to_string(),
                    serde_json::Value::String(activities.clone()),
                );
            }
            props
        },
    }
}

fn to_jsonresume_skill(skill: &types::Skill) -> crate::json_resume::Skill {
    crate::json_resume::Skill {
        name: Some(skill.name.clone()),
        level: None, // LinkedIn doesn't provide skill levels
        keywords: Some(vec![skill.name.clone()]),
        additional_properties: HashMap::new(),
    }
}

fn to_jsonresume_language(language: &types::Language) -> crate::json_resume::Language {
    let fluency = match language.proficiency.clone() {
        Some(prof) => match prof {
            types::LanguageProficiency::NativeOrBilingual => "Native",
            types::LanguageProficiency::FullProfessional => "Professional",
            types::LanguageProficiency::ProfessionalWorking => "Professional",
            types::LanguageProficiency::LimitedWorking => "Conversational",
            types::LanguageProficiency::Elementary => "Elementary",
        },
        None => "Unknown",
    };

    crate::json_resume::Language {
        language: Some(language.name.clone()),
        fluency: Some(fluency.to_string()),
        additional_properties: HashMap::new(),
    }
}

fn to_jsonresume_volunteer(
    volunteer: &types::VolunteerExperience,
) -> crate::json_resume::VolunteerExperience {
    let (start_date, end_date) = convert_time_period(&volunteer.time_period);
    let (summary, highlights) = parse_description_and_highlights(&volunteer.description);

    crate::json_resume::VolunteerExperience {
        organization: volunteer.company_name.clone(),
        position: Some(volunteer.role.clone()),
        url: volunteer
            .company
            .as_ref()
            .and_then(|company| company.mini_company.as_ref())
            .and_then(|mini| mini.universal_name.as_ref())
            .and_then(|name| {
                Url::parse(&format!("https://www.linkedin.com/company/{}", name)).ok()
            }),
        start_date,
        end_date,
        summary,
        highlights,
        additional_properties: {
            let mut props = HashMap::new();
            if let Some(cause) = &volunteer.cause {
                props.insert(
                    "cause".to_string(),
                    serde_json::Value::String(cause.clone()),
                );
            }
            if let Some(region) = &volunteer.region {
                props.insert(
                    "location".to_string(),
                    serde_json::Value::String(region.clone()),
                );
            }
            props
        },
    }
}

fn to_jsonresume_award(honor: &types::Honor) -> crate::json_resume::Award {
    let date = honor
        .issue_date
        .as_ref()
        .map(|date| format!("{}-{:02}", date.year, date.month as u8));

    crate::json_resume::Award {
        title: Some(honor.title.clone()),
        date,
        awarder: honor.issuer.clone(),
        summary: honor.description.clone(),
        additional_properties: {
            let mut props = HashMap::new();
            if let Some(occupation) = &honor.occupation {
                props.insert(
                    "occupation".to_string(),
                    serde_json::Value::String(occupation.clone()),
                );
            }
            props
        },
    }
}

fn to_jsonresume_certificate(cert: &types::Certification) -> crate::json_resume::Certificate {
    let date = cert
        .time_period
        .as_ref()
        .and_then(|tp| tp.start_date.as_ref())
        .map(|date| format!("{}-{:02}", date.year, date.month as u8));

    crate::json_resume::Certificate {
        name: Some(cert.name.clone()),
        date,
        url: cert.url.clone(),
        issuer: cert.authority.clone(),
        additional_properties: {
            let mut props = HashMap::new();
            if let Some(license_number) = &cert.license_number {
                props.insert(
                    "licenseNumber".to_string(),
                    serde_json::Value::String(license_number.clone()),
                );
            }
            if let Some(end_date) = cert
                .time_period
                .as_ref()
                .and_then(|tp| tp.end_date.as_ref())
            {
                props.insert(
                    "expiryDate".to_string(),
                    serde_json::Value::String(format!(
                        "{}-{:02}",
                        end_date.year, end_date.month as u8
                    )),
                );
            }
            props
        },
    }
}

fn to_jsonresume_test_score_as_certificate(
    test: &types::TestScore,
) -> crate::json_resume::Certificate {
    let date = test
        .date
        .as_ref()
        .map(|date| format!("{}-{:02}", date.year, date.month as u8));

    crate::json_resume::Certificate {
        name: Some(test.name.clone()),
        date,
        url: None,
        issuer: None, // Test scores don't typically have issuers in the same way
        additional_properties: {
            let mut props = HashMap::new();
            props.insert(
                "score".to_string(),
                serde_json::Value::String(test.score.clone()),
            );
            props.insert(
                "type".to_string(),
                serde_json::Value::String("test_score".to_string()),
            );
            if let Some(description) = &test.description {
                props.insert(
                    "description".to_string(),
                    serde_json::Value::String(description.clone()),
                );
            }
            if let Some(occupation) = &test.occupation {
                props.insert(
                    "occupation".to_string(),
                    serde_json::Value::String(occupation.clone()),
                );
            }
            props
        },
    }
}

fn to_jsonresume_course_as_education(course: &types::Course) -> crate::json_resume::Education {
    crate::json_resume::Education {
        institution: None,
        url: None,
        area: Some(course.name.clone()),
        study_type: Some("Course".to_string()),
        start_date: None,
        end_date: None,
        score: None,
        courses: Some(vec![course.name.clone()]),
        additional_properties: {
            let mut props = HashMap::new();
            if let Some(number) = &course.number {
                props.insert(
                    "courseNumber".to_string(),
                    serde_json::Value::String(number.clone()),
                );
            }
            props.insert(
                "type".to_string(),
                serde_json::Value::String("course".to_string()),
            );
            props
        },
    }
}

fn to_jsonresume_publication(
    pub_val: &serde_json::Value,
) -> Option<crate::json_resume::Publication> {
    // LinkedIn publications are stored as generic JSON values
    // This would need to be implemented based on the actual structure
    None
}

fn to_jsonresume_project(proj_val: &types::Project) -> Option<crate::json_resume::Project> {
    // LinkedIn projects are stored as generic JSON values
    // This would need to be implemented based on the actual structure
    None
}

// Helper functions

fn convert_time_period(
    time_period: &Option<types::TimePeriod>,
) -> (Option<String>, Option<String>) {
    match time_period {
        Some(tp) => {
            let start_date = tp
                .start_date
                .as_ref()
                .map(|date| format!("{}-{:02}", date.year, date.month as u8));
            let end_date = tp
                .end_date
                .as_ref()
                .map(|date| format!("{}-{:02}", date.year, date.month as u8));
            (start_date, end_date)
        }
        None => (None, None),
    }
}

fn parse_description_and_highlights(
    description: &Option<String>,
) -> (Option<String>, Option<Vec<String>>) {
    match description {
        Some(desc) => {
            // Split description into summary and highlights
            // Look for bullet points, numbered lists, etc.
            let lines: Vec<&str> = desc.lines().collect();
            let mut summary_lines = Vec::new();
            let mut highlight_lines = Vec::new();
            let mut in_highlights = false;

            for line in lines {
                let trimmed = line.trim();
                if trimmed.starts_with('•') || trimmed.starts_with('-') || trimmed.starts_with('*')
                {
                    in_highlights = true;
                    highlight_lines.push(
                        trimmed
                            .trim_start_matches(['•', '-', '*'])
                            .trim()
                            .to_string(),
                    );
                } else if trimmed
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_numeric() && trimmed.contains('.'))
                {
                    in_highlights = true;
                    // Remove numbering from highlights
                    if let Some(pos) = trimmed.find('.') {
                        highlight_lines.push(trimmed[pos + 1..].trim().to_string());
                    }
                } else if !in_highlights && !trimmed.is_empty() {
                    summary_lines.push(trimmed);
                } else if in_highlights && !trimmed.is_empty() {
                    highlight_lines.push(trimmed.to_string());
                }
            }

            let summary = if summary_lines.is_empty() {
                Some(desc.clone())
            } else {
                Some(summary_lines.join(" "))
            };

            let highlights = if highlight_lines.is_empty() {
                None
            } else {
                Some(highlight_lines)
            };

            (summary, highlights)
        }
        None => (None, None),
    }
}
