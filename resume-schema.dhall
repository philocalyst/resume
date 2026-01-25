-- Location Type (for work/volunteer positions)
let LocationType =
      < Remote
      | OnSite : Text
      | Hybrid : { onSite : Text, description : Optional Text }
      >

-- Academic Score/Grade
let Score =
      < 
      | GPA_Weighted : { score : Double, scale : Double }
      | Percentage : Natural
      | PassFail : Bool
      | LetterGrade : Text
      | Custom : { score : Text, scale : Text }
      >

-- Semantic Version
let SemVer =
      { Type =
          { major : Natural
          , minor : Natural
          , patch : Natural
          , prerelease : Optional Text
          , build : Optional Text
          }
      , default =
          { major = 0
          , minor = 0
          , patch = 0
          , prerelease = None Text
          , build = None Text
          }
      }

-- Simplified Address type
let Address =
      { Type =
          { street : Optional Text
          , city : Optional Text
          , region : Optional Text
          , postalCode : Optional Text
          , countryCode : Optional Text
          }
      , default =
          { street = None Text
          , city = None Text
          , region = None Text
          , postalCode = None Text
          , countryCode = None Text
          }
      }

-- Degree/Education Level
let DegreeType =
      < HighSchoolDiploma
      | GED
      | AssociateDegree
      | BachelorDegree
      | MasterDegree
      | MBA
      | JD
      | MD
      | PhD
      | PostDoc
      | Certificate
      | Diploma
      | Bootcamp
      | OnlineCourse
      | Other : Text
      >

-- Skill Proficiency Level
let SkillLevel =
      < Beginner
      | Intermediate
      | Advanced
      | Expert
      | Master
      >

-- Language Fluency Level (ILR scale)
let FluencyLevel =
      < Elementary
      | LimitedWorking
      | ProfessionalWorking
      | FullProfessional
      | NativeOrBilingual
      >

-- Spoken Languages (expanded)
let LanguageType =
      < Afrikaans
      | Albanian
      | Amharic
      | Arabic
      | Armenian
      | Azerbaijani
      | Basque
      | Belarusian
      | Bengali
      | Bosnian
      | Bulgarian
      | Burmese
      | Catalan
      | Cebuano
      | Chinese
      | Mandarin
      | Cantonese
      | Croatian
      | Czech
      | Danish
      | Dutch
      | English
      | Esperanto
      | Estonian
      | Finnish
      | French
      | Galician
      | Georgian
      | German
      | Greek
      | Gujarati
      | HaitianCreole
      | Hausa
      | Hawaiian
      | Hebrew
      | Hindi
      | Hmong
      | Hungarian
      | Icelandic
      | Igbo
      | Indonesian
      | Irish
      | Italian
      | Japanese
      | Javanese
      | Kannada
      | Kazakh
      | Khmer
      | Korean
      | Kurdish
      | Kyrgyz
      | Lao
      | Latin
      | Latvian
      | Lithuanian
      | Luxembourgish
      | Macedonian
      | Malagasy
      | Malay
      | Malayalam
      | Maltese
      | Maori
      | Marathi
      | Mongolian
      | Nepali
      | Norwegian
      | Pashto
      | Persian
      | Polish
      | Portuguese
      | Punjabi
      | Romanian
      | Russian
      | Samoan
      | ScottishGaelic
      | Serbian
      | Shona
      | Sindhi
      | Sinhala
      | Slovak
      | Slovenian
      | Somali
      | Spanish
      | Sundanese
      | Swahili
      | Swedish
      | Tagalog
      | Tajik
      | Tamil
      | Telugu
      | Thai
      | Turkish
      | Ukrainian
      | Urdu
      | Uzbek
      | Vietnamese
      | Welsh
      | Xhosa
      | Yiddish
      | Yoruba
      | Zulu
      | Other : Text
      >

-- Project Type (expanded for non-programming activities)
let ProjectType =
      < Application
      | Website
      | Library
      | Framework
      | Research
      | OpenSource
      | Conference
      | Talk
      | Presentation
      | Workshop
      | Tutorial
      | Documentation
      | Volunteering
      | Publication
      | Article
      | BlogPost
      | Book
      | Podcast
      | Video
      | Art
      | Music
      | Photography
      | Design
      | Film
      | Theater
      | Dance
      | Writing
      | Poetry
      | Community
      | Activism
      | Fundraising
      | Mentorship
      | Teaching
      | Sports
      | Competition
      | Hackathon
      | Exhibition
      | Performance
      | Installation
      | Sculpture
      | Other : Text
      >

