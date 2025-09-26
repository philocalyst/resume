#![recursion_limit = "512"]

use std::collections::HashMap;

use html::{content::Section, root::Html};
use typst;

use crate::{
    example::ResumeBuilder,
    json_resume::{
        Award, Basics, Certificate, Education, Interest, Language, Location, Meta, Project,
        Publication, Reference, Resume, Skill, VolunteerExperience, WorkExperience,
    },
};
mod example;
mod from_linkedin;
mod json_resume;

trait ToHTMLResume {
    fn build_basics(basics: Option<Basics>) -> Option<Section>;
    fn build_work(work: Option<Vec<WorkExperience>>) -> Option<Section>;
    fn build_volunteer(volunteer: Option<Vec<VolunteerExperience>>) -> Option<Section>;
    fn build_education(education: Option<Vec<Education>>) -> Option<Section>;
    fn build_awards(awards: Option<Vec<Award>>) -> Option<Section>;
    fn build_certificates(certificates: Option<Vec<Certificate>>) -> Option<Section>;
    fn build_publications(publications: Option<Vec<Publication>>) -> Option<Section>;
    fn build_skills(skills: Option<Vec<Skill>>) -> Option<Section>;
    fn build_languages(languages: Option<Vec<Language>>) -> Option<Section>;
    fn build_interests(interests: Option<Vec<Interest>>) -> Option<Section>;
    fn build_references(references: Option<Vec<Reference>>) -> Option<Section>;
    fn build_projects(projects: Option<Vec<Project>>) -> Option<Section>;
    fn build_side_projects(projects: Option<Vec<Project>>) -> Option<Section>;
    fn build_meta(meta: Option<Meta>) -> Option<Section>;
}

impl ToHTMLResume {
    /// Creates a full, HTML validated resume out of all of the information provided.
    pub fn build_resume(resume: Resume) -> Html {
        let full = Html::builder();

        full.push(build_basics(resume.basics));
        full.push(build_work(resume.work));
        full.push(build_volunteer(resume.volunteer));
        full.push(build_education(resume.education));
        full.push(build_awards(resume.awards));
        full.push(build_certificates(resume.certificates));
        full.push(build_publications(resume.publications));
        full.push(build_skills(resume.skills));
        full.push(build_languages(resume.languages));
        full.push(build_interests(resume.interests));
        full.push(build_references(resume.references));
        full.push(build_projects(resume.projects));
        full.push(build_side_projects(resume.projects));
        full.push(build_meta(resume.meta));

        full.build()
    }
}

trait ToTypstResume {
    fn build_resume<'a>(resume: Resume) -> typst::syntax::ast::Markup<'a>;
}

fn main() {
    let basics = Some(Basics {
        name: Some("John Doe".to_string()),
        label: Some("Software Engineer".to_string()),
        email: Some("john@example.com".to_string()),
        phone: Some("+1-555-0123".to_string()),
        url: Some("https://johndoe.dev".parse().unwrap()),
        summary: Some(
            "Experienced software engineer with a passion for building scalable systems."
                .to_string(),
        ),
        location: Some(Location {
            city: Some("San Francisco".to_string()),
            country_code: Some("US".to_string()),
            address: None,
            postal_code: None,
            region: None,
            additional_properties: HashMap::default(),
        }),
        image: None,
        profiles: None,
        additional_properties: HashMap::default(),
    });

    if let Some(basics_section) = ResumeBuilder::build_basics(basics) {
        println!("{}", basics_section);
    };
}
