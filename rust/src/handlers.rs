use serde::{Deserialize, Serialize};

use crate::webhook_events::{IssueEvent, Label, MergeRequestEvent, NoteEvent, PipelineEvent, PushEvent, TagPushEvent, WebhookEvent, WikiPageEvent};

#[derive(Deserialize)]
pub struct Request {
    pub body: WebhookEvent,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct SlackMessage {
    pub r#type: String,
    pub text: String,
}

impl SlackMessage {
    pub fn markdown(text: &str) -> SlackMessage {
        SlackMessage {
            r#type: "mrkdwn".to_string(),
            text: text.to_string(),
        }
    }
    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct HandleEventStatus {
    pub status: String,
    pub message: Option<SlackMessage>,
    pub error: Option<String>,
}

impl HandleEventStatus {
    pub fn ignored() -> HandleEventStatus {
        HandleEventStatus {
            status: "ignored".to_string(),
            message: None,
            error: None,
        }
    }
    pub fn sent(message: SlackMessage) -> HandleEventStatus {
        HandleEventStatus {
            status: "sent".to_string(),
            message: Some(message),
            error: None,
        }
    }
    pub fn error(msg: String) -> HandleEventStatus {
        HandleEventStatus {
            status: "error".to_string(),
            message: None,
            error: Some(msg),
        }
    }
}

fn get_labels_markdown(labels: &Vec<Label>) -> String {
    match labels.len() {
        0 => "".to_string(),
        _ => {
            let mut result = String::new();
            for label in labels {
                let s = format!("`{}`", label.title);
                match result.len() {
                    0 => result.push_str(s.as_str()),
                    _ => result.push_str(format!(", {}", s).as_str()),
                }
            }
            format!("[{}]", result)
        }
    }
}

fn handle_issue_event(_: IssueEvent) -> Option<SlackMessage> {
    None
}

fn handle_merge_request_event(event: MergeRequestEvent) -> Option<SlackMessage> {
    let action_user = event.user;
    let action_user_id = action_user.id;
    let action_user_name = action_user.name;
    let project_name = event.project.name;
    let mr = event.object_attributes;
    let mr_link = format!("<{}|{} MR !{}>", mr.url, project_name, mr.iid);
    let title = mr.title;
    let author_id = mr.author_id;
    let labels_md = get_labels_markdown(&mr.labels);
    let src_branch = mr.source_branch;
    let target_branch = mr.target_branch;
    let show_author = action_user_id != author_id;

    match mr.action {
        Some(action) => {
            match action.as_str() {
                "approve" => {
                    match show_author {
                        true => {
                            let author_name = format!("{}", author_id);
                            Some(SlackMessage::markdown(format!(":white_check_mark: {} approved {}'s {} *{}*.", action_user_name, author_name, mr_link, title).as_str()))
                        }
                        false => {
                            Some(SlackMessage::markdown(format!(":white_check_mark: {} approved {} *{}*.", action_user_name, mr_link, title).as_str()))
                        }
                    }
                }
                "close" => {
                    Some(SlackMessage::markdown(format!(":no_entry: {} closed {} *{}*.", action_user_name, mr_link, title).as_str()))
                }
                "merge" => {
                    match show_author {
                        true => {
                            let author_name = format!("{}", author_id);
                            Some(SlackMessage::markdown(format!(":tada: {} merged {}'s {} *{}*.", action_user_name, author_name, mr_link, title).as_str()))
                        }
                        false => Some(SlackMessage::markdown(format!(":tada: {} merged {} *{}*.", action_user_name, mr_link, title).as_str()))
                    }
                }
                "open" => Some(SlackMessage::markdown(format!(":blush: {} opened {} *{}*{}.\n`{}` → `{}`", action_user_name, mr_link, title, labels_md, src_branch, target_branch).as_str())),
                "unapprove" => {
                    match show_author {
                        true => {
                            let author_name = format!("{}", author_id);
                            Some(SlackMessage::markdown(format!("{} unapproved {}'s {} *{}*.", action_user_name, author_name, mr_link, title).as_str()))
                        }
                        false => Some(SlackMessage::markdown(format!("{} unapproved {} *{}*.", action_user_name, mr_link, title).as_str()))
                    }
                }
                _ => None
            }
        }
        None => None
    }
}

fn handle_note_event(_: NoteEvent) -> Option<SlackMessage> {
    None
}

fn handle_pipeline_event(event: PipelineEvent) -> Option<SlackMessage> {
    let pipeline = event.object_attributes;
    if pipeline.status == "failed" {
        let commit = event.commit;
        let merge_request = event.merge_request;
        let project = event.project;
        let committer = format!("{}<{}>", commit.author.name, commit.author.email);

        let action_user = match merge_request {
            Some(_) => event.user.name.as_str(),
            None => ""
        };
        let project_link = format!("<{}|{} project>", project.web_url, project.name);
        Some(SlackMessage::markdown(format!(":fire: {} Build pipeline failed on {} `{}`.\n- `{}` *{}*", action_user, project_link, pipeline.r#ref, committer, commit.title).as_str()))
    } else {
        None
    }
}

fn handle_push_event(_: PushEvent) -> Option<SlackMessage> {
    None
}

fn handle_tag_push_event(_: TagPushEvent) -> Option<SlackMessage> {
    None
}

fn handle_wiki_page_event(_: WikiPageEvent) -> Option<SlackMessage> {
    None
}


fn build_slack_message(event: WebhookEvent) -> Option<SlackMessage> {
    match event {
        WebhookEvent::Issue(e) => handle_issue_event(e),
        WebhookEvent::MR(e) => handle_merge_request_event(e),
        WebhookEvent::Note(e) => handle_note_event(e),
        WebhookEvent::Pipeline(e) => handle_pipeline_event(e),
        WebhookEvent::Push(e) => handle_push_event(e),
        WebhookEvent::TagPush(e) => handle_tag_push_event(e),
        WebhookEvent::Wiki(e) => handle_wiki_page_event(e),
    }
}

fn send_slack_message(message: &SlackMessage) {
    let client = reqwest::blocking::Client::new();

    let res = client.post("https://hooks.slack.com/services/T00000000/XXXXXXXXXXX/xxxxxxxxxxxxxxxxxxxxxxxx")
        .body(message.json())
        .send()?;
    if res.status().is_success() {
        println!("[SEND] {}", message.json());
    } else {
        println!("[FAIL] status: {}", res.status());
    }
}

pub async fn handle_webhook_event(event: WebhookEvent) -> Result<HandleEventStatus, String> {
    match build_slack_message(event) {
        Some(message) => {
            send_slack_message(&message);
            Ok(HandleEventStatus::sent(message))
        }
        None => Ok(HandleEventStatus::ignored()),
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::SlackMessage;
    use crate::webhook_events::webhook_event_from_file;

    use super::*;

    #[tokio::test]
    async fn test_merge_request_open() {
        let event = webhook_event_from_file("../events/mr-open.json").unwrap();
        let message = build_slack_message(event);
        assert_eq!(message, Some(SlackMessage::markdown(
            ":blush: LechuckRoh opened <https://gitlab.com/lechuckroh/gitlab-slack-notifier/-/merge_requests/1|gitlab-slack-notifier MR !1> *Rust Lambda*[`enhancement`].\n`feature/rust` → `develop`"
        )));
    }

    #[tokio::test]
    async fn test_merge_request_approve() {
        let event = webhook_event_from_file("../events/mr-approve.json").unwrap();
        let message = build_slack_message(event);
        assert_eq!(message, Some(SlackMessage::markdown(
            ":white_check_mark: LechuckRoh approved <https://gitlab.com/lechuckroh/gitlab-slack-notifier/-/merge_requests/1|gitlab-slack-notifier MR !1> *Rust Lambda*."
        )));
    }

    #[tokio::test]
    async fn test_merge_request_merge() {
        let event = webhook_event_from_file("../events/mr-merge.json").unwrap();
        let message = build_slack_message(event);
        assert_eq!(message, Some(SlackMessage::markdown(
            ":tada: LechuckRoh merged <https://gitlab.com/lechuckroh/gitlab-slack-notifier/-/merge_requests/1|gitlab-slack-notifier MR !1> *Rust Lambda*."
        )));
    }

    #[tokio::test]
    async fn test_pipeline_failed() {
        let event = webhook_event_from_file("../events/pipeline-failed.json").unwrap();
        let message = build_slack_message(event);
        assert_eq!(message, Some(SlackMessage::markdown(
            ":fire: Admin Build pipeline failed on <http://192.168.64.1:3005/gitlab-org/gitlab-test|Gitlab Test project> `master`.\n- `User<user@gitlab.com>` *test*"
        )));
    }
}
