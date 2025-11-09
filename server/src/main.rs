use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(e) = server::start().await {
        tracing::error!("CRITICAL SERVER ERROR: {}", e);
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}
