use html::{
    SectioningContent,
    content::{Heading1, Heading2, Heading3, Section},
    inline_text::{Anchor, Span},
    text_content::{Division as Div, ListItem, Paragraph, UnorderedList},
};
use svg::{
    Document,
    node::element::{Circle, Path, Polyline},
};

use crate::json_resume::{
    Award, Basics, Certificate, Education, Interest, Language, Meta, Project, Publication,
    Reference, Resume, Skill, VolunteerExperience, WorkExperience,
};

use crate::ToHTMLResume;

// Helper function to create SVG icons
fn create_svg_icon(icon_type: &str) -> String {
    let document = match icon_type {
        "location" => Document::new()
            .set("viewBox", (0, 0, 24, 24))
            .set("width", 18)
            .set("height", 18)
            .set("stroke", "currentColor")
            .set("stroke-width", 2)
            .set("fill", "none")
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .add(
                Path::new().set(
                    "d",
                    "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z",
                ),
            )
            .add(Circle::new().set("cx", 12).set("cy", 10).set("r", 3)),

        "email" => Document::new()
            .set("viewBox", (0, 0, 24, 24))
            .set("width", 18)
            .set("height", 18)
            .set("stroke", "currentColor")
            .set("stroke-width", 2)
            .set("fill", "none")
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .add(
                Path::new().set(
                    "d",
                    "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z",
                ),
            )
            .add(Polyline::new().set("points", "22,6 12,13 2,6")),

        "phone" => Document::new()
            .set("viewBox", (0, 0, 24, 24))
            .set("width", 18)
            .set("height", 18)
            .set("stroke", "currentColor")
            .set("stroke-width", 2)
            .set("fill", "none")
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .add(Path::new().set("d", "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z")),

        "website" => Document::new()
            .set("viewBox", (0, 0, 24, 24))
            .set("width", 18)
            .set("height", 18)
            .set("stroke", "currentColor")
            .set("stroke-width", 2)
            .set("fill", "none")
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .add(Path::new().set("d", "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"))
            .add(Path::new().set("d", "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71")),

        _ => Document::new().set("viewBox", (0, 0, 24, 24)),
    };

    document.to_string()
}

pub struct ResumeBuilder;

impl ToHTMLResume for ResumeBuilder {
    fn build_basics(basics: Option<Basics>) -> Option<Section> {
        let basics = basics?;

        let mut cur_section = Section::builder();
        cur_section.class("header");

        // Add name
        if let Some(name) = &basics.name {
            cur_section.push(Heading1::builder().text(name.clone()).build());
        }

        // Add label
        if let Some(label) = &basics.label {
            cur_section.push(Paragraph::builder().text(label.clone()).build());
        }

        // Contact info container
        let mut contact_info = Div::builder();
        contact_info.class("contact-info");

        // Location
        if let Some(location) = &basics.location {
            if location.city.is_some() || location.country_code.is_some() {
                let mut contact_item = Div::builder();
                contact_item.class("contact-item");

                // Add location SVG (inline as raw HTML for simplicity)
                let svg_html = create_svg_icon("location");
                contact_item.text(svg_html);

                let location_text = format!(
                    "{}, {}",
                    location.city.as_deref().unwrap_or(""),
                    location.country_code.as_deref().unwrap_or("")
                );
                contact_item.text(location_text);
                contact_info.push(contact_item.build());
            }
        }

        // Email
        if let Some(email) = &basics.email {
            let mut contact_item = Div::builder();
            contact_item.class("contact-item");
            let svg_html = create_svg_icon("email");
            contact_item.text(svg_html);

            let email_link = Anchor::builder()
                .href(format!("mailto:{}", email))
                .text(email.clone())
                .build();
            contact_item.push(email_link);
            contact_info.push(contact_item.build());
        }

        // Phone
        if let Some(phone) = &basics.phone {
            let mut contact_item = Div::builder();
            contact_item.class("contact-item");
            let svg_html = create_svg_icon("phone");
            contact_item.text(svg_html);
            contact_item.text(phone.clone());
            contact_info.push(contact_item.build());
        }

        // Website
        if let Some(url) = &basics.url {
            let mut contact_item = Div::builder();
            contact_item.class("contact-item");
            let svg_html = create_svg_icon("website");
            contact_item.text(svg_html);

            let website_link = Anchor::builder()
                .href(url.to_string())
                .target("_blank")
                .text(url.to_string())
                .build();
            contact_item.push(website_link);
            contact_info.push(contact_item.build());
        }

        cur_section.push(contact_info.build());

        // Summary
        if let Some(summary) = &basics.summary {
            cur_section.push(Paragraph::builder().text(summary.clone()).build());
        }

        Some(cur_section.build())
    }

