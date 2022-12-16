import { BuildEvent } from "./events";
import { SlackMessage } from "./handler";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export function handleBuildEvent(event: BuildEvent): SlackMessage {
  return {
    ignored: true,
  };
}
