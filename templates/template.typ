#import "@preview/basic-resume:0.2.9": *

// sys.inputs.v is the native Dict from Rust
#let data = sys.inputs
#set text(font: "Inter 18pt", 11pt)
#show heading: set text(font: "Source Serif 4", weight: "bold")
#show heading.where(level: 2): set block(above: 2em)
#show math.equation: set text(font: "STIX Two Math")
#show raw: set text(font: "JetBrains Mono")

#let basics = data.at("basics", default: (:))
#let profiles = basics.at("profiles", default: ())
#let loc = basics.at("location", default: (:))

// --- Helper: Format Date ---
#let format-date(date-str) = {
  if type(date-str) != str or date-str == "" { return "Present" }
  let parts = date-str.split("-")
  if parts.len() < 2 { return date-str }
  
  let months = (
    "01": "Jan", "02": "Feb", "03": "Mar", "04": "Apr", "05": "May", "06": "Jun",
    "07": "Jul", "08": "Aug", "09": "Sep", "10": "Oct", "11": "Nov", "12": "Dec"
  )
  months.at(parts.at(1), default: parts.at(1)) + " " + parts.at(0)
}

// --- Helper: Get Profile ---
#let get-profile(name) = {
  let p = profiles.find(it => lower(it.at("network", default: "")) == lower(name))
  if p != none { p.at("url", default: none) } else { none }
}

// --- Main Setup ---
// The panic "expected integer" often comes from accent-color if not a valid hex 
// or if personal-info-position isn't a valid alignment type.
#show: resume.with(
  author: basics.at("name", default: "Unnamed"),
  location: (loc.at("city", default: none), loc.at("countryCode", default: none)).filter(it => it != none).join(", "),
  email: basics.at("email", default: none),
  phone: basics.at("phone", default: none),
  github: get-profile("github"),
  linkedin: get-profile("linkedin"),
  personal-site: basics.at("url", default: none),
  accent-color: "#26428b",
  font: "TeX Gyre Cursor",
  paper: "us-letter",
)

#if basics.at("summary", default: none) != none [
  #basics.summary
  #v(0.5em)
]

// --- Education ---
#if data.at("education", default: none) != none {
  [== Education]
  for edu_item in data.education {
    let start = format-date(edu_item.at("startDate", default: ""))
    let end = format-date(edu_item.at("endDate", default: ""))
    
    edu(
      institution: edu_item.at("institution", default: ""),
      location: "",
      dates: dates-helper(start-date: start, end-date: end),
      degree: edu_item.at("area", default: ""),
    )
    if edu_item.at("score", default: none) != none [- GPA: #edu_item.score]
  }
}

// --- Work ---
#if data.at("work", default: none) != none {
  [== Experience]
  for w in data.work {
    work(
      title: w.at("position", default: ""),
      location: w.at("location", default: ""),
      company: w.at("name", default: ""),
      dates: dates-helper(
        start-date: format-date(w.at("startDate", default: "")), 
        end-date: format-date(w.at("endDate", default: ""))
      ),
    )
    if w.at("highlights", default: none) != none {
      for hi in w.highlights [ - #hi ]
    }
  }
}

// --- Projects ---
#if data.at("projects", default: none) != none {
  [== Projects]
  for p in data.projects {
    project(
      name: p.at("name", default: ""),
      role: p.at("roles", default: ()).join(", "),
      dates: dates-helper(
        start-date: format-date(p.at("startDate", default: "")), 
        end-date: format-date(p.at("endDate", default: ""))
      ),
      url: p.at("url", default: ""),
    )
    if p.at("highlights", default: none) != none {
      for hi in p.highlights [ - #hi ]
    }
  }
}

// --- Skills ---
#if data.at("skills", default: none) != none {
  [== Skills]
  for s in data.skills {
    let name = s.at("name", default: "")
    let keywords = s.at("keywords", default: ()).join(", ")
    [- *#name*: #keywords]
  }
}
