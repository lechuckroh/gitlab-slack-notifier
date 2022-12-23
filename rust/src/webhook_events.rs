use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct ArtifactsFile {
    filename: Option<String>,
    size: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Build {
    id: u32,
    stage: String,
    name: String,
    status: String,
    created_at: String,
    started_at: Option<String>,
    finished_at: Option<String>,
    when: String,
    manual: bool,
    user: User,
    artifacts_file: ArtifactsFile,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Commit {
    id: String,
    message: String,
    title: String,
    timestamp: String,
    url: String,
    author: Author,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Issue {
    id: u32,
    title: String,
    assignee_ids: Vec<u32>,
    assignee_id: u32,
    author_id: u32,
    project_id: u32,
    created_at: String,
    updated_at: String,
    branch_name: Option<String>,
    description: String,
    milestone_id: Option<u32>,
    state: String,
    iid: u32,
    labels: Vec<Label>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct IssueAttributes {
    id: u32,
    title: String,
    assignee_ids: Vec<u32>,
    assignee_id: u32,
    author_id: u32,
    project_id: u32,
    created_at: String,
    updated_at: String,
    updated_by_id: Option<u32>,
    last_edited_at: Option<String>,
    last_edited_by_id: Option<u32>,
    description: String,
    milestone_id: Option<u32>,
    state_id: u32,
    confidential: bool,
    iid: u32,
    url: String,
    state: String,
    action: String,
    severity: String,
    labels: Vec<Label>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Label {
    id: u32,
    title: String,
    color: String,
    project_id: u32,
    created_at: String,
    updated_at: String,
    template: bool,
    description: Option<String>,
    r#type: String,
    group_id: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LastCommit {
    id: String,
    message: String,
    timestamp: String,
    url: String,
    author: Author,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MergeRequest {
    id: u32,
    target_branch: String,
    source_branch: String,
    source_project_id: u32,
    author_id: u32,
    assignee_id: u32,
    title: String,
    state: String,
    merge_status: String,
    target_project_id: u32,
    iid: u32,
    description: String,
    labels: Vec<Label>,
    url: String,
    source: Project,
    target: Project,
    work_in_progress: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Project {
    id: u32,
    name: String,
    description: String,
    web_url: String,
    avatar_url: Option<String>,
    git_ssh_url: String,
    git_http_url: String,
    namespace: String,
    visibility_level: u32,
    path_with_namespace: String,
    default_branch: String,
    ci_config_path: Option<String>,
    homepage: String,
    url: String,
    ssh_url: String,
    http_url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Repository {
    name: String,
    url: String,
    description: String,
    homepage: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Snippet {
    id: u32,
    title: String,
    content: String,
    author_id: u32,
    project_id: u32,
    created_at: String,
    updated_at: String,
    file_name: String,
    expires_at: Option<String>,
    r#type: String,
    visibility_level: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    id: u32,
    name: String,
    username: String,
    avatar_url: String,
    email: Option<String>,
}

// --------------------------------------------------
// Events
// --------------------------------------------------

#[derive(Deserialize, Debug, PartialEq)]
pub struct IssueEvent {
    event_type: String,
    user: User,
    project: Project,
    object_attributes: IssueAttributes,
    repository: Repository,
    assignees: Vec<User>,
    labels: Vec<Label>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MergeRequestAttributes {
    assignee_id: Option<u32>,
    author_id: u32,
    description: String,
    iid: u32,
    target_branch: String,
    source_branch: String,
    source_project_id: u32,
    assignee_ids: Vec<u32>,
    title: String,
    state: String,
    blocking_discussions_resolved: bool,
    work_in_progress: bool,
    merge_status: String,
    target_project_id: u32,
    url: String,
    source: Project,
    target: Project,
    labels: Vec<Label>,
    action: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MergeRequestEvent {
    event_type: String,
    user: User,
    project: Project,
    repository: Repository,
    object_attributes: MergeRequestAttributes,
    labels: Vec<Label>,
    assignees: Vec<User>,
    reviewers: Vec<User>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NoteAttributes {
    author_id: u32,
    commit_id: Option<String>,
    id: u32,
    note: String,
    noteable_type: String,
    project_id: u32,
    description: String,
    url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NoteEvent {
    event_type: String,
    user: User,
    project_id: u32,
    project: Project,
    repository: Repository,
    object_attributes: NoteAttributes,
    commit: Option<Commit>,
    merge_request: Option<MergeRequest>,
    issue: Option<Issue>,
    snippet: Option<Snippet>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PipelineAttributes {
    id: u32,
    iid: u32,
    r#ref: String,
    tag: bool,
    sha: String,
    before_sha: String,
    source: String,
    status: String,
    stages: Vec<String>,
    created_at: String,
    finished_at: String,
    duration: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PipelineEvent {
    object_attributes: PipelineAttributes,
    user: User,
    project: Project,
    commit: Commit,
    builds: Vec<Build>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PushEvent {
    event_name: String,
    before: String,
    after: String,
    r#ref: String,
    checkout_sha: String,
    user_id: u32,
    user_name: String,
    user_username: String,
    user_email: String,
    user_avatar: String,
    project_id: u32,
    project: Project,
    repository: Repository,
    commits: Vec<Commit>,
    total_commits_count: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TagPushEvent {
    object_kind: String,
    event_name: String,
    before: String,
    after: String,
    r#ref: String,
    checkout_sha: String,
    user_id: u32,
    user_name: String,
    user_avatar: String,
    project_id: u32,
    project: Project,
    repository: Repository,
    commits: Vec<Commit>,
    total_commits_count: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Wiki {
    web_url: String,
    git_ssh_url: String,
    git_http_url: String,
    path_with_namespace: String,
    default_branch: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct WikiPageAttributes {
    title: String,
    content: String,
    format: String,
    message: String,
    slug: String,
    url: String,
    action: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct WikiPageEvent {
    user: User,
    project: Project,
    wiki: Wiki,
    object_attributes: WikiPageAttributes,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "object_kind")]
pub enum Body {
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

#[derive(Deserialize, Debug, PartialEq)]
pub struct Payload {
    body: Body,
}

// --------------------------------------------------
// Functions
// --------------------------------------------------

// pub fn payload_from_str(s: &str) -> Result<Payload, Box<dyn Error>> {
//     let payload = serde_json::from_str(s)?;
//     Ok(payload)
// }

pub fn payload_from_file<P: AsRef<Path>>(path: P) -> Result<Payload, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let payload = serde_json::from_reader(reader)?;
    Ok(payload)
}

// --------------------------------------------------
// Tests
// --------------------------------------------------

#[test]
fn it_issue_open_event_from_file() {
    let payload = payload_from_file("../events/issue-open.json").unwrap();
    match payload.body {
        Body::Issue(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_commit_event_from_file() {
    let payload = payload_from_file("../events/note-commit.json").unwrap();
    match payload.body {
        Body::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_issue_event_from_file() {
    let payload = payload_from_file("../events/note-issue.json").unwrap();
    match payload.body {
        Body::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_mr_event_from_file() {
    let payload = payload_from_file("../events/note-mr-mergeable.json").unwrap();
    match payload.body {
        Body::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_push_event_from_file() {
    let payload = payload_from_file("../events/push.json").unwrap();
    match payload.body {
        Body::Push(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_note_snippet_event_from_file() {
    let payload = payload_from_file("../events/note-snippet.json").unwrap();
    match payload.body {
        Body::Note(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_mr_open_event_from_file() {
    let payload = payload_from_file("../events/mr-open.json").unwrap();
    match payload.body {
        Body::MR(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_tag_push_event_from_file() {
    let payload = payload_from_file("../events/tag-push.json").unwrap();
    match payload.body {
        Body::TagPush(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}

#[test]
fn it_wiki_create_event_from_file() {
    let payload = payload_from_file("../events/wiki-create.json").unwrap();
    match payload.body {
        Body::Wiki(_) => assert!(true),
        _ => assert!(false, "not handled")
    }
}
