export interface ArtifactsFile {
  filename: string | null;
  size: number | null;
}

export interface Author {
  name: string;
  email: string;
}

export interface Build {
  id: number;
  stage: string;
  name: string;
  status: string;
  created_at: string;
  started_at?: string;
  finished_at?: string;
  when: string;
  manual: boolean;
  user: User;
  runner: unknown;
  artifacts_file: ArtifactsFile;
}

export interface Changes {
  updated_by_id: {
    previous: number | null;
    current: number;
  };
  updated_at: {
    previous: string;
    current: string;
  };
  labels: {
    previous: Label[];
    current: Label[];
  };
}

export interface Commit {
  id: string;
  message: string;
  title: string;
  timestamp: string;
  url: string;
  author: Author;
}

export interface Issue {
  id: number;
  title: string;
  assignee_ids: number[];
  assignee_id: number;
  author_id: number;
  project_id: number;
  created_at: string;
  updated_at: string;
  position: number;
  branch_name: string | null;
  description: string;
  milestone_id: number | null;
  state: string;
  iid: number;
  labels: Label[];
}

export interface IssueAttributes {
  id: number;
  title: string;
  assignee_ids: number[];
  assignee_id: number;
  author_id: number;
  project_id: number;
  created_at: string;
  updated_at: string;
  updated_by_id: number;
  last_edited_at: string | null;
  last_edited_by_id: number | null;
  relative_position: number;
  description: string;
  milestone_id: number;
  state_id: number;
  confidential: boolean;
  discussion_locked: boolean;
  due_date: string | null;
  moved_to_id: number | null;
  duplicated_to_id: number | null;
  time_estimate: number;
  total_time_spent: number;
  time_change: number;
  human_total_time_spent: number | null;
  human_time_estimate: number | null;
  human_time_change: number | null;
  weight: number | null;
  iid: number;
  url: string;
  state: string;
  action: "open" | "close" | "reopen" | "update";
  severity: string;
  escalation_status: string;
  escalation_policy: {
    id: number;
    name: string;
  };
  labels: Label[];
}

export interface Label {
  id: number;
  title: string;
  color: string;
  project_id: number;
  created_at: string;
  updated_at: string;
  template: boolean;
  description: string;
  type: string;
  group_id: number;
}

export interface LastCommit {
  id: string;
  message: string;
  title: string;
  timestamp: string;
  url: string;
  author: Author;
}

export interface MergeRequest {
  id: number;
  target_branch: string;
  source_branch: string;
  source_project_id: number;
  author_id: number;
  assignee_id: number;
  title: string;
  created_at: string;
  updated_at: string;
  milestone_id: number;
  state: string;
  merge_status: string;
  target_project_id: number;
  iid: number;
  description: string;
  position: number;
  labels: Label[];
  url: string;
  source: Project;
  target: Project;
  last_commit: LastCommit;
  work_in_progress: boolean;
  assignee: User;
  detailed_merge_status: string;
}

export interface MergeRequestAttributes {
  assignee_id: number;
  author_id: number;
  created_at: string;
  description: string;
  head_pipeline_id: number;
  id: number;
  iid: number;
  last_edited_at: string | null;
  last_edited_by_id: number | null;
  merge_commit_sha: string | null;
  merge_error: unknown;
  merge_params: {
    force_remove_source_branch: "0" | "1";
  };
  merge_status: string;
  merge_user_id: number | null;
  merge_when_pipeline_succeeds: boolean;
  milestone_id: number | null;
  source_branch: string;
  source_project_id: number;
  state_id: number;
  target_branch: string;
  target_project_id: number;
  time_estimate: number;
  title: string;
  updated_at: string;
  updated_by_id: number;
  url: string;
  source: Project;
  target: Project;
  last_commit: LastCommit;
  work_in_progress: boolean;
  total_time_spent: number;
  time_change: number;
  human_total_time_spent: number | null;
  human_time_change: number | null;
  human_time_estimate: number | null;
  assignee_ids: number[];
  labels: Label[];
  state: string | null;
  blocking_discussions_resolved: boolean;
  action: string | null;
}

export interface NoteAttributes {
  id: number;
  note: string;
  noteable_type: string;
  author_id: number;
  created_at: string;
  updated_at: string;
  project_id: number;
  attachment: unknown;
  line_code: string | null;
  commit_id: string;
  noteable_id: number;
  system: boolean;
  st_diff: StDiff | null;
  url: string;
}

