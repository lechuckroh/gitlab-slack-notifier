export interface GroupMetaInfo {
  gitlab: {
    id: number;
    name: string;
  };
  slack?: {
    id: string;
    name?: string;
  };
}

export interface UserMetaInfo {
  gitlab: {
    id: number;
    name: string;
  };
  slack?: {
    id: string;
    name?: string;
  };
}

export interface Config {
  groups: GroupMetaInfo[];
  users: UserMetaInfo[];
  slackIncomingWebhook: {
    hostname: string;
    port: number;
    path: string;
  };
  mention: {
    mergeRequest: {
      onComment: boolean;
      onAssigned: boolean;
      onApproved: boolean;
      onUnapproved: boolean;
      onMerged: boolean;
      onConflict: boolean;
    };
    pipeline: {
      onFailed: boolean;
    };
  };
}

export const config: Config = {
  groups: [],
  users: [],
  slackIncomingWebhook: {
    hostname: "hooks.slack.com",
    port: 443,
    path:
      process.env.SLACK_INCOMING_WEBHOOK_PATH ||
      "/services/T00000000/XXXXXXXXXXX/xxxxxxxxxxxxxxxxxxxxxxxx",
  },
  mention: {
    mergeRequest: {
      onComment: false,
      onAssigned: false,
      onApproved: false,
      onUnapproved: false,
      onMerged: false,
      onConflict: true,
    },
    pipeline: {
      onFailed: false,
    },
  },
};
