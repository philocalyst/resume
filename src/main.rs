#![recursion_limit = "512"]

use html::content::Section;
use json_resume;

trait HTMLResume {
    fn build_header() -> Section;
    fn build_interests() -> Section;
    fn build_languages() -> Section;
    fn build_references() -> Section;
    fn build_section() -> Section;
    fn build_skills() -> Section;
    fn build_summary() -> Section;
}

fn main() {
    println!("Hello, world!");
}
