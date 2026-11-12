# weavatrix-refactor

Transactional refactoring MCP extension for coding agents: 12 evidence-backed
plan/apply tools, hash-bound previews, crash-recoverable writes, and rollback.
It uses the Rust Weavatrix graph internally without exporting Core methods.

```bash
npx -y weavatrix-refactor mcp /absolute/path/to/repository
```

For rename-only sessions, keep the agent context small:

```bash
npx -y weavatrix-refactor mcp /absolute/path/to/repository --profile=rename
```

That profile exposes only `rename_symbol` and `rollback_last_apply`. The default
`--profile=refactor` surface exposes all 12 Refactor methods.

One process serves many calls at once. Pass `repository` to select a child of
the launch root. Each child has its own graph, confirm token, and rollback, and
a token from one child does not apply to another. A path outside the launch
root needs `--allow-retarget`.

When a separate Weavatrix Core server already supplies the 43 read-only tools,
load only the 12 Refactor extensions:

```bash
npx -y weavatrix-refactor mcp /absolute/path/to/repository --profile=refactor
```

The official Cursor, Codex, Claude Code, and Grok Build plugin bundle uses this
extension-only profile to avoid duplicate MCP definitions. Plugin MCP args use
`${workspaceFolder}` (not `.`) so writes stay on the open workspace root.

Core methods cannot be called through this package. Use the separate Weavatrix
product for read-only repository intelligence.

This package ships the native Rust binary. Nothing is compiled at install time,
there are no lifecycle scripts, and no runtime dependencies.

Producing a plan is a read. Applying one requires two enforced gates:
`WEAVATRIX_ALLOW_SOURCE_EDITS=1` and a single-use token bound to that exact plan
and repository. Selecting this write-capable package or a narrower tool profile
does not grant write authority.

`delete_closure` plans a symbol, an endpoint, or a file. A repository returns
`BLOCKED`. A symbol with no source span returns `NOT_A_SYMBOL`. A uniquely
named handler in another file is part of the route. `organize_imports` removes
an unused Rust snake_case `use`, a Python `from` import, a Go path segment, a
Java import, a C# `using`, a C++ `using` name, and a Kotlin `import`. A trait
import, a bare Python import, a Go blank import, a star import, a C++ header,
and a `using namespace` stay. `bulk_replace` matches code, and includes comments
and strings only with `include_trivia`. An unknown extension still skips
comments, quotes, and a leading `#`. `change_signature` works in every indexed
language and stays `PARTIAL`. `move_file` renames the file and rewrites relative
specifiers; a missing parent directory is created at apply and removed on
rollback when it is empty. A rename or delete entry missing its path or hash
refuses the whole plan as `INVALID_PLAN`. Rollback restores the newest retained
apply.
`delete_readiness` can report `safe: true` for a private unused symbol, keeps
the verdict `REVIEW_REQUIRED`, and writes nothing.

> **1.0 is the engine change.** Versions 0.1.x were a JavaScript package. That
> package line continues as
> [`weavatrix-refactor-js`](https://www.npmjs.com/package/weavatrix-refactor-js),
> which picks up the version line at 0.1.6 where `weavatrix-refactor@0.1.5` left
> off. The two keep separate state directories, so they never share a lock, a
> token store, or a rollback journal.

Full documentation:
[github.com/Weavatrix/weavatrix-refactor](https://github.com/Weavatrix/weavatrix-refactor)

MIT.
