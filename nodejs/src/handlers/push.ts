import { PushEvent } from "./events";
import { SlackMessage } from "./handler";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export function handlePushEvent(event: PushEvent): SlackMessage {
  return {
    ignored: true,
  };
}
