#set text(font: "Lora", fill: white, size: 0.9em)
#set page(margin: 0.5in, fill: black)
#set list(indent: 1em, marker: "*")
#let resume = yaml("resume.yaml")
#place(
  center + horizon,
  image("./spiral_dots_overlay.png", width: 190%, height: 190%),
)
#let datetime-from-str(s) = {
  let RE = regex("^([1-2][0-9]{3})-([0-1][0-9])-([0-3][0-9])$")
  let caps = s.match(RE).captures.map(int)
  datetime(year: caps.at(0), month: caps.at(1), day: caps.at(2))
}
#let section-item(
  name,
  url: none,
  note: none,
  description: none,
  start: none,
  end: none,
  body: none,
) = {
  name = if url == none {
    name
  } else {
    [#underline(link(url, name))]
  }
  note = if note != none {
    [ | #note ]
  }
  description = if description != none {
    [ _ #description _ ]
  }
  let date = if start == none and end == none {
    none
  } else if start == none {
    panic("cannot specify only end")
  } else {
    let end = if end == none {
      "Present"
    } else {
      datetime-from-str(end).display("[month repr:short]. [year]")
    }
    [#datetime-from-str(start).display("[month repr:short]. [year]") - #end]
  }
  body = if type(body) == array {
    list(..body)
  } else {
    body
  }
  block(below: 0.75em)[#box[=== #name] #note #h(1fr) #date]
  description
  body
}
#let section(name, items, f) = {
  let items = items.map(f)
  block(below: 0.5em)[== #name]
  line(length: 100%)
  block(above: 0.75em, for item in items {
    section-item(item.remove("name"), ..item)
  })
}
#let name_block = box(
  fill: white,
  pad(x: 12pt, y: 6pt, text(
    size: 1.25em,
    font: "Gap Sans",
    fill: black,
    "miles wirht",
  )),
)
#place(
  top + left,
  dx: -52pt,
  dy: 3pt,
  rotate(90deg, origin: center + top)[#name_block],
)
#show heading.where(level: 1): set text(size: 32pt, font: "Bagnard")
#align(center)[
  = THE STUDENT
  #link("mailto:" + resume.basics.email, resume.basics.email) |
  #(
    resume
      .basics
      .profiles
      .map(p => link(
        p.url,
      )[*\[#lower(p.network)\]* #p.at("username", default: p.url)])
      .join(" | ")
  )
]

// Education: score is a float in YAML, convert to string; no courses field so use activities
#section("Education", resume.education, e => {
  (
    name: e.institution,
    note: "GPA: " + str(e.score),
    description: e.studyType + " in " + e.area,
    start: e.startDate,
    end: e.at("endDate", default: none),
    body: e.at("activities", default: none),
  )
})

// Work: location folded into note alongside position
#section("Experience", resume.work, w => {
  (
    name: w.name,
    note: w.position + " | " + w.at("location", default: ""),
    description: w.at("description", default: none),
    start: w.startDate,
    end: w.at("endDate", default: none),
    body: w.highlights,
  )
})

// Projects: no startDate on all entries, keywords optional
#section("Projects", resume.projects, p => {
  (
    name: p.name,
    url: p.at("url", default: none),
    note: p.at("keywords", default: ()).join(", "),
    description: p.at("description", default: none),
    start: p.at("startDate", default: none),
    end: p.at("endDate", default: none),
    body: p.highlights,
  )
})

// Skills: keywords is an array of strings — join into a prose line
#section("Technical Skills", resume.skills, s => {
  (name: s.name, body: s.keywords.join(", "))
})

// Volunteer
#section("Volunteering", resume.volunteer, v => {
  (
    name: v.organization,
    note: v.position,
    description: v.at("location", default: none),
    start: v.startDate,
    end: v.at("endDate", default: none),
    body: v.highlights,
  )
})

// Awards
#section("Honors & Awards", resume.awards, a => {
  (
    name: a.title,
    note: a.awarder,
    start: a.date,
    body: none,
  )
})

