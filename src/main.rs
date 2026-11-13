mod mcp;
mod write_gate;

use std::env;
use std::path::Path;
use std::process::ExitCode;
use weavatrix_rust::Weavatrix;
use weavatrix_rust_refactor::operations as refactor;
use write_gate::WriteGate;

fn main() -> ExitCode {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    match run(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("weavatrix-refactor: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: &[String]) -> Result<(), String> {
    match arguments.first().map(String::as_str) {
        Some("--version") => {
            println!(
                "weavatrix-refactor {} (engine {}, refactor {}, contract v{})",
                env!("CARGO_PKG_VERSION"),
                weavatrix_rust::VERSION,
                weavatrix_rust_refactor::VERSION,
                weavatrix_rust_refactor::CONTRACT_VERSION
            );
            Ok(())
        }
        Some("--help" | "-h") => {
            print_help();
            Ok(())
        }
        Some("mcp") => serve_mcp(arguments),
        Some("list-tools") => list_tools(arguments),
        Some("tool") => call_tool(arguments),
        _ => {
            print_help();
            Err("expected the `mcp`, `tool`, or `list-tools` command".into())
        }
    }
}

fn serve_mcp(arguments: &[String]) -> Result<(), String> {
    let mut repository = ".";
    let mut surface = mcp::ToolSurface::default();
    let mut allow_retarget = false;
    for argument in arguments.iter().skip(1) {
        if let Some(profile) = argument.strip_prefix("--profile=") {
            surface = mcp::ToolSurface::from_profile(profile)
                .ok_or_else(|| format!("unknown MCP profile: {profile}"))?;
            continue;
        }
        if argument == "--allow-retarget" {
            allow_retarget = true;
            continue;
        }
        if argument.starts_with('-') {
            return Err(format!("unknown MCP option: {argument}"));
        }
        repository = argument;
    }
    mcp::serve_with_surface(
        repository,
        WriteGate::from_environment(),
        surface,
        allow_retarget,
    )
    .map_err(|error| error.to_string())
}

fn list_tools(arguments: &[String]) -> Result<(), String> {
    let mut surface = mcp::ToolSurface::default();
    for argument in arguments.iter().skip(1) {
        let Some(profile) = argument.strip_prefix("--profile=") else {
            return Err(format!("unknown list-tools option: {argument}"));
        };
        surface = mcp::ToolSurface::from_profile(profile)
            .ok_or_else(|| format!("unknown tool profile: {profile}"))?;
    }
    let catalog = mcp::catalog_for_surface(surface).map_err(|error| error.to_string())?;
    println!(
        "{}",
        blazingly_json::to_string_pretty(&catalog).map_err(|error| error.to_string())?
    );
    Ok(())
}

/// Calls one tool from the command line.
///
/// The write gate applies here exactly as it does over MCP: a refactoring tool that would write
/// refuses unless the environment opened it. A CLI is not a way around a safety boundary.
fn call_tool(arguments: &[String]) -> Result<(), String> {
    let name = arguments
        .get(1)
        .ok_or_else(|| "tool requires an operation name".to_owned())?;
    let repository = arguments.get(2).map_or(".", String::as_str);
    let input = arguments
        .get(3)
        .map_or_else(
            || Ok(blazingly_json::json!({})),
            |value| blazingly_json::from_str(value),
        )
        .map_err(|error| format!("invalid operation JSON: {error}"))?;
    if refactor::Operation::from_name(name).is_none() {
        return Err(format!(
            "tool `{name}` is not part of the Weavatrix Refactor extension"
        ));
    }
    // The gate is passed into the session rather than checked beside it. Checking here and
    // then handing the session a different answer is how a caller gets WRITE_GATE_CLOSED
    // with the gate open. The lock is taken before the graph is built, so this process cannot
    // read a file another process is writing. A busy repository is a tool result.
    let gate = WriteGate::from_environment();
    let _guard =
        match refactor::repository_guard(Path::new(repository), name, &input, gate.is_open()) {
            Ok(guard) => guard,
            Err(refusal) => {
                println!(
                    "{}",
                    blazingly_json::to_string_pretty(&refusal).map_err(|error| error.to_string())?
                );
                return Ok(());
            }
        };
    let engine = Weavatrix::open(repository).map_err(|error| error.to_string())?;
    let output =
        refactor::RefactorSession::new(gate.is_open()).call(engine.state(), name, &input)?;
    println!(
        "{}",
        blazingly_json::to_string_pretty(&output).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn print_help() {
    println!(
        "Weavatrix Refactor: transactional refactoring backed by an internal repository graph\n\n\
Usage:\n  weavatrix-refactor mcp [REPOSITORY] [--profile=refactor|rename] [--allow-retarget]\n\
  weavatrix-refactor list-tools [--profile=refactor|rename]\n\
  weavatrix-refactor tool NAME [REPOSITORY] ['{{\"argument\":\"value\"}}']\n\
  weavatrix-refactor --version\n\n\
The MCP process pins REPOSITORY and serves many calls at once. Pass repository \
on a call to use a child directory; each repository keeps its own graph, confirm \
token, and rollback. --allow-retarget also accepts a repository outside that pin.\n\n\
Source edits additionally require {}=1 and a plan-bound single-use token. A repository \
another process is using returns REPO_BUSY and does not spend that token.",
        WriteGate::VARIABLE
    );
}
