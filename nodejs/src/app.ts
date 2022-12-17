import { APIGatewayProxyEvent, APIGatewayProxyResult } from "aws-lambda";

import { handleEvent } from "./handlers/handler";

function createResponse(
  statusCode: number,
  body: unknown
): APIGatewayProxyResult {
  return {
    statusCode: statusCode,
    body: JSON.stringify(body),
  };
}

/**
 * AWS Lambda Handler
 *
 * Event doc: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-input-format
 * @param {Object} event - API Gateway Lambda Proxy Input Format
 *
 * Return doc: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html
 * @returns {Object} object - API Gateway Lambda Proxy Output Format
 */
export const lambdaHandler = async (
  event: APIGatewayProxyEvent
): Promise<APIGatewayProxyResult> => {
  if (event.body == null) {
    return createResponse(500, { message: "payload 'body' is null" });
  }

  try {
    const body = JSON.parse(event.body);
    const status = await handleEvent(body);

    return createResponse(200, status);
  } catch (e) {
    console.error(e);
    if (e instanceof Error) {
      return createResponse(500, { message: e.message });
    }
    return createResponse(500, { message: JSON.stringify(e) });
  }
};
