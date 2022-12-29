use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct ArtifactsFile {
    pub filename: Option<String>,
    pub size: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Build {
    pub id: u32,
    pub stage: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub when: String,
    pub manual: bool,
    pub user: User,
    pub artifacts_file: ArtifactsFile,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub title: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Issue {
    pub id: u32,
    pub title: String,
    pub assignee_ids: Vec<u32>,
    pub assignee_id: u32,
    pub author_id: u32,
    pub project_id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub branch_name: Option<String>,
    pub description: String,
    pub milestone_id: Option<u32>,
    pub state: String,
    pub iid: u32,
    pub labels: Vec<Label>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct IssueAttributes {
    pub id: u32,
    pub title: String,
    pub assignee_ids: Vec<u32>,
    pub assignee_id: u32,
    pub author_id: u32,
    pub project_id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub updated_by_id: Option<u32>,
    pub last_edited_at: Option<String>,
    pub last_edited_by_id: Option<u32>,
    pub description: String,
    pub milestone_id: Option<u32>,
    pub state_id: u32,
    pub confidential: bool,
    pub iid: u32,
    pub url: String,
    pub state: String,
    pub action: String,
    pub severity: String,
    pub labels: Vec<Label>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Label {
    pub id: u32,
    pub title: String,
    pub color: String,
    pub project_id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub template: bool,
    pub description: Option<String>,
    pub r#type: String,
    pub group_id: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LastCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MergeRequest {
    pub id: u32,
    pub target_branch: String,
    pub source_branch: String,
    pub source_project_id: u32,
    pub author_id: u32,
    pub assignee_id: u32,
    pub title: String,
    pub state: String,
    pub merge_status: String,
    pub target_project_id: u32,
    pub iid: u32,
    pub description: String,
    pub labels: Vec<Label>,
    pub url: String,
    pub source: Project,
    pub target: Project,
    pub work_in_progress: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: u32,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub ci_config_path: Option<String>,
    pub homepage: Option<String>,
    pub url: Option<String>,
    pub ssh_url: Option<String>,
    pub http_url: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: String,
    pub homepage: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Snippet {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
    pub project_id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub file_name: String,
    pub expires_at: Option<String>,
    pub r#type: String,
    pub visibility_level: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub avatar_url: String,
    pub email: Option<String>,
}

// --------------------------------------------------
// Events
// --------------------------------------------------

#[derive(Deserialize, Debug, PartialEq)]
pub struct IssueEvent {
    pub event_type: String,
    pub user: User,
    pub project: Project,
    pub object_attributes: IssueAttributes,
    pub repository: Repository,
    pub assignees: Vec<User>,
    pub labels: Vec<Label>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MergeRequestAttributes {
    pub assignee_id: Option<u32>,
    pub author_id: u32,
    pub description: String,
    pub iid: u32,
    pub target_branch: String,
    pub source_branch: String,
    pub source_project_id: u32,
    pub assignee_ids: Vec<u32>,
    pub title: String,
    pub state: String,
    pub blocking_discussions_resolved: bool,
    pub work_in_progress: bool,
    pub merge_status: String,
    pub target_project_id: u32,
    pub url: String,
    pub source: Project,
    pub target: Project,
    pub labels: Vec<Label>,
    pub action: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MergeRequestEvent {
    pub event_type: String,
    pub user: User,
    pub project: Project,
    pub repository: Repository,
    pub object_attributes: MergeRequestAttributes,
    pub labels: Vec<Label>,
    pub assignees: Vec<User>,
    pub reviewers: Vec<User>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NoteAttributes {
    pub author_id: u32,
    pub commit_id: Option<String>,
    pub id: u32,
    pub note: String,
    pub noteable_type: String,
    pub project_id: u32,
    pub description: String,
    pub url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NoteEvent {
    pub event_type: String,
    pub user: User,
    pub project_id: u32,
    pub project: Project,
    pub repository: Repository,
    pub object_attributes: NoteAttributes,
    pub commit: Option<Commit>,
    pub merge_request: Option<MergeRequest>,
    pub issue: Option<Issue>,
    pub snippet: Option<Snippet>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PipelineAttributes {
    pub id: u32,
    pub iid: u32,
    pub r#ref: String,
    pub tag: bool,
    pub sha: String,
    pub before_sha: String,
    pub source: String,
    pub status: String,
    pub stages: Vec<String>,
    pub created_at: String,
    pub finished_at: String,
    pub duration: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PipelineMergeRequestAttributes {
    pub id: u32,
    pub iid: u32,
    pub title: String,
    pub source_branch: String,
    pub source_project_id: u32,
    pub target_branch: String,
    pub target_project_id: u32,
    pub state: String,
    pub merge_status: String,
    pub detailed_merge_status: String,
    pub url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PipelineEvent {
    pub object_attributes: PipelineAttributes,
    pub merge_request: Option<PipelineMergeRequestAttributes>,
    pub user: User,
    pub project: Project,
    pub commit: Commit,
    pub builds: Vec<Build>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PushEvent {
    pub event_name: String,
    pub before: String,
    pub after: String,
    pub r#ref: String,
    pub checkout_sha: String,
    pub user_id: u32,
    pub user_name: String,
    pub user_username: String,
    pub user_email: String,
    pub user_avatar: String,
    pub project_id: u32,
    pub project: Project,
    pub repository: Repository,
    pub commits: Vec<Commit>,
    pub total_commits_count: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TagPushEvent {
    pub event_name: String,
    pub before: String,
    pub after: String,
    pub r#ref: String,
    pub checkout_sha: String,
    pub user_id: u32,
    pub user_name: String,
    pub user_avatar: String,
    pub project_id: u32,
    pub project: Project,
    pub repository: Repository,
    pub commits: Vec<Commit>,
    pub total_commits_count: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Wiki {
    pub web_url: String,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub path_with_namespace: String,
    pub default_branch: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct WikiPageAttributes {
    pub title: String,
    pub content: String,
    pub format: String,
    pub message: String,
    pub slug: String,
    pub url: String,
    pub action: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct WikiPageEvent {
    pub user: User,
    pub project: Project,
    pub wiki: Wiki,
    pub object_attributes: WikiPageAttributes,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "object_kind")]
pub enum WebhookEvent {
    #[serde(rename = "issue")]
    Issue(IssueEvent),
    #[serde(rename = "merge_request")]
    MR(MergeRequestEvent),
    #[serde(rename = "note")]
    Note(NoteEvent),
    #[serde(rename = "pipeline")]
    Pipeline(PipelineEvent),
    #[serde(rename = "push")]
    Push(PushEvent),
    #[serde(rename = "tag_push")]
    TagPush(TagPushEvent),
    #[serde(rename = "wiki_page")]
    Wiki(WikiPageEvent),
}

// --------------------------------------------------
// Functions
// --------------------------------------------------

pub fn payload_from_str(s: &str) -> Result<WebhookEvent, Box<dyn Error>> {
    let payload = serde_json::from_str(s)?;
    Ok(payload)
}

pub fn webhook_event_from_file<P: AsRef<Path>>(path: P) -> Result<WebhookEvent, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let event = serde_json::from_reader(reader)?;
    Ok(event)
}

// --------------------------------------------------
// Tests
// --------------------------------------------------

#[test]
fn it_issue_open_event_from_file() {
    let event = webhook_event_from_file("../events/issue-open.json").unwrap();
    match event {
        WebhookEvent::Issue(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_commit_event_from_file() {
    let event = webhook_event_from_file("../events/note-commit.json").unwrap();
    match event {
        WebhookEvent::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_issue_event_from_file() {
    let event = webhook_event_from_file("../events/note-issue.json").unwrap();
    match event {
        WebhookEvent::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_mr_event_from_file() {
    let event = webhook_event_from_file("../events/note-mr-mergeable.json").unwrap();
    match event {
        WebhookEvent::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_push_event_from_file() {
    let event = webhook_event_from_file("../events/push.json").unwrap();
    match event {
        WebhookEvent::Push(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_snippet_event_from_file() {
    let event = webhook_event_from_file("../events/note-snippet.json").unwrap();
    match event {
        WebhookEvent::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_mr_open_event_from_file() {
    let event = webhook_event_from_file("../events/mr-open.json").unwrap();
    match event {
        WebhookEvent::MR(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_tag_push_event_from_file() {
    let event = webhook_event_from_file("../events/tag-push.json").unwrap();
    match event {
        WebhookEvent::TagPush(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_wiki_create_event_from_file() {
    let event = webhook_event_from_file("../events/wiki-create.json").unwrap();
    match event {
        WebhookEvent::Wiki(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}
