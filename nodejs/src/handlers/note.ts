import { config } from "../config";
import { NoteEvent } from "./events";
import { SlackMessage } from "./handler";
import { getLabelsMarkdown, toSlackMarkdown } from "./slack";
import { getSlackUserName } from "./users";

function isConflict(mergeStatus: string): boolean {
  return mergeStatus === "cannot_be_merged";
}

function quoteNoteText(event: NoteEvent): string {
  const note = event.object_attributes;
  return toSlackMarkdown(note.note, event.project.homepage)
    .split("\n")
    .map((line) => "> " + line)
    .join("\n");
}

function getSlackMessage(event: NoteEvent): Record<string, unknown> | null {
  const actionUser = event.user;
  const actionUserId = actionUser.id;
  const actionUserName = getSlackUserName(actionUserId, {
    defaultName: actionUser.name,
  });
  const projectName = event.project.name;
  const note = event.object_attributes;
  const ignore = ignoreEvent(event);

  // comment on MergeRequest
  if (event.merge_request) {
    const {
      author_id: authorId,
      iid,
      labels,
      merge_status,
      title,
      url,
    } = event.merge_request;
    const mrLink = `<${url}|${projectName} MR !${iid}>`;
    const emoji = ":speech_balloon:";
    const conflict = isConflict(merge_status);
    const labelsMarkdown = getLabelsMarkdown(labels);

    if (ignore) {
      if (conflict) {
        const authorName = getSlackUserName(authorId, {
          mention: config.mention.mergeRequest.onConflict,
        });
        return {
          type: "mrkdwn",
          text: `${emoji} :boom:*Merge conflict*:boom: on ${authorName}'s ${mrLink} *${title}*${labelsMarkdown}.`,
        };
      }
      return null;
    }

    const showAuthor = actionUserId != authorId;
    if (conflict) {
      const mention = config.mention.mergeRequest.onConflict;
      const authorName = getSlackUserName(authorId, { mention });
      return {
        type: "mrkdwn",
        text: `${emoji} [:boom:*Merge conflict*:boom:] ${actionUserName} <${
          note.url
        }|commented> on ${authorName}'s ${mrLink} *${title}*${labelsMarkdown}.\n${quoteNoteText(
          event
        )}`,
      };
    } else {
      if (showAuthor) {
        const mention = config.mention.mergeRequest.onComment;
        const authorName = getSlackUserName(authorId, { mention });
        return {
          type: "mrkdwn",
          text: `${emoji} ${actionUserName} <${
            note.url
          }|commented> on ${authorName}'s ${mrLink} *${title}*${labelsMarkdown}.\n${quoteNoteText(
            event
          )}`,
        };
      } else {
        return {
          type: "mrkdwn",
          text: `${emoji} ${actionUserName} <${
            note.url
          }|commented> on ${mrLink} *${title}*${labelsMarkdown}.\n${quoteNoteText(
            event
          )}`,
        };
      }
    }
  }

  if (ignore) {
    return null;
  }

  if (event.commit) {
    const { url } = event.commit;
    return {
      type: "mrkdwn",
      text: `:speech_balloon: ${actionUserName} <${
        note.url
      }|commented> on ${projectName} <${url}|\`${
        note.commit_id
      }\` commit>.\n${quoteNoteText(event)}`,
    };
  }

  if (event.issue) {
    const { title } = event.issue;
    return {
      type: "mrkdwn",
      text: `:speech_balloon: ${actionUserName} <${
        note.url
      }|commented> on ${projectName} issue *${title}.\n${quoteNoteText(event)}`,
    };
  }

  if (event.snippet) {
    const { title } = event.snippet;
    return {
      type: "mrkdwn",
      text: `:speech_balloon: ${actionUserName} <${
        note.url
      }|commented> on ${projectName} snippet *${title}*.\n${quoteNoteText(
        event
      )}`,
    };
  }

  throw new Error(`unhandled comment`);
}

function ignoreEvent(event: NoteEvent): boolean {
  const note = event.object_attributes.note;

  // ignore sonarqube code analysis
  if (note.startsWith("## SonarQube Code Analysis")) {
    return true;
  }

  // add more rules..
  return false;
}

export function handleNoteEvent(event: NoteEvent): SlackMessage {
  const message = getSlackMessage(event);
  return message ? { message } : { ignored: true };
}
