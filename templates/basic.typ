#import "@preview/typsy:0.2.2": class, Int, Str, Array, Union, None, Any

// ============================================================================
// JSON Resume Schema Classes (as provided)
// ============================================================================

#let Location = class(
    fields: (
        address: Union(Str, None),
        postalCode: Union(Str, None),
        city: Union(Str, None),
        countryCode: Union(Str, None),
        region: Union(Str, None)
    )
)

#let Profile = class(
    fields: (
        network: Union(Str, None),
        username: Union(Str, None),
        url: Union(Str, None)
    )
)

#let Basics = class(
    fields: (
        name: Union(Str, None),
        label: Union(Str, None),
        image: Union(Str, None),
        email: Union(Str, None),
        phone: Union(Str, None),
        url: Union(Str, None),
        summary: Union(Str, None),
        location: Union(Location, None),
        profiles: Union(Array(Profile), None)
    )
)

#let Work = class(
    fields: (
        name: Union(Str, None),
        position: Union(Str, None),
        url: Union(Str, None),
        startDate: Union(Str, None),
        endDate: Union(Str, None),
        summary: Union(Str, None),
        highlights: Union(Array(Str), None)
    )
)

#let Volunteer = class(
    fields: (
        organization: Union(Str, None),
        position: Union(Str, None),
        url: Union(Str, None),
        startDate: Union(Str, None),
        endDate: Union(Str, None),
        summary: Union(Str, None),
        highlights: Union(Array(Str), None)
    )
)

#let Education = class(
    fields: (
        institution: Union(Str, None),
        url: Union(Str, None),
        area: Union(Str, None),
        studyType: Union(Str, None),
        startDate: Union(Str, None),
        endDate: Union(Str, None),
        score: Union(Str, None),
        courses: Union(Array(Str), None)
    )
)

#let Award = class(
    fields: (
        title: Union(Str, None),
        date: Union(Str, None),
        awarder: Union(Str, None),
        summary: Union(Str, None)
    )
)

#let Certificate = class(
    fields: (
        name: Union(Str, None),
        date: Union(Str, None),
        issuer: Union(Str, None),
        url: Union(Str, None)
    )
)

#let Publication = class(
    fields: (
        name: Union(Str, None),
        publisher: Union(Str, None),
        releaseDate: Union(Str, None),
        url: Union(Str, None),
        summary: Union(Str, None)
    )
)

#let Skill = class(
    fields: (
        name: Union(Str, None),
        level: Union(Str, None),
        keywords: Union(Array(Str), None)
    )
)

#let Language = class(
    fields: (
        language: Union(Str, None),
        fluency: Union(Str, None)
    )
)

#let Interest = class(
    fields: (
        name: Union(Str, None),
        keywords: Union(Array(Str), None)
    )
)

#let Reference = class(
    fields: (
        name: Union(Str, None),
        reference: Union(Str, None)
    )
)

#let Project = class(
    fields: (
        name: Union(Str, None),
        description: Union(Str, None),
        highlights: Union(Array(Str), None),
        keywords: Union(Array(Str), None),
        startDate: Union(Str, None),
        endDate: Union(Str, None),
        url: Union(Str, None),
        roles: Union(Array(Str), None),
        entity: Union(Str, None),
        type: Union(Str, None)
    )
)

#let Resume = class(
    fields: (
        basics: Union(Basics, None),
        work: Union(Array(Work), None),
        volunteer: Union(Array(Volunteer), None),
        education: Union(Array(Education), None),
        awards: Union(Array(Award), None),
        certificates: Union(Array(Certificate), None),
        publications: Union(Array(Publication), None),
        skills: Union(Array(Skill), None),
        languages: Union(Array(Language), None),
        interests: Union(Array(Interest), None),
        references: Union(Array(Reference), None),
        projects: Union(Array(Project), None)
    )
)

// ============================================================================
// Helper Functions
// ============================================================================

#let format-location(location) = {
  if location == none { return "" }
  let parts = ()
  if location.city != none { parts.push(location.city) }
  if location.region != none { parts.push(location.region) }
  if location.countryCode != none { parts.push(location.countryCode) }
  parts.join(", ")
}

