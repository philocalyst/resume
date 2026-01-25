-- Import the resume schema (assuming it's saved as resume-schema.dhall)
let Resume = ./resume-schema.dhall

in  Resume.Resume::{
    , basics = Some Resume.Basics::{
      , name = Some "Dr. Sarah Chen"
      , pronouns = Some { 
        display = "she/her",
        primary = None Resume.PronounSet,
        additional = [] : List Resume.PronounSet
      }
      , label = Some "Senior Software Engineer & Tech Lead"
      , email = Some "sarah.chen@email.com"
      , phone = Some "+1 (555) 123-4567"
      , url = Some "https://sarahchen.dev"
      , summary = Some
          "Full-stack engineer with 8+ years of experience building scalable distributed systems. Passionate about functional programming, type systems, and developer tooling. PhD in Computer Science with focus on programming language theory."
      , image = Some "https://sarahchen.dev/profile.jpg"
      , location = Some Resume.Address::{
        , street = Some "742 Evergreen Terrace"
        , city = Some "San Francisco"
        , region = Some "CA"
        , postalCode = Some "94102"
        , countryCode = Some "US"
        }
      , profiles = Some
        [ Resume.Profile::{
          , network = Some Resume.NetworkType.GitHub
          , username = Some "schen"
          , url = Some "https://github.com/schen"
          }
        , Resume.Profile::{
          , network = Some Resume.NetworkType.LinkedIn
          , username = Some "sarahchen"
          , url = Some "https://linkedin.com/in/sarahchen"
          }
        , Resume.Profile::{
          , network = Some Resume.NetworkType.Twitter
          , username = Some "sarahcodes"
          , url = Some "https://twitter.com/sarahcodes"
          }
        , Resume.Profile::{
          , network = Some Resume.NetworkType.Mastodon
          , username = Some "@sarah@fosstodon.org"
          , url = Some "https://fosstodon.org/@sarah"
          }
        , Resume.Profile::{
          , network = Some Resume.NetworkType.Medium
          , username = Some "sarahchen"
          , url = Some "https://medium.com/@sarahchen"
          }
        , Resume.Profile::{
          , network = Some Resume.NetworkType.StackOverflow
          , username = Some "schen"
          , url = Some "https://stackoverflow.com/users/12345/schen"
          }
        ]
      }
    , work = Some
      [ Resume.Work::{
        , name = Some "TechCorp Inc"
        , location = Some
            ( Resume.LocationType.Hybrid
                { onSite = "San Francisco, CA"
                , description = Some "3 days in office, 2 days remote"
                }
            )
        , description = Some "Cloud infrastructure and developer tools company"
        , position = Some "Senior Software Engineer & Tech Lead"
        , url = Some "https://techcorp.example"
        , startDate = Some "2020-03-01"
        , employmentType = Some Resume.EmploymentType.FullTime
        , summary = Some
            "Lead a team of 6 engineers building distributed systems for real-time data processing. Architected migration from monolith to microservices serving 10M+ requests per day."
        , highlights = Some
          [ "Reduced system latency by 65% through architectural redesign and optimization"
          , "Led adoption of Rust for performance-critical services, improving throughput by 3x"
          , "Mentored 4 junior engineers, 2 of whom were promoted to mid-level"
          , "Established on-call rotation and incident response procedures"
          , "Presented at internal tech talks on type-driven development"
          ]
        }
      , Resume.Work::{
        , name = Some "StartupXYZ"
        , location = Some Resume.LocationType.Remote
        , description = Some "Early-stage fintech startup"
        , position = Some "Software Engineer"
        , url = Some "https://startupxyz.example"
        , startDate = Some "2018-06-01"
        , endDate = Some "2020-02-01"
        , employmentType = Some Resume.EmploymentType.FullTime
        , summary = Some
            "Full-stack engineer in a fast-paced startup environment. Built features across the stack from database to frontend."
        , highlights = Some
          [ "Implemented real-time payment processing system handling $2M+ monthly volume"
          , "Built RESTful APIs serving mobile and web clients"
          , "Contributed to all phases of product development from design to deployment"
          ]
        }
      , Resume.Work::{
        , name = Some "Self-Employed"
        , location = Some Resume.LocationType.Remote
        , position = Some "Freelance Developer"
        , startDate = Some "2017-01-01"
        , endDate = Some "2018-05-01"
        , employmentType = Some Resume.EmploymentType.Freelance
        , summary = Some
            "Provided web development and consulting services to small businesses and startups."
        , highlights = Some
          [ "Delivered 12+ client projects on time and within budget"
          , "Specialized in React, Node.js, and PostgreSQL stacks"
          , "Maintained 100% client satisfaction rate"
          ]
        }
      ]
    , education = Some
      [ Resume.Education::{
        , institution = Some "Stanford University"
        , url = Some "https://stanford.edu"
        , area = Some "Computer Science"
        , studyType = Some Resume.DegreeType.PhD
        , startDate = Some "2013-09-01"
        , endDate = Some "2017-06-01"
        , score = Some (Resume.Score.GPA_Weighted {
          score = 3.95,
          scale = 4.0,
        })
        , courses = Some
          [ "Advanced Type Systems"
          , "Compiler Construction"
          , "Distributed Systems"
          , "Program Analysis and Verification"
          ]
        }
      , Resume.Education::{
        , institution = Some "UC Berkeley"
        , url = Some "https://berkeley.edu"
        , area = Some "Computer Science"
        , studyType = Some Resume.DegreeType.BachelorDegree
        , startDate = Some "2009-08-01"
        , endDate = Some "2013-05-01"
        , score = Some
            ( Resume.Score.GPA_Weighted
                { score = 4.15, scale = 5.0 }
            )
        , courses = Some
          [ "Data Structures and Algorithms"
          , "Operating Systems"
          , "Computer Architecture"
          , "Artificial Intelligence"
          ]
        }
      , Resume.Education::{
        , institution = Some "Fullstack Academy"
        , url = Some "https://fullstackacademy.com"
        , area = Some "Web Development"
        , studyType = Some Resume.DegreeType.Bootcamp
        , startDate = Some "2016-06-01"
        , endDate = Some "2016-09-01"
        , score = Some (Resume.Score.PassFail True)
        }
      ]
    , skills = Some
      [ Resume.Skill::{
        , name = Some "Programming Languages"
        , level = Some Resume.SkillLevel.Expert
        , keywords = Some
          [ "Rust"
          , "TypeScript"
          , "JavaScript"
          , "Python"
          , "Go"
          , "Haskell"
          , "OCaml"
          ]
        }
      , Resume.Skill::{
        , name = Some "Web Technologies"
        , level = Some Resume.SkillLevel.Expert
        , keywords = Some
          [ "React"
          , "Node.js"
          , "GraphQL"
          , "REST APIs"
          , "WebAssembly"
          , "HTML"
          , "CSS"
          ]
        }
      , Resume.Skill::{
        , name = Some "Infrastructure & DevOps"
        , level = Some Resume.SkillLevel.Advanced
        , keywords = Some
          [ "Kubernetes"
          , "Docker"
          , "AWS"
          , "PostgreSQL"
          , "Redis"
          , "Terraform"
          , "CI/CD"
          ]
        }
      , Resume.Skill::{
        , name = Some "Tools & Practices"
        , level = Some Resume.SkillLevel.Expert
        , keywords = Some
          [ "Git"
          , "Test-Driven Development"
          , "Code Review"
          , "Agile"
          , "System Design"
          ]
        }
      , Resume.Skill::{
        , name = Some "Configuration Languages"
        , level = Some Resume.SkillLevel.Advanced
        , keywords = Some [ "Dhall", "YAML", "JSON", "TOML" ]
        }
      ]
    , languages = Some
      [ Resume.Language::{
        , language = Some Resume.LanguageType.English
        , fluency = Some Resume.FluencyLevel.NativeOrBilingual
        }
      , Resume.Language::{
        , language = Some Resume.LanguageType.Mandarin
        , fluency = Some Resume.FluencyLevel.NativeOrBilingual
        }
      , Resume.Language::{
        , language = Some Resume.LanguageType.Spanish
        , fluency = Some Resume.FluencyLevel.ProfessionalWorking
        }
      , Resume.Language::{
        , language = Some Resume.LanguageType.French
        , fluency = Some Resume.FluencyLevel.LimitedWorking
        }
      ]
    , projects = Some
      [ Resume.Project::{
        , name = Some "RustQL"
        , description = Some
            "Type-safe GraphQL client library for Rust with compile-time query validation"
        , type = Some Resume.ProjectType.OpenSource
        , url = Some "https://github.com/schen/rustql"
        , startDate = Some "2021-03-01"
        , roles = Some [ "Creator", "Maintainer" ]
        , highlights = Some
          [ "1,200+ GitHub stars"
          , "Used by 50+ production applications"
          , "Featured in Rust weekly newsletter"
          ]
        , keywords = Some [ "Rust", "GraphQL", "Type Systems", "Code Generation" ]
        }
      , Resume.Project::{
        , name = Some "Dhall Resume Generator"
        , description = Some
            "Tool for generating beautiful resumes from Dhall configuration files"
        , type = Some Resume.ProjectType.Application
        , url = Some "https://github.com/schen/dhall-resume"
        , startDate = Some "2023-11-01"
        , roles = Some [ "Creator" ]
        , highlights = Some
          [ "Supports multiple output formats (PDF, HTML, Markdown)"
          , "Type-safe configuration with Dhall"
          , "Customizable themes and layouts"
          ]
        , keywords = Some [ "Dhall", "TypeScript", "PDF Generation" ]
        }
      , Resume.Project::{
        , name = Some "Tech Talks: Functional Programming in Production"
        , description = Some
            "Series of conference talks on adopting functional programming in industry"
        , type = Some Resume.ProjectType.Talk
        , startDate = Some "2022-01-01"
        , roles = Some [ "Speaker" ]
        , highlights = Some
          [ "Presented at 5 conferences including StrangeLoop and LambdaConf"
          , "Reached 2000+ attendees across all talks"
          , "Published accompanying blog series with 50k+ views"
          ]
        , keywords = Some
          [ "Public Speaking", "Functional Programming", "Type Systems" ]
        }
      , Resume.Project::{
        , name = Some "Community Photography Exhibition"
        , description = Some
            "Curated photography exhibition showcasing Bay Area tech culture"
        , type = Some Resume.ProjectType.Exhibition
        , startDate = Some "2023-03-01"
        , endDate = Some "2023-03-31"
        , roles = Some [ "Photographer", "Curator" ]
        , highlights = Some
          [ "Featured 40 original photographs"
          , "Exhibited at local gallery with 500+ visitors"
          , "Raised $5000 for STEM education nonprofits"
          ]
        , keywords = Some [ "Photography", "Art", "Community" ]
        }
      , Resume.Project::{
        , name = Some "Tech Diversity Mentorship Program"
        , description = Some
            "Founded mentorship program pairing senior engineers with underrepresented minorities"
        , type = Some Resume.ProjectType.Mentorship
        , startDate = Some "2021-06-01"
        , roles = Some [ "Founder", "Mentor" ]
        , entity = Some "Tech Community Foundation"
        , highlights = Some
          [ "Mentored 8 junior engineers over 2 years"
          , "3 mentees received promotions during program"
          , "Program expanded to 25+ mentor-mentee pairs"
          ]
        , keywords = Some [ "Mentorship", "Diversity", "Community" ]
        }
      ]
    , publications = Some
      [ Resume.Publication::{
        , name = Some
            "Type-Preserving Compilation for Large-Scale Optimizing Compilers"
        , publisher = Some "ACM SIGPLAN"
        , releaseDate = Some "2016-06-15"
        , url = Some "https://dl.acm.org/doi/example"
        , summary = Some
            "Presented novel techniques for maintaining type information through aggressive compiler optimizations, enabling better error messages and verification."
        }
      , Resume.Publication::{
        , name = Some "The Pragmatic Type Theorist's Guide to Rust"
        , publisher = Some "Medium"
        , releaseDate = Some "2022-08-01"
        , url = Some "https://medium.com/@schen/pragmatic-type-theory"
        , summary = Some
            "Popular blog series exploring Rust's type system through the lens of programming language theory. 5-part series with 100k+ cumulative views."
        }
      ]
    , certificates = Some
      [ Resume.Certificate::{
        , name = Some "AWS Certified Solutions Architect - Professional"
        , issuer = Some "Amazon Web Services"
        , date = Some "2022-03-15"
        , expirationDate = Some "2025-03-15"
        , url = Some "https://aws.amazon.com/certification/"
        }
      , Resume.Certificate::{
        , name = Some "Certified Kubernetes Administrator (CKA)"
        , issuer = Some "Cloud Native Computing Foundation"
        , date = Some "2021-09-20"
        , expirationDate = Some "2024-09-20"
        , url = Some "https://www.cncf.io/certification/cka/"
        }
      ]
    , awards = Some
      [ Resume.Award::{
        , title = Some "Best Paper Award"
        , date = Some "2016-06-15"
        , awarder = Some "PLDI 2016"
        , summary = Some
            "Recognized for outstanding research contribution in programming languages"
        }
      , Resume.Award::{
        , title = Some "Graduate Fellowship"
        , date = Some "2013-09-01"
        , awarder = Some "National Science Foundation"
        , summary = Some "Competitive fellowship for PhD studies in Computer Science"
        }
      , Resume.Award::{
        , title = Some "Top 10 Tech Influencers in Functional Programming"
        , date = Some "2023-12-01"
        , awarder = Some "Functional Programming Magazine"
        , summary = Some
            "Recognized for contributions to functional programming community through talks, writing, and open source"
        }
      ]
    , volunteer = Some
      [ Resume.Volunteer::{
        , organization = Some "Code for America"
        , position = Some "Volunteer Developer"
        , url = Some "https://codeforamerica.org"
        , location = Some Resume.LocationType.Remote
        , startDate = Some "2019-01-01"
        , summary = Some
            "Contribute to civic tech projects improving government services"
        , highlights = Some
          [ "Built web application for voter registration assistance"
          , "Mentored new volunteers on project onboarding"
          ]
        }
      , Resume.Volunteer::{
        , organization = Some "SF SPCA"
        , position = Some "Dog Walker & Event Volunteer"
        , url = Some "https://sfspca.org"
        , location = Some (Resume.LocationType.OnSite "San Francisco, CA")
        , startDate = Some "2020-06-01"
        , summary = Some
            "Weekly volunteer helping care for shelter animals and supporting adoption events"
        , highlights = Some
          [ "Walked shelter dogs 3x per week for 3 years"
          , "Helped facilitate 50+ successful adoptions at events"
          ]
        }
      ]
    , interests = Some
      [ Resume.Interest::{
        , name = Some "Open Source Software"
        , keywords = Some
          [ "Rust", "Type Systems", "Developer Tools", "Programming Languages" ]
        }
      , Resume.Interest::{
        , name = Some "Photography"
        , keywords = Some
          [ "Street Photography", "Landscape", "Film Photography", "Darkroom" ]
        }
      , Resume.Interest::{
        , name = Some "Outdoor Activities"
        , keywords = Some
          [ "Hiking", "Rock Climbing", "Backpacking", "Trail Running" ]
        }
      , Resume.Interest::{
        , name = Some "Music"
        , keywords = Some [ "Piano", "Jazz", "Classical", "Vinyl Collecting" ]
        }
      ]
    , references = Some
      [ Resume.Reference::{
        , name = Some "Dr. James Wilson"
        , reference = Some
            "Sarah was one of the most talented PhD students I've advised. Her research on type systems was groundbreaking, and her ability to explain complex concepts is exceptional. She would be an asset to any team."
        }
      , Resume.Reference::{
        , name = Some "Alex Rodriguez, CTO at TechCorp"
        , reference = Some
            "Sarah is an exceptional technical leader who combines deep expertise with excellent mentorship skills. She transformed our engineering culture and delivered outstanding results. I can't recommend her highly enough."
        }
      ]
    , meta = Some Resume.Meta::{
      , canonical = Some "https://sarahchen.dev/resume.json"
      , version = Some Resume.SemVer::{
        , major = 2
        , minor = 1
        , patch = 0
        , prerelease = None Text
        , build = None Text
        }
      , lastModified = Some
          { date = "2024-01-15", time = "14:30:00" }
      }
    }