    fn build_work(work: Option<Vec<WorkExperience>>) -> Option<Section> {
        let work_experiences = work?;
        if work_experiences.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("work-section");

        cur_section.push(Heading2::builder().text("Work Experience").build());

        for experience in work_experiences {
            let mut work_item = Div::builder();
            work_item.class("work-item");

            if let Some(name) = &experience.name {
                work_item.push(Heading3::builder().text(name.clone()).build());
            }

            if let Some(position) = &experience.position {
                work_item.push(
                    Paragraph::builder()
                        .class("position")
                        .text(position.clone())
                        .build(),
                );
            }

            // Date range
            let mut date_parts = Vec::new();
            if let Some(start) = &experience.start_date {
                date_parts.push(start.clone());
            }
            if let Some(end) = &experience.end_date {
                date_parts.push(format!("to {}", end));
            } else if experience.start_date.is_some() {
                date_parts.push("to Present".to_string());
            }

            if !date_parts.is_empty() {
                work_item.push(
                    Paragraph::builder()
                        .class("dates")
                        .text(date_parts.join(" "))
                        .build(),
                );
            }

            if let Some(summary) = &experience.summary {
                work_item.push(Paragraph::builder().text(summary.clone()).build());
            }

            // Highlights
            if let Some(highlights) = &experience.highlights {
                if !highlights.is_empty() {
                    let mut highlights_list = UnorderedList::builder();
                    for highlight in highlights {
                        highlights_list.push(ListItem::builder().text(highlight.clone()).build());
                    }
                    work_item.push(highlights_list.build());
                }
            }

            cur_section.push(work_item.build());
        }

        Some(cur_section.build())
    }

    fn build_volunteer(volunteer: Option<Vec<VolunteerExperience>>) -> Option<Section> {
        let volunteer_experiences = volunteer?;
        if volunteer_experiences.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("volunteer-section");
        cur_section.push(Heading2::builder().text("Volunteer Experience").build());

        for experience in volunteer_experiences {
            let mut volunteer_item = Div::builder();
            volunteer_item.class("volunteer-item");

            if let Some(organization) = &experience.organization {
                volunteer_item.push(Heading3::builder().text(organization.clone()).build());
            }

            if let Some(position) = &experience.position {
                volunteer_item.push(
                    Paragraph::builder()
                        .class("position")
                        .text(position.clone())
                        .build(),
                );
            }

            if let Some(summary) = &experience.summary {
                volunteer_item.push(Paragraph::builder().text(summary.clone()).build());
            }

            cur_section.push(volunteer_item.build());
        }

        Some(cur_section.build())
    }

    fn build_education(education: Option<Vec<Education>>) -> Option<Section> {
        let education_items = education?;
        if education_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("education-section");

        cur_section.push(Heading2::builder().text("Education").build());

        for edu in education_items {
            let mut edu_item = Div::builder();
            edu_item.class("education-item");

            if let Some(institution) = &edu.institution {
                edu_item.push(Heading3::builder().text(institution.clone()).build());
            }

            let mut degree_info = Vec::new();
            if let Some(study_type) = &edu.study_type {
                degree_info.push(study_type.clone());
            }
            if let Some(area) = &edu.area {
                degree_info.push(format!("in {}", area));
            }

            if !degree_info.is_empty() {
                edu_item.push(
                    Paragraph::builder()
                        .class("degree")
                        .text(degree_info.join(" "))
                        .build(),
                );
            }

            if let Some(score) = &edu.score {
                edu_item.push(
                    Paragraph::builder()
                        .class("score")
                        .text(format!("Score: {}", score))
                        .build(),
                );
            }

            cur_section.push(edu_item.build());
        }

        Some(cur_section.build())
    }

