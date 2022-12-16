variable "role_name" {
  type        = string
  description = "lambda exec role name"
  default     = "gitlab-slack-notifier-lambda-exec.role"
}

variable "policy_name" {
  type        = string
  description = "lambda exec role IAM policy"
  default     = "gitlab-slack-notifier-lambda-exec.policy"
}

variable "source_dir" {
  type        = string
  description = "lambda source directory"
  default     = "../../nodejs/build/src"
}

variable "output_path" {
  type        = string
  description = "lambda distribution zip file path"
  default     = "../../nodejs/function.zip"
}

variable "function_name" {
  type        = string
  description = "lambda function name"
  default     = "gitlab-slack-notifier"
}

variable "handler_name" {
  type        = string
  description = "lambda handler name"
  default     = "app.lambdaHandler"
}

variable "runtime" {
  type        = string
  description = "lambda runtime"
  default     = "nodejs18.x"
}

variable "architecture" {
  type        = string
  description = "Instruction set architecture for your Lambda function"
  default     = "arm64"
}

variable "slack_incoming_webhook_path" {
  type        = string
  description = "Slack incoming webhook path"
}
