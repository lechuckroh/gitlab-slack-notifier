import { config } from "../config";
import { PipelineEvent } from "./events";
import { SlackMessage } from "./handler";
import { getSlackUserName } from "./users";

function getSlackMessage(event: PipelineEvent): Record<string, unknown> | null {
  const pipeline = event.object_attributes;
  const mention = config.mention.pipeline;

  if (pipeline.status === "failed") {
    const { commit, merge_request, project } = event;
    const projectLink = `<${project.web_url}|${project.name} project>`;
    const actionUser =
      merge_request == null
        ? ""
        : getSlackUserName(event.user.id, {
            mention: mention.onFailed,
            defaultName: event.user.name,
          });

    const committer = `${commit.author.name}<${commit.author.email}>`;
    return {
      type: "mrkdwn",
      text: `:fire: ${actionUser} Build pipeline failed on ${projectLink} \`${
        pipeline.ref
      }\`.\n- \`${committer}\` *${commit.title ?? commit.message}*`,
    };
  }
  return null;
}

export function handlePipelineEvent(event: PipelineEvent): SlackMessage {
  const message = getSlackMessage(event);
  return message ? { message } : { ignored: true };
}