#let format-date-range(start, end) = {
  let start-str = if start != none { start } else { "" }
  let end-str = if end != none { end } else { "Present" }
  if start-str == "" and end-str == "Present" { "" }
  else if start-str == "" { end-str }
  else { start-str + " " + $dash.em$ + " " + end-str }
}

#let generic-two-by-two(top-left: "", top-right: "", bottom-left: "", bottom-right: "") = {
  [
    #top-left #h(1fr) #top-right \
    #bottom-left #h(1fr) #bottom-right
  ]
}

#let generic-one-by-two(left: "", right: "") = {
  [
    #left #h(1fr) #right
  ]
}
// ============================================================================
// Section: Projects
// ============================================================================

#let render-projects(project-items) = {
  if project-items == none or project-items.len() == 0 { return }

  [== Projects]

  for item in project-items {
    block(above: 1.2em, below: 0em)[
      #grid(
        columns: (auto, 1fr, auto),
        align: (left, center, right),
        gutter: 0.5em,

        text(weight: 300, size: 10.5pt)[*#item.name*],
        line(length: 100%, stroke: 0.5pt + rgb("#D8DEE9")),
        text(size: 9pt, fill: rgb("#4C566A"))[
          #{
            let start = if item.startDate != none { item.startDate } else { "" }
            let end = if item.endDate != none { item.endDate } else { "Present" }
            if start != "" { start + " — " + end }
          }
        ]
      )
    ]

    // Website / URL
    if item.url != none {
      align(right)[
        #text(size: 9pt)[#link(item.url)[#item.url]]
      ]
      v(-0.4em)
    }

    // Description
    if item.description != none {
      v(0.3em)
      par[#item.description]
    }

    // Highlights
    if item.highlights != none and item.highlights.len() > 0 {
      v(0.3em)
      par[*Highlights*]
      for highlight in item.highlights {
        pad(left: 1em)[
          #box(width: 0.8em)[#text(fill: rgb("#000000").lighten(90%), size: 9pt)[▲]]
          #h(0.2em)#highlight
        ]
      }
    }
  }
}

// ============================================================================
// Section: Volunteer
// ============================================================================

#let render-volunteer(volunteer-items) = {
  if volunteer-items == none or volunteer-items.len() == 0 { return }

  [== Volunteer]

  for item in volunteer-items {
    block(above: 1.2em, below: 0em)[
      #grid(
        columns: (auto, 1fr, auto),
        align: (left, center, right),
        gutter: 0.5em,

        text(weight: 300, size: 10.5pt)[*#item.organization*],
        line(length: 100%, stroke: 0.5pt + rgb("#D8DEE9")),
        text(size: 9pt, fill: rgb("#4C566A"))[
          #{
            let start = if item.startDate != none { item.startDate } else { "" }
            let end = if item.endDate != none { item.endDate } else { "Present" }
            if start != "" { start + " — " + end }
          }
        ]
      )
    ]

    // Website
    if item.url != none {
      align(right)[
        #text(size: 9pt)[#link(item.url)[#item.url]]
      ]
      v(-0.4em)
    }

    // Position
    if item.position != none {
      text(fill: rgb("#5E81AC"), weight: 400, size: 10pt)[#item.position]
    }

    // Summary
    if item.summary != none {
      v(0.3em)
      par[#item.summary]
    }

    // Highlights
    if item.highlights != none and item.highlights.len() > 0 {
      v(0.3em)
      par[*Highlights*]
      for highlight in item.highlights {
        pad(left: 1em)[
          #box(width: 0.8em)[#text(fill: rgb("#000000").lighten(90%), size: 9pt)[▲]]
          #h(0.2em)#highlight
        ]
      }
    }
  }
}

// ============================================================================
// Section: Awards
// ============================================================================

