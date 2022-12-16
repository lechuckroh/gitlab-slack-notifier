import { WikiPageEvent } from "./events";
import { SlackMessage } from "./handler";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export function handleWikiPageEvent(event: WikiPageEvent): SlackMessage {
  return {
    ignored: true,
  };
}
