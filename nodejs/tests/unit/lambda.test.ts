import { APIGatewayProxyEvent, APIGatewayProxyResult } from "aws-lambda";
import * as fs from "fs";

import { lambdaHandler } from "../../src/app";
import { config } from "../../src/config";

function createLambdaEvent(body: string): APIGatewayProxyEvent {
  return {
    httpMethod: "get",
    body: body,
    headers: {},
    isBase64Encoded: false,
    multiValueHeaders: {},
    multiValueQueryStringParameters: {},
    path: "/",
    pathParameters: {},
    queryStringParameters: {},
    requestContext: {
      accountId: "123456789012",
      apiId: "1234",
      authorizer: {},
      httpMethod: "get",
      identity: {
        accessKey: "",
        accountId: "",
        apiKey: "",
        apiKeyId: "",
        caller: "",
        clientCert: {
          clientCertPem: "",
          issuerDN: "",
          serialNumber: "",
          subjectDN: "",
          validity: { notAfter: "", notBefore: "" },
        },
        cognitoAuthenticationProvider: "",
        cognitoAuthenticationType: "",
        cognitoIdentityId: "",
        cognitoIdentityPoolId: "",
        principalOrgId: "",
        sourceIp: "",
        user: "",
        userAgent: "",
        userArn: "",
      },
      path: "/",
      protocol: "HTTP/1.1",
      requestId: "c6af9ac6-7b61-11e6-9a41-93e8deadbeef",
      requestTimeEpoch: 1428582896000,
      resourceId: "123456",
      resourcePath: "/",
      stage: "dev",
    },
    resource: "",
    stageVariables: {},
  };
}

beforeAll(() => {
  config.mention = {
    mergeRequest: {
      onComment: false,
      onAssigned: false,
      onApproved: false,
      onUnapproved: false,
      onMerged: false,
      onConflict: false,
    },
    pipeline: {
      onFailed: false,
    },
  };
  config.groups = [
    {
      gitlab: {
        id: 100,
        name: "dev",
      },
      slack: {
        id: "A01234567",
        name: "dev",
      },
    },
  ];
  config.users = [
    {
      gitlab: {
        id: 1,
        name: "admin",
      },
      slack: {
        id: "U0000000001",
        name: "Admin",
      },
    },
    {
      gitlab: {
        id: 6,
        name: "user1",
      },
      slack: {
        id: "U0123456789",
        name: "User1",
      },
    },
    {
      gitlab: {
        id: 51,
        name: "user2",
      },
      slack: {
        id: "U1123456789",
        name: "User2",
      },
    },
  ];
});

describe("MergeRequest event", function () {
  it("verifies merge request open", async () => {
    const payload = JSON.parse(
      fs.readFileSync("./events/mr-open.json", "utf8")
    );
    const event = createLambdaEvent(JSON.stringify(payload.body));
    const result: APIGatewayProxyResult = await lambdaHandler(event);

    expect(result.statusCode).toEqual(200);
    expect(JSON.parse(result.body)).toEqual({
      status: "sent",
      message: {
        type: "mrkdwn",
        text: ":blush: Admin opend <http://example.com/diaspora/merge_requests/1|Gitlab Test MR !1> *MS-Viewport*[`API`] and assigned to User1.\n`ms-viewport` → `master`",
      },
    });
  });

  it("verifies merge request approved", async () => {
    const payload = JSON.parse(
      fs.readFileSync("./events/mr-approve.json", "utf8")
    );
    const event = createLambdaEvent(JSON.stringify(payload.body));
    const result: APIGatewayProxyResult = await lambdaHandler(event);

    expect(result.statusCode).toEqual(200);
    expect(JSON.parse(result.body)).toEqual({
      status: "sent",
      message: {
        type: "mrkdwn",
        text: ":white_check_mark: Admin approved User2's <https://example.com/diaspora/merge_requests/1|Gitlab Test MR !1> *MS-Viewport*.",
      },
    });
  });

  it("verifies merge request merged", async () => {
    const payload = JSON.parse(
      fs.readFileSync("./events/mr-merge.json", "utf8")
    );
    const event = createLambdaEvent(JSON.stringify(payload.body));
    const result: APIGatewayProxyResult = await lambdaHandler(event);

    expect(result.statusCode).toEqual(200);
    expect(JSON.parse(result.body)).toEqual({
      status: "sent",
      message: {
        type: "mrkdwn",
        text: ":tada: Admin merged User2's <http://example.com/diaspora/merge_requests/1|Gitlab Test MR !1> *MS-Viewport*.",
      },
    });
  });
});

describe("Note event", function () {
  it("verifies comment on merge request with labels and conflict", async () => {
    const payload = JSON.parse(
      fs.readFileSync("./events/note-mr-conflict.json", "utf8")
    );
    const event = createLambdaEvent(JSON.stringify(payload.body));
    const result: APIGatewayProxyResult = await lambdaHandler(event);

    expect(result.statusCode).toEqual(200);
    expect(JSON.parse(result.body)).toEqual({
      status: "sent",
      message: {
        type: "mrkdwn",
        text: ":speech_balloon: [:boom:*Merge conflict*:boom:] Admin <http://example.com/gitlab-org/gitlab-test/merge_requests/1#note_1244|commented> on User2's <undefined|Gitlab Test MR !1> *Tempora et eos debitis quae laborum et.*[`Afterpod`,`Element`].\n> This MR needs work.",
      },
    });
  });

  it("ignores sonarqube code analysis", async () => {
    const payload = JSON.parse(
      fs.readFileSync("./events/note-mr-sonarqube-can-be-merged.json", "utf8")
    );
    const event = createLambdaEvent(JSON.stringify(payload.body));
    const result: APIGatewayProxyResult = await lambdaHandler(event);

    expect(result.statusCode).toEqual(200);
    expect(JSON.parse(result.body)).toEqual({
      status: "ignored",
    });
  });

  it("verifies sonarqybe code analysis with conflict", async () => {
    const payload = JSON.parse(
      fs.readFileSync("./events/note-mr-sonarqube-conflict.json", "utf8")
    );
    const event = createLambdaEvent(JSON.stringify(payload.body));
    const result: APIGatewayProxyResult = await lambdaHandler(event);

    expect(result.statusCode).toEqual(200);
    expect(JSON.parse(result.body)).toEqual({
      status: "sent",
      message: {
        type: "mrkdwn",
        text: ":speech_balloon: :boom:*Merge conflict*:boom: on User1's <undefined|Gitlab Test MR !1> *Tempora et eos debitis quae laborum et.*[`Afterpod`,`Element`].",
      },
    });
  });
});

describe("Pipeline event", function () {
  it("verifies failed pipeline", async () => {
    const payload = JSON.parse(
      fs.readFileSync("./events/pipeline-failed.json", "utf8")
    );
    const event = createLambdaEvent(JSON.stringify(payload.body));
    const result: APIGatewayProxyResult = await lambdaHandler(event);

    expect(result.statusCode).toEqual(200);
    expect(JSON.parse(result.body)).toEqual({
      status: "sent",
      message: {
        type: "mrkdwn",
        text:
          ":fire: Admin Build pipeline failed on <http://192.168.64.1:3005/gitlab-org/gitlab-test|Gitlab Test project> `master`.\n" +
          "- `User<user@gitlab.com>` *test*",
      },
    });
  });
});
