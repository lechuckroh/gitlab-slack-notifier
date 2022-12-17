output "qualified_arn" {
  description = "lambda function qualified ARN"
  value       = aws_lambda_function.fn.qualified_arn
}

output "function_url" {
  description = "HTTP URL endpoint for the function"
  value       = aws_lambda_function_url.webhook_latest.function_url
}