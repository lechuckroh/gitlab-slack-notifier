import { IssueEvent } from "./events";
import { SlackMessage } from "./handler";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export function handleIssueEvent(event: IssueEvent): SlackMessage {
  return {
    ignored: true,
  };
}