    fn build_awards(awards: Option<Vec<Award>>) -> Option<Section> {
        let award_items = awards?;
        if award_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("awards-section");

        cur_section.push(Heading2::builder().text("Awards").build());

        let mut awards_list = UnorderedList::builder();
        for award in award_items {
            let mut award_text = String::new();
            if let Some(title) = &award.title {
                award_text.push_str(title);
            }
            if let Some(awarder) = &award.awarder {
                award_text.push_str(&format!(" - {}", awarder));
            }
            if let Some(date) = &award.date {
                award_text.push_str(&format!(" ({})", date));
            }

            awards_list.push(ListItem::builder().text(award_text).build());
        }
        cur_section.push(awards_list.build());

        Some(cur_section.build())
    }

    fn build_certificates(certificates: Option<Vec<Certificate>>) -> Option<Section> {
        let cert_items = certificates?;
        if cert_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("certificates-section");

        cur_section.push(Heading2::builder().text("Certificates").build());

        let mut certs_list = UnorderedList::builder();
        for cert in cert_items {
            let mut cert_text = String::new();
            if let Some(name) = &cert.name {
                cert_text.push_str(name);
            }
            if let Some(issuer) = &cert.issuer {
                cert_text.push_str(&format!(" - {}", issuer));
            }

            certs_list.push(ListItem::builder().text(cert_text).build());
        }
        cur_section.push(certs_list.build());

        Some(cur_section.build())
    }

    fn build_publications(publications: Option<Vec<Publication>>) -> Option<Section> {
        let pub_items = publications?;
        if pub_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("publications-section");

        cur_section.push(Heading2::builder().text("Publications").build());

        let mut pubs_list = UnorderedList::builder();
        for pub_item in pub_items {
            let mut pub_text = String::new();
            if let Some(name) = &pub_item.name {
                pub_text.push_str(name);
            }
            if let Some(publisher) = &pub_item.publisher {
                pub_text.push_str(&format!(" - {}", publisher));
            }

            pubs_list.push(ListItem::builder().text(pub_text).build());
        }
        cur_section.push(pubs_list.build());

        Some(cur_section.build())
    }

    fn build_skills(skills: Option<Vec<Skill>>) -> Option<Section> {
        let skill_items = skills?;
        if skill_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("skills-section");
        cur_section.push(Heading2::builder().text("Skills").build());

        for skill in skill_items {
            let mut skill_div = Div::builder();
            skill_div.class("skill-item");

            if let Some(name) = &skill.name {
                skill_div.push(
                    Span::builder()
                        .class("skill-name")
                        .text(name.clone())
                        .build(),
                );
            }

            if let Some(level) = &skill.level {
                skill_div.push(
                    Span::builder()
                        .class("skill-level")
                        .text(format!(" ({})", level))
                        .build(),
                );
            }

            if let Some(keywords) = &skill.keywords {
                if !keywords.is_empty() {
                    skill_div.push(
                        Paragraph::builder()
                            .class("skill-keywords")
                            .text(keywords.join(", "))
                            .build(),
                    );
                }
            }

            cur_section.push(skill_div.build());
        }

        Some(cur_section.build())
    }