#let render-awards(award-items) = {
  if award-items == none or award-items.len() == 0 { return }

  [== Awards]

  for item in award-items {
    block(above: 1.2em, below: 0em)[
      #grid(
        columns: (auto, 1fr, auto),
        align: (left, center, right),
        gutter: 0.5em,

        text(weight: 300, size: 10.5pt)[*#item.title*],
        line(length: 100%, stroke: 0.5pt + rgb("#D8DEE9")),
        text(size: 9pt, fill: rgb("#4C566A"))[#if item.date != none { item.date }]
      )
    ]

    if item.awarder != none {
      text(fill: rgb("#5E81AC"), weight: 400, size: 10pt)[#item.awarder]
    }

    if item.summary != none {
      v(0.3em)
      par[#item.summary]
    }
  }
}

// ============================================================================
// Section: Publications
// ============================================================================

#let render-publications(pub-items) = {
  if pub-items == none or pub-items.len() == 0 { return }

  [== Publications]

  for item in pub-items {
    block(above: 1.2em, below: 0em)[
      #grid(
        columns: (auto, 1fr, auto),
        align: (left, center, right),
        gutter: 0.5em,

        text(weight: 300, size: 10.5pt)[*#item.name*],
        line(length: 100%, stroke: 0.5pt + rgb("#D8DEE9")),
        text(size: 9pt, fill: rgb("#4C566A"))[#if item.releaseDate != none { item.releaseDate }]
      )
    ]

    if item.publisher != none {
      text(fill: rgb("#5E81AC"), weight: 400, size: 10pt)[#item.publisher]
    }

    if item.url != none {
       h(0.5em)
       text(size: 9pt)[#link(item.url)[Link]]
    }

    if item.summary != none {
      v(0.3em)
      par[#item.summary]
    }
  }
}

// ============================================================================
// Section: Languages
// ============================================================================

#let render-languages(lang-items) = {
  if lang-items == none or lang-items.len() == 0 { return }

  [== Languages]

  let items = ()
  for item in lang-items {
    items.push([
      *#item.language* #if item.fluency != none { text(fill: rgb("#4C566A"), size: 0.9em)[— #item.fluency] }
    ])
  }

  grid(
    columns: (1fr, 1fr),
    row-gutter: 0.6em,
    column-gutter: 1em,
    ..items
  )
}

// ============================================================================
// Section: Interests
// ============================================================================

#let render-interests(interest-items) = {
  if interest-items == none or interest-items.len() == 0 { return }

  [== Interests]

  let items = ()
  for item in interest-items {
    items.push([
      === #item.name
      #if item.keywords != none and item.keywords.len() > 0 {
        for keyword in item.keywords {
          pad(left: 1em)[
            #box(width: 0.8em)[#text(fill: rgb("#000000").lighten(90%), size: 9pt)[▲]]
            #h(0.2em)#keyword
          ]
        }
      }
    ])
  }

  grid(
    columns: (1fr, 1fr),
    row-gutter: 0.8em,
    column-gutter: 1em,
    ..items
  )
}

// ============================================================================
// Section: References
// ============================================================================

#let render-references(ref-items) = {
  if ref-items == none or ref-items.len() == 0 { return }

  [== References]

  for item in ref-items {
    block(above: 1.2em, below: 0em)[
       text(weight: 300, size: 10.5pt)[*#item.name*]
    ]
    if item.reference != none {
      v(0.3em)
      par[#item.reference]
    }
  }
}

// ============================================================================
// Nordic-inspired Resume Template (matching HTML/CSS theme)
// ============================================================================

#let resume(
  json-resume,
  accent-color: rgb("#5E81AC"),
  secondary-color: rgb("#81A1C1"),
  text-color: rgb("#2E3440"),
  muted-color: rgb("#4C566A"),
  background: rgb("#ECEFF4"),
  font: "Lato",
  paper: "us-letter",
  author-font-size: 28pt,
  font-size: 10.5pt,
  lang: "en",
  body,
) = {
  
  let basics = json-resume.basics
  let display-name = if basics != none and basics.name != none {
    basics.name
  } else {
    "Resume"
  }
  
  set document(author: display-name, title: display-name)
  
  set text(
    font: font,
    size: font-size,
    lang: lang,
    fill: text-color,
    weight: 300,
  )
  
  set page(
    margin: (top: 0.75in, bottom: 0.75in, left: 0.75in, right: 0.75in),
    paper: paper,
    fill: background,
  )
  
  set par(
    justify: true,
    leading: 0.65em,
  )
  
  // Link styling - underlined with decorative color
  show link: it => {
    set text(fill: accent-color, weight: 500)
    underline(it, stroke: 1.5pt + rgb("#D8DEE9"), offset: 2pt)
  }
  
  // Strong text styling
  show strong: it => {
    set text(fill: muted-color, weight: 400, size: 0.95em)
    it
  }
  
  // Emphasis styling
  show emph: it => {
    set text(fill: muted-color, weight: 300, style: "normal")
    it
  }
  
  // Section headings (level 2)
  show heading.where(level: 2): it => {
    set text(fill: secondary-color, size: 18pt, weight: 300)
    block(above: 1.5em, below: 0.8em, it.body)
  }
  
  // Name (level 1)
  show heading.where(level: 1): it => {
    set text(weight: 300, size: author-font-size, fill: text-color)
    block(above: 0em, below: 0.3em, it.body)
  }
  
  // Header with name and title
  pad(top: 2em, bottom: 1.5em)[
    #grid(
      columns: (auto, 1fr),
      gutter: 1.5em,
      align: (center, left),
      
      // Profile image placeholder (circular)
      if basics != none and basics.image != none {
        box(
          width: 88pt,
          height: 88pt,
          radius: 50%,
          clip: true,
          fill: rgb("#D8DEE9"),
        )[
          #align(center + horizon)[
            #text(fill: muted-color, size: 8pt)[Profile]
          ]
        ]
      } else {
        box(
          width: 88pt,
          height: 88pt,
          radius: 50%,
          fill: rgb("#D8DEE9"),
        )[
          #align(center + horizon)[
            #text(fill: muted-color, size: 8pt)[Profile]
          ]
        ]
      },
      
      // Name and label
      align(left + horizon)[
        = #display-name
        #if basics != none and basics.label != none {
          text(size: 16pt, fill: muted-color)[#basics.label]
        }
      ]
    )
  ]
  
  body
}

