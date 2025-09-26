#![recursion_limit = "512"]

use html::content::Section;
use typst;

use crate::json_resume::{
    Award, Basics, Certificate, Education, Interest, Language, Meta, Project, Publication,
    Reference, Resume, Skill, VolunteerExperience, WorkExperience,
};
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

trait ToTypstResume {
    fn build_resume(resume: Resume) -> typst::Syntax::ast;
}

fn main() {
    println!("Hello, world!");
}
