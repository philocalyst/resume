#![recursion_limit = "512"]

use html::content::Section;
mod json_resume;

trait HTMLResume {
    fn build_basics() -> Section;
    fn build_work() -> Section;
    fn build_volunteer() -> Section;
    fn build_education() -> Section;
    fn build_awards() -> Section;
    fn build_certificates() -> Section;
    fn build_publications() -> Section;
    fn build_skills() -> Section;
    fn build_languages() -> Section;
    fn build_interests() -> Section;
    fn build_references() -> Section;
    fn build_projects() -> Section;
    fn build_side_projects() -> Section;
    fn build_meta() -> Section;
}

fn main() {
    println!("Hello, world!");
}