// ============================================================================
// Section: Contact
// ============================================================================

#let render-contact(basics) = {
  if basics == none { return }
  
  [== Contact]
  
  grid(
    columns: (1fr, 1fr),
    row-gutter: 0.6em,
    column-gutter: 1em,
    
    if basics.email != none {
      [
        *Email*\
        #link("mailto:" + basics.email)[#basics.email]
      ]
    },
    
    if basics.phone != none {
      [
        *Phone*\
        #basics.phone
      ]
    },
    
    if basics.url != none {
      [
        *Website*\
        #link(basics.url)[#basics.url]
      ]
    },
  )
}

// ============================================================================
// Section: About
// ============================================================================

#let render-about(basics) = {
  if basics == none or basics.summary == none { return }
  
  [== About]
  
  par(justify: true)[#basics.summary]
}

// ============================================================================
// Section: Profiles
// ============================================================================

#let render-profiles(profiles) = {
  if profiles == none or profiles.len() == 0 { return }
  
  [== Profiles]
  
  let items = ()
  for profile in profiles {
    if profile.network != none or profile.url != none {
      items.push([
        *#upper(profile.network.at(0) + profile.network.slice(1))*\
        #link(profile.url)[#{if profile.username != none { profile.username } else { profile.url }}]
      ])
    }
  }
  
  grid(
    columns: (1fr, 1fr),
    row-gutter: 0.6em,
    column-gutter: 1em,
    ..items
  )
}

// ============================================================================
// Section: Work Experience
// ============================================================================

#let render-work(work-items) = {
  if work-items == none or work-items.len() == 0 { return }
  
  [== Work]
  
  for item in work-items {
    block(above: 1.2em, below: 0em)[
      // Company name with date range (strike-through effect)
      #grid(
        columns: (auto, 1fr, auto),
        align: (left, center, right),
        gutter: 0.5em,
        
        text(weight: 300, size: 10.5pt)[*#item.name*],
        line(length: 100%, stroke: 0.5pt + rgb("#D8DEE9")),
        text(size: 9pt, fill: rgb("#4C566A"))[
          #{
            let start = if item.startDate != none { item.startDate } else { "" }
            let end = if item.endDate != none { item.endDate } else { "Now" }
            if start != "" { start + " — " + end }
          }
        ]
      )
    ]
    
    // Website
    if item.url != none {
      align(right)[
        #text(size: 9pt)[#link(item.url)[#item.url]]
      ]
      v(-0.4em)
    }
    
    // Position
    text(fill: rgb("#5E81AC"), weight: 400, size: 10pt)[#item.position]
    
    // Summary
    if item.summary != none {
      v(0.3em)
      par[#item.summary]
    }
    
    // Highlights
    if item.highlights != none and item.highlights.len() > 0 {
      v(0.3em)
      par[*Highlights*]
      for highlight in item.highlights {
        pad(left: 1em)[
          #box(width: 0.8em)[#text(fill: rgb("#000000").lighten(90%), size: 9pt)[▲]]
          #h(0.2em)#highlight
        ]
      }
    }
  }
}

