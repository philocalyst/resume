#![recursion_limit = "512"]

use std::collections::HashMap;

use html::{content::Section, root::Html};
use typst;

use crate::{
    example::ResumeBuilder,
    from_linkedin::ToJsonResume,
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
    fn build_meta(meta: Option<Meta>) -> Option<Section>;

    /// Creates a full, HTML validated resume out of all of the information provided.
    /// This is a default implementation that can be overridden by implementors.
    fn build_resume(resume: Resume) -> Html {
        let mut full = Html::builder();
        let mut body = html::root::Body::builder();
        full.push(body.build());

        if let Some(section) = Self::build_basics(resume.basics) {
            body.push(section);
        }
        if let Some(section) = Self::build_work(resume.work) {
            body.push(section);
        }
        if let Some(section) = Self::build_volunteer(resume.volunteer) {
            body.push(section);
        }
        if let Some(section) = Self::build_education(resume.education) {
            body.push(section);
        }
        if let Some(section) = Self::build_awards(resume.awards) {
            body.push(section);
        }
        if let Some(section) = Self::build_certificates(resume.certificates) {
            body.push(section);
        }
        if let Some(section) = Self::build_publications(resume.publications) {
            body.push(section);
        }
        if let Some(section) = Self::build_skills(resume.skills) {
            body.push(section);
        }
        if let Some(section) = Self::build_languages(resume.languages) {
            body.push(section);
        }
        if let Some(section) = Self::build_interests(resume.interests) {
            body.push(section);
        }
        if let Some(section) = Self::build_references(resume.references) {
            body.push(section);
        }
        if let Some(section) = Self::build_projects(resume.projects) {
            body.push(section);
        }
        if let Some(section) = Self::build_meta(resume.meta) {
            body.push(section);
        }

        full.build()
    }
}

trait ToTypstResume {
    fn build_resume<'a>(resume: Resume) -> typst::syntax::ast::Markup<'a>;
}

use linkedin_api::{Linkedin, LinkedinError, types::Identity};

#[tokio::main]
async fn main() -> Result<(), LinkedinError> {
    let input = Identity {
        authentication_token: String::from(
            "AQEDAUEPdx8FJo2CAAABmXMEsS0AAAGZlxE1LU4Aclty_bQmV4p4VWnlBVAerIOntfpKC8rMVg107RrypH6OLlgHUK0PqJ5Nssev_4lzITN-GptrMsPInTcSfuKKQwQAqJNEjhM9sWywSaYzvoobkkoc",
        ),
        session_cookie: String::from("ajax:8702309092900260000"),
    };

    let api = Linkedin::new(&input, false).await?;

    let profile = api.get_profile("miles-wirht-b3b675265").await?;

    let json_resume = profile.to_json_resume();

    let resume = ResumeBuilder::build_resume(json_resume);

    println!("{}", resume.to_string());

    Ok(())
}