-- Employment Type
let EmploymentType =
      < FullTime
      | PartTime
      | Contract
      | Freelance
      | Internship
      | Apprenticeship
      | Seasonal
      | SelfEmployed
      | Volunteer
      >

-- Social Network Type
let NetworkType =
      < GitHub
      | GitLab
      | Bitbucket
      | LinkedIn
      | Twitter
      | Mastodon
      | Facebook
      | Instagram
      | YouTube
      | Twitch
      | TikTok
      | Reddit
      | StackOverflow
      | HackerNews
      | Medium
      | DevTo
      | Hashnode
      | Substack
      | Personal
      | Portfolio
      | Blog
      | Dribbble
      | Behance
      | Figma
      | CodePen
      | Glitch
      | Replit
      | Observable
      | Kaggle
      | Discord
      | Slack
      | Telegram
      | WhatsApp
      | Signal
      | Email
      | Website
      | Other : Text
      >

-- Social Profile
let Profile =
      { Type =
          { network : Optional NetworkType
          , username : Optional Text
          , url : Optional Text
          }
      , default =
          { network = None NetworkType, username = None Text, url = None Text }
      }

      let PronounSet =
      { subject : Text  -- he, she, they
      , object : Text   -- him, her, them
      , possessiveAdj : Text  -- his, her, their
      , possessivePronoun : Text  -- his, hers, theirs
      , reflexive : Text  -- himself, herself, themself/themselves
      }

let Pronouns =
      { primary : Optional PronounSet
      , additional : List PronounSet
      , display : Text
      }

-- Basic Information
let Basics =
      { Type =
          { name : Optional Text
          , label : Optional Text
          , pronouns: Optional Pronouns 
          , image : Optional Text
          , email : Optional Text
          , phone : Optional Text
          , url : Optional Text
          , summary : Optional Text
          , location : Optional Address.Type
          , profiles : Optional (List Profile.Type)
          }
      , default =
          { name = None Text
          , pronouns = None Pronouns
          , label = None Text
          , image = None Text
          , email = None Text
          , phone = None Text
          , url = None Text
          , summary = None Text
          , location = None Address.Type
          , profiles = None (List Profile.Type)
          }
      }

-- Work Experience
let Work =
      { Type =
          { name : Optional Text
          , location : Optional LocationType
          , description : Optional Text
          , position : Optional Text
          , url : Optional Text
          , startDate : Optional Text
          , endDate : Optional Text
          , summary : Optional Text
          , highlights : Optional (List Text)
          , employmentType : Optional EmploymentType
          }
      , default =
          { name = None Text
          , location = None LocationType
          , description = None Text
          , position = None Text
          , url = None Text
          , startDate = None Text
          , endDate = None Text
          , summary = None Text
          , highlights = None (List Text)
          , employmentType = None EmploymentType
          }
      }

-- Volunteer Experience
let Volunteer =
      { Type =
          { organization : Optional Text
          , position : Optional Text
          , url : Optional Text
          , location : Optional LocationType
          , startDate : Optional Text
          , endDate : Optional Text
          , summary : Optional Text
          , highlights : Optional (List Text)
          }
      , default =
          { organization = None Text
          , position = None Text
          , url = None Text
          , location = None LocationType
          , startDate = None Text
          , endDate = None Text
          , summary = None Text
          , highlights = None (List Text)
          }
      }

-- Education
let Education =
      { Type =
          { institution : Optional Text
          , url : Optional Text
          , area : Optional Text
          , studyType : Optional DegreeType
          , startDate : Optional Text
          , endDate : Optional Text
          , score : Optional Score
          , courses : Optional (List Text)
          }
      , default =
          { institution = None Text
          , url = None Text
          , area = None Text
          , studyType = None DegreeType
          , startDate = None Text
          , endDate = None Text
          , score = None Score
          , courses = None (List Text)
          }
      }