// ============================================================================
// Section: Skills
// ============================================================================

#let render-skills(skill-items) = {
  if skill-items == none or skill-items.len() == 0 { return }
  
  [== Skills]
  
  let items = ()
  for item in skill-items {
    items.push([
      === #item.name
      #if item.keywords != none and item.keywords.len() > 0 {
        for keyword in item.keywords {
          pad(left: 1em)[
            #box(width: 0.8em)[#text(fill: rgb("#000000").lighten(90%), size: 9pt)[▲]]
            #h(0.2em)#keyword
          ]
        }
      }
    ])
  }
  
  grid(
    columns: (1fr, 1fr),
    row-gutter: 0.8em,
    column-gutter: 1em,
    ..items
  )
}

// ============================================================================
// Section: Education
// ============================================================================

#let render-education(edu-items) = {
  if edu-items == none or edu-items.len() == 0 { return }
  
  [== Education]
  
  for item in edu-items {
    block(above: 1.2em, below: 0em)[
      #grid(
        columns: (auto, 1fr, auto),
        align: (left, center, right),
        gutter: 0.5em,
        
        text(weight: 300, size: 10.5pt)[*#item.institution*],
        line(length: 100%, stroke: 0.5pt + rgb("#D8DEE9")),
        text(size: 9pt, fill: rgb("#4C566A"))[
          #{
            let start = if item.startDate != none { item.startDate } else { "" }
            let end = if item.endDate != none { item.endDate } else { "Now" }
            if start != "" { start + " — " + end }
          }
        ]
      )
    ]
    
    // Area with icon
    text(weight: 400)[#text(font: "Octicons", size: 11pt)[▼] #h(0.3em)#item.area]
    
    // Study type and GPA
    pad(left: 1.8em)[
      #{item.studyType}#{if item.score != none { [ (#item.score)] }}
    ]
    
    // Courses
    if item.courses != none and item.courses.len() > 0 {
      v(0.5em)
      text(weight: 400, size: 9.5pt)[Courses]
      for course in item.courses {
        pad(left: 1em)[
          #box(width: 0.8em)[#text(fill: rgb("#000000").lighten(90%), size: 9pt)[▲]]
          #h(0.2em)#course
        ]
      }
    }
  }
}

// ============================================================================
// Section: Certifications
// ============================================================================

#let render-certificates(cert-items) = {
  if cert-items == none or cert-items.len() == 0 { return }
  
  [== Certifications]
  
  for item in cert-items {
    block(above: 1.2em, below: 0em)[
      #grid(
        columns: (auto, 1fr, auto),
        align: (left, center, right),
        gutter: 0.5em,
        
        text(weight: 300, size: 10.5pt)[*#item.name*],
        line(length: 100%, stroke: 0.5pt + rgb("#D8DEE9")),
        text(size: 9pt, fill: rgb("#4C566A"))[#item.date]
      )
    ]
    
    [_by_ *#item.issuer*]
    
    if item.url != none {
      v(0.3em)
      [_Credential ID_ #link(item.url)[#item.url]]
    }
  }
}

// ============================================================================
// Complete JSON Resume Renderer
// ============================================================================

#let render-json-resume(json-resume) = {
  resume(json-resume)[
    #render-contact(json-resume.basics)
    #render-about(json-resume.basics)
    #render-profiles(json-resume.basics.profiles)
    #render-work(json-resume.work)
    #render-project(json-resume.project)
    #render-skills(json-resume.skills)
    #render-education(json-resume.education)
    #render-certificates(json-resume.certificates)
#render-awards(json-resume.awards)                                                                      │
#render-publications(json-resume.publications)                                                          │
#render-volunteer(json-resume.volunteer)                                                                │
#render-languages(json-resume.languages)                                                                │
#render-interests(json-resume.interests)                                                                │
#render-references(json-resume.references)
  ]
}
