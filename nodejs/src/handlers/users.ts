import { GroupMetaInfo, UserMetaInfo, config } from "../config";

class UserCache {
  private groupsByGitLabGroupName = new Map<string, GroupMetaInfo>();
  private usersByGitLabUserId = new Map<number, UserMetaInfo>();
  private usersByGitLabUserName = new Map<string, UserMetaInfo>();

  constructor(groups: GroupMetaInfo[], users: UserMetaInfo[]) {
    groups.forEach((group) => {
      this.groupsByGitLabGroupName.set(group.gitlab.name, group);
    });
    users.forEach((user) => {
      this.usersByGitLabUserId.set(user.gitlab.id, user);
      this.usersByGitLabUserName.set(user.gitlab.name, user);
    });
  }

  getGitLabGroupNameToSlackGroupIdMap(): Record<string, string> {
    const result: Record<string, string> = {};
    this.groupsByGitLabGroupName.forEach((group, gitlabGroupName) => {
      const slackGroupId = group.slack?.id;
      if (slackGroupId) {
        result[gitlabGroupName] = slackGroupId;
      }
    });
    return result;
  }

  getUserByGitlabUserId(userId: number): UserMetaInfo | undefined {
    return this.usersByGitLabUserId.get(userId);
  }

  getGitLabUserNameToSlackUserIdMap(): Record<string, string> {
    const result: Record<string, string> = {};
    this.usersByGitLabUserName.forEach((user, gitlabUserName) => {
      const slackUserId = user.slack?.id;
      if (slackUserId) {
        result[gitlabUserName] = slackUserId;
      }
    });
    return result;
  }
}

let userCache: UserCache | undefined = undefined;

function getUserCache(): UserCache {
  if (userCache === undefined) {
    userCache = new UserCache(config.groups, config.users);
  }
  return userCache;
}

export function getUserMetaInfo(
  gitlabUserId: number
): UserMetaInfo | undefined {
  return getUserCache().getUserByGitlabUserId(gitlabUserId);
}

export function replaceGitlabGroupBySlackGroup(text: string): string {
  const map = getUserCache().getGitLabGroupNameToSlackGroupIdMap();
  let result = text;
  for (const [key, value] of Object.entries(map)) {
    result = result.replace(`@${key}`, `<!subteam^${value}>`);
  }
  return result;
}

export function replaceGitlabUserBySlackUser(text: string): string {
  const map = getUserCache().getGitLabUserNameToSlackUserIdMap();
  let result = text;
  for (const [key, value] of Object.entries(map)) {
    result = result.replace(`@${key}`, `<@${value}>`);
  }
  return result;
}

export function getSlackUserName(
  gitlabUserId: number,
  opt?: {
    mention?: boolean;
    defaultName?: string;
  }
): string {
  const user = getUserMetaInfo(gitlabUserId);

  // slack user is not defined
  if (user === undefined) {
    return opt?.defaultName ?? `user#${gitlabUserId}`;
  }

  // slack user is not mapped
  if (user.slack == null) {
    return user.gitlab.name;
  }

  // slack
  const slackUser = user.slack;
  if (opt?.mention) {
    return `<@${slackUser.id}>`;
  }

  return slackUser.name ?? user.gitlab.name;
}
