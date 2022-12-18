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

variable "slack_incoming_webhook_path" {
  type        = string
  description = "Slack incoming webhook path"
}

variable "lambda_language" {
  type        = string
  description = "lambda language: nodejs, rust"
  default     = "nodejs"
}

variable "nodejs_source_dir" {
  type        = string
  description = "node.js lambda source directory"
  default     = "../../nodejs/build/src"
}

variable "nodejs_output_path" {
  type        = string
  description = "node.js lambda distribution zip file path"
  default     = "../../nodejs/function.zip"
}

variable "nodejs_function_name" {
  type        = string
  description = "node.js lambda function name"
  default     = "gitlab-slack-notifier-nodejs"
}

variable "nodejs_handler_name" {
  type        = string
  description = "node.js lambda handler name"
  default     = "app.lambdaHandler"
}

variable "nodejs_runtime" {
  type        = string
  description = "node.js lambda runtime"
  default     = "nodejs18.x"
}

variable "nodejs_architectures" {
  type        = list(string)
  description = "Instruction set architectures for your node.js lambda function"
  default     = ["arm64"]
}

variable "rust_zip_filename" {
  type        = string
  description = "rust lambda package file"
  default     = "../../rust/target/lambda/gitlab-slack-notifier/bootstrap.zip"
}

variable "rust_function_name" {
  type        = string
  description = "rust lambda function name"
  default     = "gitlab-slack-notifier-rust"
}

variable "rust_handler_name" {
  type        = string
  description = "rust lambda handler name"
  default     = "bootstrap"
}

variable "rust_runtime" {
  type        = string
  description = "rust lambda runtime"
  default     = "provided.al2"
}

variable "rust_architectures" {
  type        = list(string)
  description = "Instruction set architectures for your rust lambda function"
  default     = ["arm64"]
}

