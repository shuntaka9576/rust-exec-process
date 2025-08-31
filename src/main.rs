use command_group::AsyncCommandGroup;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Claude Code test...");

    // Claude Code 起動コマンド（vibe-kanban と同じ）
    let command = "npx -y @anthropic-ai/claude-code@latest -p --dangerously-skip-permissions --verbose --output-format=stream-json";

    // テスト用プロンプト
    let prompt = "What is 2 + 2?";

    println!("Executing command: {}", command);
    println!("Current dir: {:?}", std::env::current_dir()?);

    // コマンド実行（vibe-kanban と全く同じ設定）
    let mut command_builder = Command::new("sh");
    command_builder
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // vibe-kanban と同じ group_spawn() を使用（66行目）
    let mut child = command_builder.group_spawn()?;

    println!("Claude process spawned, PID: {:?}", child.inner().id());

    // プロンプト送信（vibe-kanban の70-81行目と同じ処理）
    if let Some(mut stdin) = child.inner().stdin.take() {
        println!("Sending prompt: {}", prompt);
        stdin.write_all(prompt.as_bytes()).await?;
        stdin.shutdown().await?;
        println!("Prompt sent and stdin closed");
    } else {
        eprintln!("Failed to get stdin for Claude process");
        return Err("Failed to get stdin".into());
    }

    println!("Waiting for Claude to complete...");

    // 出力を取得
    let mut stdout_buf = Vec::new();
    let mut stderr_buf = Vec::new();

    if let Some(mut stdout) = child.inner().stdout.take() {
        stdout.read_to_end(&mut stdout_buf).await?;
    }

    if let Some(mut stderr) = child.inner().stderr.take() {
        stderr.read_to_end(&mut stderr_buf).await?;
    }

    // プロセス完了を待つ
    let status = child.wait().await?;

    println!("=== RESULTS ===");
    println!("Exit status: {}", status);

    // 出力を表示
    if !stdout_buf.is_empty() {
        println!("\n=== STDOUT ===");
        println!("{}", String::from_utf8_lossy(&stdout_buf));
    }

    if !stderr_buf.is_empty() {
        println!("\n=== STDERR ===");
        println!("{}", String::from_utf8_lossy(&stderr_buf));
    }

    Ok(())
}
