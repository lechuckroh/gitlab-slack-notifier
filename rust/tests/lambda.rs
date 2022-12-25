use gitlab_slack_notifier;

#[test]
fn test_my_lambda_handler() {
    let input = include_str!("../events/mr-open.json");

    let request = lambda_http::request::from_str(input)
        .expect("failed to create request");

    let response = my_lambda_handler(request).await.expect("failed to handle request");
}
