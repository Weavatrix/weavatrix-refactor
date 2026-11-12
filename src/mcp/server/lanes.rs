//! Parallel-lane tests. Kept out of `tests.rs` so neither file crosses the file-length budget.

use super::parallel::ParallelServer;
use crate::mcp::adapters::ToolSurface;
use crate::write_gate::WriteGate;
use mcport::ToolReply;
use std::fs;

fn status_of(reply: &ToolReply) -> String {
    match reply {
        ToolReply::Success { value, .. } => value
            .get("status")
            .and_then(|status| status.as_str())
            .unwrap_or_default()
            .to_owned(),
        _ => "NO_STATUS".to_owned(),
    }
}

#[test]
fn two_repositories_keep_separate_lanes_tokens_and_rollbacks() {
    use std::sync::Arc;

    let launch = std::env::temp_dir().join(format!("wvxr-lanes-{}", std::process::id()));
    let _ = fs::remove_dir_all(&launch);
    let left = launch.join("left");
    let right = launch.join("right");
    rust_file(&left, "pub fn left_only() -> u32 { 1 }\n");
    rust_file(&right, "pub fn right_only() -> u32 { 2 }\n");
    let left_id = symbol_id(&left, "left_only");
    let right_id = symbol_id(&right, "right_only");
    let server = Arc::new(
        ParallelServer::open(&launch, WriteGate::open(), ToolSurface::Refactor, false)
            .expect("parallel server"),
    );
    let left_preview = spawn_preview(&server, "left", left_id.clone(), "left_renamed")
        .join()
        .expect("left lane");
    let right_preview = spawn_preview(&server, "right", right_id.clone(), "right_renamed")
        .join()
        .expect("right lane");
    assert_eq!(status_of(&left_preview), "PLANNED", "{left_preview:?}");
    assert_eq!(status_of(&right_preview), "PLANNED", "{right_preview:?}");
    let left_token = token_of(&left_preview);
    let right_token = token_of(&right_preview);
    let crossed = server.answer(
        "apply_edit_plan",
        blazingly_json::json!({
            "repository": "right",
            "mode": "apply",
            "confirm_token": left_token,
        }),
    );
    assert_eq!(status_of(&crossed), "TOKEN_UNKNOWN", "{crossed:?}");
    assert!(source_has(&right, "fn right_only()"));
    let applied = rename_apply(&server, "left", &left_id, "left_renamed", &left_token);
    assert_eq!(status_of(&applied), "APPLIED", "{applied:?}");
    assert!(source_has(&left, "fn left_renamed()"));
    let rolled = server.answer(
        "rollback_last_apply",
        blazingly_json::json!({"repository": "left"}),
    );
    assert_eq!(status_of(&rolled), "ROLLED_BACK", "{rolled:?}");
    assert!(source_has(&left, "fn left_only()"));
    let right_applied = rename_apply(&server, "right", &right_id, "right_renamed", &right_token);
    assert_eq!(status_of(&right_applied), "APPLIED", "{right_applied:?}");
    assert!(
        source_has(&left, "fn left_only()"),
        "rolling back and editing the other lane must leave left restored"
    );
    let _ = fs::remove_dir_all(&launch);
}

#[test]
fn a_repository_outside_the_launch_root_needs_retarget() {
    let launch = std::env::temp_dir().join(format!("wvxr-pin-{}", std::process::id()));
    let outside = std::env::temp_dir().join(format!("wvxr-out-{}", std::process::id()));
    let _ = fs::remove_dir_all(&launch);
    let _ = fs::remove_dir_all(&outside);
    fs::create_dir_all(launch.join("src")).expect("launch");
    fs::create_dir_all(outside.join("src")).expect("outside");
    fs::write(launch.join("src/lib.rs"), "pub fn inside() -> u32 { 1 }\n").expect("inside");
    fs::write(
        outside.join("src/lib.rs"),
        "pub fn outside() -> u32 { 1 }\n",
    )
    .expect("outside");
    let pinned = ParallelServer::open(&launch, WriteGate::closed(), ToolSurface::Refactor, false)
        .expect("pinned server");
    let refused = pinned.answer(
        "delete_readiness",
        blazingly_json::json!({
            "repository": outside.display().to_string(),
            "symbol": "outside",
        }),
    );
    let ToolReply::Error(reply) = refused else {
        panic!("an outside repository must be refused: {refused:?}");
    };
    assert!(reply.contains("--allow-retarget"), "{reply}");
    let opened = ParallelServer::open(&launch, WriteGate::closed(), ToolSurface::Refactor, true)
        .expect("retarget server");
    let allowed = opened.answer(
        "delete_readiness",
        blazingly_json::json!({
            "repository": outside.display().to_string(),
            "symbol": "outside",
        }),
    );
    assert_ne!(status_of(&allowed), "NO_STATUS", "{allowed:?}");
    assert!(status_of(&allowed) != "WRITE_GATE_CLOSED");
    let _ = fs::remove_dir_all(&launch);
    let _ = fs::remove_dir_all(&outside);
}

fn symbol_id(root: &std::path::Path, prefix: &str) -> String {
    let engine = weavatrix_rust::Weavatrix::open(root).expect("open fixture");
    engine
        .state()
        .graph()
        .nodes()
        .iter()
        .find(|node| node.label.starts_with(prefix))
        .map(|node| node.id.as_str().to_owned())
        .expect("indexed symbol")
}

fn token_of(reply: &ToolReply) -> String {
    match reply {
        ToolReply::Success { value, .. } => value
            .get("confirmToken")
            .and_then(blazingly_json::Value::as_str)
            .expect("confirm token")
            .to_owned(),
        other => panic!("preview did not return a token: {other:?}"),
    }
}

fn rust_file(root: &std::path::Path, source: &str) {
    fs::create_dir_all(root.join("src")).expect("lane");
    fs::write(root.join("src/lib.rs"), source).expect("lane source");
}

fn source_has(root: &std::path::Path, needle: &str) -> bool {
    fs::read_to_string(root.join("src/lib.rs"))
        .expect("source")
        .contains(needle)
}

fn spawn_preview(
    server: &std::sync::Arc<ParallelServer>,
    repository: &str,
    symbol: String,
    new_name: &str,
) -> std::thread::JoinHandle<ToolReply> {
    let server = std::sync::Arc::clone(server);
    let repository = repository.to_owned();
    let new_name = new_name.to_owned();
    std::thread::spawn(move || {
        server.answer(
            "rename_symbol",
            blazingly_json::json!({
                "repository": repository,
                "symbol": symbol,
                "new_name": new_name,
            }),
        )
    })
}

fn rename_apply(
    server: &ParallelServer,
    repository: &str,
    symbol: &str,
    new_name: &str,
    token: &str,
) -> ToolReply {
    server.answer(
        "rename_symbol",
        blazingly_json::json!({
            "repository": repository,
            "symbol": symbol,
            "new_name": new_name,
            "mode": "apply",
            "confirm_token": token,
        }),
    )
}