    fn build_languages(languages: Option<Vec<Language>>) -> Option<Section> {
        let lang_items = languages?;
        if lang_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("languages-section");
        cur_section.push(Heading2::builder().text("Languages").build());

        let mut langs_list = UnorderedList::builder();
        for lang in lang_items {
            let mut lang_text = String::new();
            if let Some(language) = &lang.language {
                lang_text.push_str(language);
            }
            if let Some(fluency) = &lang.fluency {
                lang_text.push_str(&format!(" - {}", fluency));
            }

            langs_list.push(ListItem::builder().text(lang_text).build());
        }
        cur_section.push(langs_list.build());

        Some(cur_section.build())
    }

    fn build_interests(interests: Option<Vec<Interest>>) -> Option<Section> {
        let interest_items = interests?;
        if interest_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("interests-section");
        cur_section.push(Heading2::builder().text("Interests").build());

        let mut interests_list = UnorderedList::builder();
        for interest in interest_items {
            if let Some(name) = &interest.name {
                interests_list.push(ListItem::builder().text(name.clone()).build());
            }
        }
        cur_section.push(interests_list.build());

        Some(cur_section.build())
    }

    fn build_references(references: Option<Vec<Reference>>) -> Option<Section> {
        let ref_items = references?;
        if ref_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("references-section");

        cur_section.push(Heading2::builder().text("References").build());

        for reference in ref_items {
            let mut ref_div = Div::builder();
            ref_div.class("reference-item");

            if let Some(name) = &reference.name {
                ref_div.push(Heading3::builder().text(name.clone()).build());
            }

            if let Some(ref_text) = &reference.reference {
                ref_div.push(Paragraph::builder().text(ref_text.clone()).build());
            }

            cur_section.push(ref_div.build());
        }

        Some(cur_section.build())
    }

    fn build_projects(projects: Option<Vec<Project>>) -> Option<Section> {
        let project_items = projects?;
        if project_items.is_empty() {
            return None;
        }

        let mut cur_section = Section::builder();
        cur_section.class("projects-section");

        cur_section.push(Heading2::builder().text("Projects").build());

        for project in project_items {
            let mut project_div = Div::builder();
            project_div.class("project-item");

            if let Some(name) = &project.name {
                project_div.push(Heading3::builder().text(name.clone()).build());
            }

            if let Some(description) = &project.description {
                project_div.push(Paragraph::builder().text(description.clone()).build());
            }

            if let Some(highlights) = &project.highlights {
                if !highlights.is_empty() {
                    let mut highlights_list = UnorderedList::builder();
                    for highlight in highlights {
                        highlights_list.push(ListItem::builder().text(highlight.clone()).build());
                    }
                    project_div.push(highlights_list.build());
                }
            }

            cur_section.push(project_div.build());
        }

        Some(cur_section.build())
    }

    fn build_side_projects(projects: Option<Vec<Project>>) -> Option<Section> {
        // Same as build_projects but with different heading
        let project_items = projects?;
        if project_items.is_empty() {
            return None;
        }

        let mut section_div = Section::builder();
        section_div.class("side-projects-section");

        section_div.push(Heading2::builder().text("Side Projects").build());

        // Rest is identical to build_projects...
        for project in project_items {
            let mut project_div = Div::builder();
            project_div.class("project-item");

            if let Some(name) = &project.name {
                project_div.push(Heading3::builder().text(name.clone()).build());
            }

            if let Some(description) = &project.description {
                project_div.push(Paragraph::builder().text(description.clone()).build());
            }

            section_div.push(project_div.build());
        }

        Some(section_div.build())
    }

    fn build_meta(meta: Option<Meta>) -> Option<Section> {
        let meta_info = meta?;

        let mut cur_section = Section::builder();
        cur_section.class("meta-section");

        if let Some(version) = &meta_info.version {
            cur_section.push(
                Paragraph::builder()
                    .class("version")
                    .text(format!("Version: {}", version))
                    .build(),
            );
        }

        if let Some(last_modified) = &meta_info.last_modified {
            cur_section.push(
                Paragraph::builder()
                    .class("last-modified")
                    .text(format!("Last Modified: {}", last_modified))
                    .build(),
            );
        }

        Some(cur_section.build())
    }
}
