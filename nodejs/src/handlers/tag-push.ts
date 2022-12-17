import { TagPushEvent } from "./events";
import { SlackMessage } from "./handler";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export function handleTagPushEvent(event: TagPushEvent): SlackMessage {
  return {
    ignored: true,
  };
}
