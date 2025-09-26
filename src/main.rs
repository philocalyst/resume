#![recursion_limit = "512"]

use html::content::Section;
mod json_resume;

trait HTMLResume {
    fn build_basics() -> Option<Section>;
    fn build_work() -> Option<Section>;
    fn build_volunteer() -> Option<Section>;
    fn build_education() -> Option<Section>;
    fn build_awards() -> Option<Section>;
    fn build_certificates() -> Option<Section>;
    fn build_publications() -> Option<Section>;
    fn build_skills() -> Option<Section>;
    fn build_languages() -> Option<Section>;
    fn build_interests() -> Option<Section>;
    fn build_references() -> Option<Section>;
    fn build_projects() -> Option<Section>;
    fn build_side_projects() -> Option<Section>;
    fn build_meta() -> Option<Section>;
}

fn main() {
    println!("Hello, world!");
}