-- Award
let Award =
      { Type =
          { title : Optional Text
          , date : Optional Text
          , awarder : Optional Text
          , summary : Optional Text
          }
      , default =
          { title = None Text
          , date = None Text
          , awarder = None Text
          , summary = None Text
          }
      }

-- Certificate
let Certificate =
      { Type =
          { name : Optional Text
          , date : Optional Text
          , url : Optional Text
          , issuer : Optional Text
          , expirationDate : Optional Text
          }
      , default =
          { name = None Text
          , date = None Text
          , url = None Text
          , issuer = None Text
          , expirationDate = None Text
          }
      }

-- Publication
let Publication =
      { Type =
          { name : Optional Text
          , publisher : Optional Text
          , releaseDate : Optional Text
          , url : Optional Text
          , summary : Optional Text
          }
      , default =
          { name = None Text
          , publisher = None Text
          , releaseDate = None Text
          , url = None Text
          , summary = None Text
          }
      }

-- Skill
let Skill =
      { Type =
          { name : Optional Text
          , level : Optional SkillLevel
          , keywords : Optional (List Text)
          }
      , default =
          { name = None Text
          , level = None SkillLevel
          , keywords = None (List Text)
          }
      }

-- Language (Spoken only - programming languages go in Skills)
let Language =
      { Type =
          { language : Optional LanguageType
          , fluency : Optional FluencyLevel
          }
      , default =
          { language = None LanguageType, fluency = None FluencyLevel }
      }

-- Interest
let Interest =
      { Type = { name : Optional Text, keywords : Optional (List Text) }
      , default = { name = None Text, keywords = None (List Text) }
      }

-- Reference
let Reference =
      { Type = { name : Optional Text, reference : Optional Text }
      , default = { name = None Text, reference = None Text }
      }

-- Project
let Project =
      { Type =
          { name : Optional Text
          , description : Optional Text
          , highlights : Optional (List Text)
          , keywords : Optional (List Text)
          , startDate : Optional Text
          , endDate : Optional Text
          , url : Optional Text
          , roles : Optional (List Text)
          , entity : Optional Text
          , type : Optional ProjectType
          }
      , default =
          { name = None Text
          , description = None Text
          , highlights = None (List Text)
          , keywords = None (List Text)
          , startDate = None Text
          , endDate = None Text
          , url = None Text
          , roles = None (List Text)
          , entity = None Text
          , type = None ProjectType
          }
      }

-- Meta
let Meta =
      { Type =
          { canonical : Optional Text
          , version : Optional SemVer.Type
          , lastModified : Optional { date : Text, time : Text }
          }
      , default =
          { canonical = None Text
          , version = None SemVer.Type
          , lastModified = None { date : Text, time : Text }
          }
      }

-- Complete Resume Schema
let Resume =
      { Type =
          { basics : Optional Basics.Type
          , work : Optional (List Work.Type)
          , volunteer : Optional (List Volunteer.Type)
          , education : Optional (List Education.Type)
          , awards : Optional (List Award.Type)
          , certificates : Optional (List Certificate.Type)
          , publications : Optional (List Publication.Type)
          , skills : Optional (List Skill.Type)
          , languages : Optional (List Language.Type)
          , interests : Optional (List Interest.Type)
          , references : Optional (List Reference.Type)
          , projects : Optional (List Project.Type)
          , meta : Optional Meta.Type
          }
      , default =
          { basics = None Basics.Type
          , work = None (List Work.Type)
          , volunteer = None (List Volunteer.Type)
          , education = None (List Education.Type)
          , awards = None (List Award.Type)
          , certificates = None (List Certificate.Type)
          , publications = None (List Publication.Type)
          , skills = None (List Skill.Type)
          , languages = None (List Language.Type)
          , interests = None (List Interest.Type)
          , references = None (List Reference.Type)
          , projects = None (List Project.Type)
          , meta = None Meta.Type
          }
      }

in  { Address
    , Profile
    , Basics
    , Work
    , Volunteer
    , Education
    , Award
    , Certificate
    , Publication
    , Skill
    , Language
    , Interest
    , Reference
    , Project
    , Meta
    , Resume
    , DegreeType
    , SkillLevel
    , FluencyLevel
    , LanguageType
    , ProjectType
    , EmploymentType
    , LocationType
    , NetworkType
    , Score
    , Pronouns
    , PronounSet
    , SemVer
    }
