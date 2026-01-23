// Import the custom template (assuming it's in a local file or package)
// Replace with actual path/package name as needed
#import "./basic.typ": resume, render-work, render-education, render-projects, render-skills, render-certificates, render-awards, render-publications, render-volunteer, render-languages, render-interests, render-references

// Get data from sys.inputs (native Dict from Rust)
#let data = sys.inputs

// Configure text and headings
#set text(font: "New Computer Modern", 10pt)
#show heading: set text(font: "Source Serif 4", weight: "bold")
#show heading.where(level: 2): set block(above: 2em)
#show math.equation: set text(font: "STIX Two Math")
#show raw: set text(font: "JetBrains Mono")


// --- Helper: Format Date ---
#let format-date(date-str) = {
  if type(date-str) != str or date-str == "" { return none }
  let parts = date-str.split("-")
  if parts.len() < 2 { return date-str }
  
  let months = (
    "01": "Jan", "02": "Feb", "03": "Mar", "04": "Apr", "05": "May", "06": "Jun",
    "07": "Jul", "08": "Aug", "09": "Sep", "10": "Oct", "11": "Nov", "12": "Dec"
  )
  months.at(parts.at(1), default: parts.at(1)) + " " + parts.at(0)
}

// --- Convert sys.inputs data to typed Resume object ---
#let basics-data = data.at("basics", default: (:))
#let location-data = basics-data.at("location", default: (:))
#let profiles-data = basics-data.at("profiles", default: ())

// Build the json-resume object structure
#let json-resume = (
  basics: (
    name: basics-data.at("name", default: none),
    label: basics-data.at("label", default: none),
    image: basics-data.at("image", default: none),
    email: basics-data.at("email", default: none),
    phone: basics-data.at("phone", default: none),
    url: basics-data.at("url", default: none),
    summary: basics-data.at("summary", default: none),
    location: if location-data != (:) {
      (
        address: location-data.at("address", default: none),
        postalCode: location-data.at("postalCode", default: none),
        city: location-data.at("city", default: none),
        countryCode: location-data.at("countryCode", default: none),
        region: location-data.at("region", default: none),
      )
    } else { none },
    profiles: if profiles-data.len() > 0 {
      profiles-data.map(p => (
        network: p.at("network", default: none),
        username: p.at("username", default: none),
        url: p.at("url", default: none),
      ))
    } else { none },
  ),
  work: if data.at("work", default: none) != none {
    data.work.map(w => (
      name: w.at("name", default: none),
      position: w.at("position", default: none),
      url: w.at("url", default: none),
      startDate: format-date(w.at("startDate", default: "")),
      endDate: format-date(w.at("endDate", default: "")),
      summary: w.at("summary", default: none),
      highlights: w.at("highlights", default: none),
    ))
  } else { none },
  education: if data.at("education", default: none) != none {
    data.education.map(e => (
      institution: e.at("institution", default: none),
      url: e.at("url", default: none),
      area: e.at("area", default: none),
      studyType: e.at("studyType", default: none),
      startDate: format-date(e.at("startDate", default: "")),
      endDate: format-date(e.at("endDate", default: "")),
      score: e.at("score", default: none),
      courses: e.at("courses", default: none),
    ))
  } else { none },
  projects: if data.at("projects", default: none) != none {
    data.projects.map(p => (
      name: p.at("name", default: none),
      description: p.at("description", default: none),
      highlights: p.at("highlights", default: none),
      keywords: p.at("keywords", default: none),
      startDate: format-date(p.at("startDate", default: "")),
      endDate: format-date(p.at("endDate", default: "")),
      url: p.at("url", default: none),
      roles: p.at("roles", default: none),
      entity: p.at("entity", default: none),
      type: p.at("type", default: none),
    ))
  } else { none },
  skills: if data.at("skills", default: none) != none {
    data.skills.map(s => (
      name: s.at("name", default: none),
      level: s.at("level", default: none),
      keywords: s.at("keywords", default: none),
    ))
  } else { none },
  certificates: data.at("certificates", default: none),
  awards: data.at("awards", default: none),
  publications: data.at("publications", default: none),
  volunteer: data.at("volunteer", default: none),
  languages: data.at("languages", default: none),
  interests: data.at("interests", default: none),
  references: data.at("references", default: none),
)

// --- Render the resume using the custom template ---
#show: resume.with(
  json-resume,
  accent-color: rgb("#5E81AC"),
  font: "New Computer Modern",
  paper: "us-letter",
  author-font-size: 20pt,
  font-size: 10pt,
  lang: "en",
)

// Render all sections
#render-work(json-resume.work)
#render-education(json-resume.education)
#render-projects(json-resume.projects)
#render-skills(json-resume.skills)
#render-certificates(json-resume.certificates)
#render-awards(json-resume.awards)
#render-publications(json-resume.publications)
#render-volunteer(json-resume.volunteer)
#render-languages(json-resume.languages)
#render-interests(json-resume.interests)
#render-references(json-resume.references)
