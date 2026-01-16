use derive_typst_intoval::{IntoDict, IntoValue};
use std::{collections::HashMap, fs};
use typst::foundations::{Bytes, Dict, IntoValue};
use typst_as_lib::{TypstEngine, typst_kit_options::TypstKitFontOptions};
use url::Url;

use crate::json_resume::{
    Basics, Certificate, Education, Interest, Language, Location, Meta, Profile, Project, Resume,
    Skill, WorkExperience,
};

mod json_resume;

static TEMPLATE_FILE: &str = include_str!("./templates/template.typ");
static FONT: &[u8] = include_bytes!("./fonts/texgyrecursor-regular.otf");
static OUTPUT: &str = "./output.pdf";
static IMAGE: &[u8] = include_bytes!("./templates/images/typst.png");

fn main() {
    // Read in fonts and the main source file.
    // We can use this template more than once, if needed (Possibly
    // with different input each time).
    let template = TypstEngine::builder()
        .main_file(TEMPLATE_FILE)
        .search_fonts_with(
            TypstKitFontOptions::default()
                .include_system_fonts(false)
                .include_embedded_fonts(true),
        )
        .with_package_file_resolver()
        .build();

    // Run it
    let doc = template
        .compile_with_input(get_dummy_resume())
        .output
        .expect("typst::compile() returned an error!");

    // Create pdf
    let options = Default::default();
    let pdf = typst_pdf::pdf(&doc, &options).expect("Could not generate pdf.");
    fs::write(OUTPUT, pdf).expect("Could not write pdf.");
}

pub fn get_dummy_resume() -> Resume {
    Resume {
        schema: Some(
            Url::parse(
                "https://raw.githubusercontent.com/jsonresume/resume-schema/v1.0.0/schema.json",
            )
            .unwrap(),
        ),
        basics: Some(Basics {
            name: Some("Jane Doe".to_string()),
            label: Some("Systems Architect".to_string()),
            image: Some(Url::parse("https://example.com/avatar.png").unwrap()),
            email: Some("jane.doe@example.com".to_string()),
            phone: Some("+1-555-0199".to_string()),
            url: Some(Url::parse("https://janedoe.dev").unwrap()),
            summary: Some(
                "Experienced engineer focused on memory safety and distributed systems."
                    .to_string(),
            ),
            location: Some(Location {
                address: Some("123 Rust Lane".to_string()),
                postal_code: Some("10001".to_string()),
                city: Some("New York".to_string()),
                country_code: Some("US".to_string()),
                region: Some("NY".to_string()),
                additional_properties: HashMap::new(),
            }),
            profiles: Some(vec![Profile {
                network: Some("GitHub".to_string()),
                username: Some("janedoe".to_string()),
                url: Some(Url::parse("https://github.com/janedoe").unwrap()),
                additional_properties: HashMap::new(),
            }]),
            additional_properties: HashMap::new(),
        }),
        work: Some(vec![WorkExperience {
            name: Some("Tech Solutions Inc.".to_string()),
            location: Some("San Francisco, CA".to_string()),
            description: Some("Cloud Infrastructure Provider".to_string()),
            position: Some("Senior Backend Engineer".to_string()),
            url: Some(Url::parse("https://techsolutions.com").unwrap()),
            start_date: Some("2021-03-01".to_string()),
            end_date: None, // Current position
            summary: Some("Leading the migration of legacy services to Rust.".to_string()),
            highlights: Some(vec![
                "Reduced p99 latency by 40%".to_string(),
                "Architected a zero-trust auth system".to_string(),
            ]),
            additional_properties: HashMap::new(),
        }]),
        volunteer: None,
        education: Some(vec![Education {
            institution: Some("University of Technology".to_string()),
            url: Some(Url::parse("https://unitech.edu").unwrap()),
            area: Some("Computer Science".to_string()),
            study_type: Some("Bachelor".to_string()),
            start_date: Some("2014-09".to_string()),
            end_date: Some("2018-05".to_string()),
            score: Some("3.9".to_string()),
            courses: Some(vec![
                "Distributed Systems".to_string(),
                "Compilers".to_string(),
            ]),
            additional_properties: HashMap::new(),
        }]),
        awards: None,
        certificates: Some(vec![Certificate {
            name: Some("AWS Certified Solutions Architect".to_string()),
            date: Some("2022-06-15".to_string()),
            url: Some(Url::parse("https://aws.amazon.com/verification").unwrap()),
            issuer: Some("Amazon Web Services".to_string()),
            additional_properties: HashMap::new(),
        }]),
        publications: None,
        skills: Some(vec![Skill {
            name: Some("Programming Languages".to_string()),
            level: Some("Expert".to_string()),
            keywords: Some(vec!["Rust".into(), "C++".into(), "Go".into()]),
            additional_properties: HashMap::new(),
        }]),
        languages: Some(vec![Language {
            language: Some("English".to_string()),
            fluency: Some("Native".to_string()),
            additional_properties: HashMap::new(),
        }]),
        interests: Some(vec![Interest {
            name: Some("Open Source".to_string()),
            keywords: Some(vec!["Linux Kernel".into(), "WebAssembly".into()]),
            additional_properties: HashMap::new(),
        }]),
        references: None,
        projects: Some(vec![Project {
            name: Some("Custom Hypervisor".to_string()),
            description: Some("A lightweight hypervisor written in Rust.".to_string()),
            highlights: Some(vec!["Supports KVM backend".to_string()]),
            keywords: Some(vec!["Virtualization".into(), "Low-level".into()]),
            start_date: Some("2022-01".to_string()),
            end_date: None,
            url: None,
            roles: Some(vec!["Lead Developer".to_string()]),
            entity: None,
            project_type: Some("Personal".to_string()),
            additional_properties: HashMap::new(),
        }]),
        meta: Some(Meta {
            canonical: None,
            version: Some("v1.0.0".to_string()),
            last_modified: Some("2026-01-15T20:00:00Z".to_string()),
            additional_properties: HashMap::new(),
        }),
        additional_properties: HashMap::new(),
    }
}
