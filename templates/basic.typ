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
// Main Resume Template
// ============================================================================

#let resume(
  json-resume,
  accent-color: "#000000",
  font: "New Computer Modern",
  paper: "us-letter",
  author-font-size: 20pt,
  font-size: 10pt,
  lang: "en",
  author-position: left,
  personal-info-position: left,
  body,
) = {
  
  let basics = json-resume.basics
  let display-name = if basics != none and basics.name != none {
    basics.name
  } else {
    "Resume"
  }
  
  // Sets document metadata
  set document(author: display-name, title: display-name)
  
  // Document-wide formatting
  set text(
    font: font,
    size: font-size,
    lang: lang,
    ligatures: false
  )
  
  set page(
    margin: (0.5in),
    paper: paper,
  )
  
  show link: underline
  
  // Section titles
  show heading.where(level: 2): it => [
    #pad(top: 0pt, bottom: -10pt, [#smallcaps(it.body)])
    #line(length: 100%, stroke: 1pt)
  ]
  
  // Accent color styling
  show heading: set text(fill: rgb(accent-color))
  show link: set text(fill: rgb(accent-color))
  
  // Name styling
  show heading.where(level: 1): it => [
    #set align(author-position)
    #set text(weight: 700, size: author-font-size)
    #pad(it.body)
  ]
  
  // Display name
  [= #display-name]
  
  // Build contact info from JSON Resume
  pad(
    top: 0.25em,
    align(personal-info-position)[
      #{
        let items = ()
        
        if basics != none {
          if basics.label != none { items.push(basics.label) }
          if basics.phone != none { items.push(basics.phone) }
          if basics.location != none {
            let loc = format-location(basics.location)
            if loc != "" { items.push(loc) }
          }
          if basics.email != none {
            items.push(link("mailto:" + basics.email)[#basics.email])
          }
          if basics.url != none {
            items.push(link(basics.url)[#basics.url])
          }
          
          // Add profiles
          if basics.profiles != none {
            for profile in basics.profiles {
              if profile.url != none {
                let display = if profile.network != none {
                  profile.network + if profile.username != none { ": " + profile.username } else { "" }
                } else if profile.username != none {
                  profile.username
                } else {
                  profile.url
                }
                items.push(link(profile.url)[#display])
              }
            }
          }
        }
        
        items.filter(x => x != none).join("  |  ")
      }
    ],
  )
  
  // Display summary if present
  if basics != none and basics.summary != none {
    pad(top: 0.5em, bottom: 0.5em)[
      #basics.summary
    ]
  }
  
  set par(justify: true)
  
  body
}

// ============================================================================
// Section Rendering Functions
// ============================================================================

#let render-work(work-items) = {
  if work-items == none or work-items.len() == 0 { return }
  
  [== Experience]
  
  for item in work-items {
    generic-two-by-two(
      top-left: strong(if item.position != none { item.position } else { "" }),
      top-right: format-date-range(item.startDate, item.endDate),
      bottom-left: if item.name != none { item.name } else { "" },
      bottom-right: emph(if item.url != none { item.url } else { "" }),
    )
    
    if item.summary != none {
      par[#item.summary]
    }
    
    if item.highlights != none and item.highlights.len() > 0 {
      list(..item.highlights)
    }
    
    v(0.5em)
  }
}

#let render-education(edu-items) = {
  if edu-items == none or edu-items.len() == 0 { return }
  
  [== Education]
  
  for item in edu-items {
    let degree = if item.studyType != none and item.area != none {
      item.studyType + " in " + item.area
    } else if item.studyType != none {
      item.studyType
    } else if item.area != none {
      item.area
    } else { "" }
    
    generic-two-by-two(
      top-left: strong(if item.institution != none { item.institution } else { "" }),
      top-right: format-date-range(item.startDate, item.endDate),
      bottom-left: emph(degree),
      bottom-right: if item.score != none { emph("GPA: " + item.score) } else { "" },
    )
    
    if item.courses != none and item.courses.len() > 0 {
      par[Courses: #item.courses.join(", ")]
    }
    
    v(0.5em)
  }
}

#let render-projects(project-items) = {
  if project-items == none or project-items.len() == 0 { return }
  
  [== Projects]
  
  for item in project-items {
    let roles-str = if item.roles != none {
      item.roles.join(", ")
    } else { "" }
    
    generic-one-by-two(
      left: {
        [*#{if item.name != none { item.name } else { "" }}*]
        if roles-str != "" [ (#roles-str)]
        if item.url != none [ (#link(item.url)[#item.url])]
      },
      right: format-date-range(item.startDate, item.endDate),
    )
    
    if item.description != none {
      par[#item.description]
    }
    
    if item.highlights != none and item.highlights.len() > 0 {
      list(..item.highlights)
    }
    
    v(0.5em)
  }
}

#let render-skills(skill-items) = {
  if skill-items == none or skill-items.len() == 0 { return }
  
  [== Skills]
  
  for item in skill-items {
    let keywords-str = if item.keywords != none {
      item.keywords.join(", ")
    } else { "" }
    
    [*#{if item.name != none { item.name } else { "" }}*: #keywords-str]
    
    v(0.3em)
  }
}

#let render-certificates(cert-items) = {
  if cert-items == none or cert-items.len() == 0 { return }
  
  [== Certificates]
  
  for item in cert-items {
    [
      *#{if item.name != none { item.name } else { "" }}*, #{if item.issuer != none { item.issuer } else { "" }}
      #if item.url != none {
        [ (#link(item.url)[#item.url])]
      }
      #h(1fr) #{if item.date != none { item.date } else { "" }}
    ]
    
    v(0.3em)
  }
}

#let render-awards(award-items) = {
  if award-items == none or award-items.len() == 0 { return }
  
  [== Awards]
  
  for item in award-items {
    generic-one-by-two(
      left: [*#{if item.title != none { item.title } else { "" }}*],
      right: if item.date != none { item.date } else { "" },
    )
    
    if item.awarder != none {
      par[Awarded by: #item.awarder]
    }
    
    if item.summary != none {
      par[#item.summary]
    }
    
    v(0.5em)
  }
}

#let render-publications(pub-items) = {
  if pub-items == none or pub-items.len() == 0 { return }
  
  [== Publications]
  
  for item in pub-items {
    [
      *#{if item.name != none { item.name } else { "" }}*
      #if item.publisher != none [ — #item.publisher]
      #if item.url != none [ (#link(item.url)[link])]
      #h(1fr) #{if item.releaseDate != none { item.releaseDate } else { "" }}
    ]
    
    if item.summary != none {
      par[#item.summary]
    }
    
    v(0.5em)
  }
}

#let render-volunteer(volunteer-items) = {
  if volunteer-items == none or volunteer-items.len() == 0 { return }
  
  [== Volunteer Work]
  
  for item in volunteer-items {
    generic-two-by-two(
      top-left: strong(if item.position != none { item.position } else { "" }),
      top-right: format-date-range(item.startDate, item.endDate),
      bottom-left: if item.organization != none { item.organization } else { "" },
      bottom-right: emph(if item.url != none { item.url } else { "" }),
    )
    
    if item.summary != none {
      par[#item.summary]
    }
    
    if item.highlights != none and item.highlights.len() > 0 {
      list(..item.highlights)
    }
    
    v(0.5em)
  }
}

#let render-languages(lang-items) = {
  if lang-items == none or lang-items.len() == 0 { return }
  
  [== Languages]
  
  for item in lang-items {
    [*#{if item.language != none { item.language } else { "" }}*: #{if item.fluency != none { item.fluency } else { "" }}]
    
    v(0.3em)
  }
}

#let render-interests(interest-items) = {
  if interest-items == none or interest-items.len() == 0 { return }
  
  [== Interests]
  
  for item in interest-items {
    let keywords-str = if item.keywords != none {
      item.keywords.join(", ")
    } else { "" }
    
    [*#{if item.name != none { item.name } else { "" }}*: #keywords-str]
    
    v(0.3em)
  }
}

#let render-references(ref-items) = {
  if ref-items == none or ref-items.len() == 0 { return }
  
  [== References]
  
  for item in ref-items {
    [*#{if item.name != none { item.name } else { "" }}*]
    
    if item.reference != none {
      par[#item.reference]
    }
    
    v(0.5em)
  }
}

// ============================================================================
// Complete JSON Resume Renderer
// ============================================================================

#let render-json-resume(
  json-resume,
  accent-color: "#000000",
  font: "New Computer Modern",
  paper: "us-letter",
  author-font-size: 20pt,
  font-size: 10pt,
  lang: "en",
) = {
  resume(
    json-resume,
    accent-color: accent-color,
    font: font,
    paper: paper,
    author-font-size: author-font-size,
    font-size: font-size,
    lang: lang,
  )[
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
  ]
}
