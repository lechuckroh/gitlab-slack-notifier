import { handleBuildEvent } from "./build";
import { WebhookEvent } from "./events";
import { handleIssueEvent } from "./issue";
import { handleMergeRequestEvent } from "./merge-request";
import { handleNoteEvent } from "./note";
import { handlePipelineEvent } from "./pipeline";
import { handlePushEvent } from "./push";
import { sendSlackMessage } from "./slack";
import { handleTagPushEvent } from "./tag-push";
import { handleWikiPageEvent } from "./wiki-page";

export interface HandleEventStatus {
  status: "ignored" | "sent" | "error";
  message?: Record<string, unknown>;
  error?: string;
}

export interface SlackMessage {
  ignored?: boolean;
  message?: Record<string, unknown>;
  error?: string;
}

function buildSlackMessage(event: WebhookEvent): SlackMessage {
  const objectKind = event.object_kind;
  switch (objectKind) {
    case "build":
      return handleBuildEvent(event);
    case "issue":
      return handleIssueEvent(event);
    case "merge_request":
      return handleMergeRequestEvent(event);
    case "note":
      return handleNoteEvent(event);
    case "pipeline":
      return handlePipelineEvent(event);
    case "push":
      return handlePushEvent(event);
    case "tag_push":
      return handleTagPushEvent(event);
    case "wiki_page":
      return handleWikiPageEvent(event);
    default:
      return {
        error: `unhandled object_kind: ${objectKind}`,
      };
  }
}

export async function handleEvent(
  event: WebhookEvent
): Promise<HandleEventStatus> {
  try {
    const msg = buildSlackMessage(event);

    if (msg.ignored) {
      return { status: "ignored" };
    }
    if (msg.error) {
      return { status: "error", error: msg.error };
    }
    if (msg.message) {
      const slackSendResult = await sendSlackMessage(msg.message);
      return {
        status: "sent",
        ...slackSendResult,
      };
    }
    return {
      status: "error",
      error: "unhandled event",
    };
  } catch (e) {
    if (e instanceof Error) {
      return {
        status: "error",
        error: e.message,
      };
    } else {
      return {
        status: "error",
        error: JSON.stringify(e),
      };
    }
  }
}
