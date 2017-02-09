extern crate serde_json;
extern crate termion;

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

pub fn load_from_json(contents: String) -> Issue {
    serde_json::from_str(&*contents).unwrap()
}

pub fn print(issue: Issue) {
    use self::termion::color::{ Fg, Red, Reset };

    println!("[{}] {}{}{}", issue.key, Fg(Red), issue.fields.summary, Fg(Reset));
    println!("");
    println!("{}", issue.fields.description);
}
