extern crate serde_json;
extern crate termion;

extern crate hyphenation;
extern crate textwrap;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Label {
  pub name: String,
  pub description: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
  pub id: i64,
  pub key: String,
  pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IssueFields {
    pub summary: String,
    pub description: String,

    pub assignee: User,
    pub creator: User,
    pub reporter: User,

    pub project: Project,

    pub status: Label,

    #[serde(rename = "issuetype")]
    pub issue_type: Label
}

#[derive(Deserialize, Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields
}

pub fn print(issue: Issue) {
    use self::termion::color;
    use self::termion::style;

    use self::hyphenation::Language;
    use self::textwrap::Wrapper;

    let corpus = hyphenation::load(Language::English_US).unwrap();
    let mut wrapper = Wrapper::new(80);
    wrapper.corpus = Some(&corpus);

    // Header
    println!("");
    println!("{}[{}] {}{}{}{}",
             style::Bold,
             issue.key,
             color::Fg(color::Green),
             issue.fields.summary,
             color::Fg(color::Reset),
             style::Reset);

    // Body
    println!("");
    println!("{}", wrapper.fill(issue.fields.description.as_str()));
    println!("");

    // Metadata
    println!("* {}Creator:{} {}", style::Bold, style::Reset,  issue.fields.creator.display_name);
    println!("* {}Assignee:{} {}", style::Bold, style::Reset, issue.fields.assignee.display_name);
    println!("* {}Reporter:{} {}", style::Bold, style::Reset, issue.fields.reporter.display_name);

    println!("");
}
