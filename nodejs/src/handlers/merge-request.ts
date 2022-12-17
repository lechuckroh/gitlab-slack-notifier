import { config } from "../config";
import { MergeRequestEvent } from "./events";
import { SlackMessage } from "./handler";
import { getLabelsMarkdown } from "./slack";
import { getSlackUserName } from "./users";

function getSlackMessage(
  event: MergeRequestEvent
): Record<string, unknown> | null {
  const mr = event.object_attributes;
  const action = mr.action || mr.state;
  const actionUser = event.user;
  const actionUserId = actionUser.id;
  const actionUserName = getSlackUserName(actionUserId, {
    defaultName: actionUser.name,
  });
  const projectName = event.project.name;

  const {
    author_id: authorId,
    assignee_id: assigneeId,
    source_branch: sourceBranch,
    target_branch: targetBranch,
    title,
  } = mr;

  const mrLink = `<${mr.url}|${projectName} MR !${mr.iid}>`;
  const showAssignee = assigneeId && actionUserId !== assigneeId;
  const showAuthor = actionUserId !== authorId;

  switch (action) {
    case "open":
    case "opened": {
      const labelsMarkdown = getLabelsMarkdown(mr.labels);
      if (showAssignee) {
        const mention = config.mention.mergeRequest.onAssigned;
        const assignee = getSlackUserName(assigneeId, {
          mention,
          defaultName: event.assignees[0]?.name,
        });
        return {
          type: "mrkdwn",
          text: `:blush: ${actionUserName} opend ${mrLink} *${title}*${labelsMarkdown} and assigned to ${assignee}.\n\`${sourceBranch}\` → \`${targetBranch}\``,
        };
      } else {
        return {
          type: "mrkdwn",
          text: `:blush: ${actionUserName} opened ${mrLink} *${title}*${labelsMarkdown}.\n\`${sourceBranch}\` → \`${targetBranch}\``,
        };
      }
    }
    case "reopen":
    case "reopened": {
      const labelsMarkdown = getLabelsMarkdown(mr.labels);
      if (showAssignee) {
        const mention = config.mention.mergeRequest.onAssigned;
        const assignee = getSlackUserName(assigneeId, {
          mention,
          defaultName: event.assignees[0]?.name,
        });
        return {
          type: "mrkdwn",
          text: `:smirk: ${actionUserName} reopened ${mrLink} *${title}*${labelsMarkdown} and assigned to ${assignee}.\n\`${sourceBranch}\` → \`${targetBranch}\``,
        };
      } else {
        return {
          type: "mrkdwn",
          text: `:smirk: ${actionUserName} reopened ${mrLink} *${title}*${labelsMarkdown}.\n\`${sourceBranch}\` → \`${targetBranch}\``,
        };
      }
    }
    case "close":
    case "closed":
      return {
        type: "mrkdwn",
        text: `:no_entry: ${actionUserName} closed ${mrLink} *${title}*.`,
      };
    case "merge":
    case "merged": {
      if (showAuthor) {
        const mention = config.mention.mergeRequest.onMerged;
        const authorName = getSlackUserName(authorId, { mention });
        return {
          type: "mrkdwn",
          text: `:tada: ${actionUserName} merged ${authorName}'s ${mrLink} *${title}*.`,
        };
      } else {
        return {
          type: "mrkdwn",
          text: `:tada: ${actionUserName} merged ${mrLink} *${title}*.`,
        };
      }
    }
    case "approve":
    case "approved": {
      if (showAuthor) {
        const mention = config.mention.mergeRequest.onApproved;
        const authorName = getSlackUserName(authorId, { mention });
        return {
          type: "mrkdwn",
          text: `:white_check_mark: ${actionUserName} approved ${authorName}'s ${mrLink} *${title}*.`,
        };
      } else {
        return {
          type: "mrkdwn",
          text: `:white_check_mark: ${actionUserName} approved ${mrLink} *${title}*.`,
        };
      }
    }
    case "unapprove":
    case "unapproved": {
      if (showAuthor) {
        const mention = config.mention.mergeRequest.onUnapproved;
        const authorName = getSlackUserName(authorId, { mention });
        return {
          type: "mrkdwn",
          text: `${actionUserName} unapproved ${authorName}'s ${mrLink} $*{title}*.`,
        };
      } else {
        return {
          type: "mrkdwn",
          text: `${actionUserName} unapproved ${mrLink} ${title}.`,
        };
      }
    }
    case "update": {
      return null;
    }
  }
  throw new Error(`unhandled action: ${action}`);
}

export function handleMergeRequestEvent(
  event: MergeRequestEvent
): SlackMessage {
  const message = getSlackMessage(event);
  return message ? { message } : { ignored: true };
}
