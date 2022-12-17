import * as https from "https";

import { config } from "../config";
import { Label } from "./events";
import {
  replaceGitlabGroupBySlackGroup,
  replaceGitlabUserBySlackUser,
} from "./users";

const testEnv = process.env.NODE_ENV === "test";

export interface SlackSendResult {
  response: unknown;
  message: Record<string, unknown>;
}

export function getLabelsMarkdown(labels: Label[]): string {
  if (labels.length === 0) {
    return "";
  }
  const enumTtiles = labels.map((label) => `\`${label.title}\``).join(",");
  return `[${enumTtiles}]`;
}

const REGEX_MARKDOWN_IMAGE_LINK = /!\[([^\]]*)]\(([^)]+)\)/g;
const REGEX_MARKDOWN_LINK = /\[([^\]]*)]\(([^)]+)\)/g;

export function toSlackMarkdown(text: string, projectUrl?: string): string {
  REGEX_MARKDOWN_IMAGE_LINK.lastIndex = 0;
  REGEX_MARKDOWN_LINK.lastIndex = 0;

  let result = text;
  if (projectUrl) {
    result = result.replace(
      REGEX_MARKDOWN_IMAGE_LINK,
      "<" + projectUrl + "$2|$1>"
    );
  }
  result = result.replace(REGEX_MARKDOWN_LINK, "<$2|$1>");
  result = replaceGitlabGroupBySlackGroup(result);
  result = replaceGitlabUserBySlackUser(result);

  return result;
}

export async function sendSlackMessage(
  msgData: Record<string, unknown>
): Promise<SlackSendResult> {
  const payloadText = JSON.stringify(msgData);

  if (testEnv) {
    return {
      response: undefined,
      message: msgData,
    };
  }

  const options = {
    hostname: config.slackIncomingWebhook.hostname,
    port: config.slackIncomingWebhook.port,
    path: config.slackIncomingWebhook.path,
    method: "POST",
    headers: {
      "Content-Type": "application/json; charset=utf-8",
    },
  };

  const p = new Promise((resolve, reject) => {
    const req = https.request(options, (res) => {
      res.setEncoding("utf8");
      let responseBody = "";

      res.on("data", (chunk) => {
        responseBody += chunk;
      });

      res.on("end", () => {
        resolve(responseBody);
      });
    });

    req.on("error", (err) => {
      reject(err);
    });

    req.write(payloadText);
    req.end();
  });
  const response = await p;

  return {
    response,
    message: msgData,
  };
}