export interface PipelineAttributes {
  id: number;
  iid: number;
  ref: string;
  tag: boolean;
  sha: string;
  before_sha: string;
  source: string;
  status: string;
  stages: string[];
  created_at: string;
  finished_at: string;
  duration: number;
  variables: unknown[];
}

export interface Project {
  id: number;
  name: string;
  description: string;
  web_url: string;
  avatar_url: string | null;
  git_ssh_url: string;
  git_http_url: string;
  namespace: string;
  visibility_level: number;
  path_with_namespace: string;
  default_branch: string;
  ci_config_path: string | null;
  homepage: string;
  url: string;
  ssh_url: string;
  http_url: string;
}

export interface Repository {
  name: string;
  url: string;
  description: string;
  homepage: string;
}

export interface Snippet {
  id: number;
  title: string;
  content: string;
  author_id: number;
  project_id: number;
  created_at: string;
  updated_at: string;
  file_name: string;
  expires_at: string | null;
  type: string;
  visibility_level: number;
}

export interface StDiff {
  diff: string;
  new_path: string;
  old_path: string;
  a_mode: string;
  b_mode: string;
  new_file: boolean;
  renamed_file: boolean;
  deleted_file: boolean;
}

export interface User {
  id: number;
  name: string;
  username: string;
  avatar_url: string;
  email: string;
}

export interface Wiki {
  web_url: string;
  git_ssh_url: string;
  git_http_url: string;
  path_with_namespace: string;
  default_branch: string;
}

export interface WikiPageAttributes {
  title: string;
  content: string;
  format: string;
  message: string;
  slug: string;
  url: string;
  action: string;
}

// --------------------------------------------------
// Events
// --------------------------------------------------

export interface BuildEvent {
  object_kind: "build";
  ref: string;
  tag: boolean;
  before_sha: string;
  sha: string;
  build_id: number;
  build_name: string;
  build_stage: string;
  build_status: string;
  build_started_at: string;
  build_finished_at: string;
  build_duration: unknown;
  build_allow_failure: boolean;
  retries_count: number;
  pipelin_id: number;
  project_id: number;
  project_name: string;
  user: User;
  commit: {
    id: string;
    sha: string;
    message: string;
    author_name: string;
    author_email: string;
    status: string;
    duration: unknown;
    started_at: string | null;
    finished_at: string | null;
  };
  repository: Repository;
  runner: unknown;
  environment: unknown;
}

export interface IssueEvent {
  object_kind: "issue";
  event_type: "issue";
  user: User;
  project: Project;
  object_attributes: IssueAttributes;
  repository: Repository;
  assignees: User[];
  assignee: User;
  labels: Label[];
  changes: Changes;
}

export interface MergeRequestEvent {
  object_kind: "merge_request";
  event_type: "merge_request";
  user: User;
  project: Project;
  object_attributes: MergeRequestAttributes;
  labels: Label[];
  changes: unknown;
  repository: Repository;
  assignees: User[];
}

export interface NoteEvent {
  object_kind: "note";
  event_type: "note";
  user: User;
  project_id: number;
  project: Project;
  repository: Repository;
  object_attributes: NoteAttributes;
  commit?: Commit;
  merge_request?: MergeRequest;
  issue?: Issue;
  snippet?: Snippet;
}

export interface PipelineEvent {
  object_kind: "pipeline";
  object_attributes: PipelineAttributes;
  merge_request: unknown;
  user: User;
  project: Project;
  commit: Commit;
  source_pipeline: unknown;
  builds: Build[];
}

export interface PushEvent {
  object_kind: "push";
  event_name: "push";
  before: string;
  after: string;
  ref: string;
  checkout_sha: string;
  user_id: number;
  user_name: string;
  user_username: string;
  user_email: string;
  user_avatar: string;
  project_id: number;
  project: Project;
  repository: Repository;
  commits: Commit[];
  total_commits_count: number;
}

export interface TagPushEvent {
  object_kind: "tag_push";
  event_name: "tag_push";
  before: string;
  after: string;
  ref: string;
  checkout_sha: string;
  user_id: number;
  user_name: string;
  user_avatar: string;
  project_id: number;
  project: Project;
  repository: Repository;
  commits: Commit[];
  total_commits_count: number;
}

export interface WikiPageEvent {
  object_kind: "wiki_page";
  user: User;
  project: Project;
  wiki: Wiki;
  object_attributes: WikiPageAttributes;
}

export type WebhookEvent =
  | BuildEvent
  | IssueEvent
  | MergeRequestEvent
  | NoteEvent
  | PipelineEvent
  | PushEvent
  | TagPushEvent
  | WikiPageEvent;
