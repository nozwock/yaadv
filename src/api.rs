use crate::inputs::AdvInput;
use std::time::Duration;
use ureq::{AgentBuilder, Response};

/// Fetches AOC inputs synchronously
pub fn fetch_inputs(
    inputs: &Vec<AdvInput>,
    session_token: &str,
) -> Vec<Result<Response, ureq::Error>> {
    let mut out = vec![];
    let agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let session_token = format!("session={}", session_token);

    for input in inputs {
        let body = agent
            .get(&input.request_url())
            .set("Cookie", &session_token)
            .call();
        out.push(body);
    }

    out
}
