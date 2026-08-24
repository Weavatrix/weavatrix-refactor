# weavatrix-refactor

Transactional refactoring MCP extension for coding agents: 11 evidence-backed
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
`--profile=refactor` surface exposes all 11 Refactor methods.

When a separate Weavatrix Core server already supplies the 43 read-only tools,
load only the 11 Refactor extensions:

```bash
npx -y weavatrix-refactor mcp /absolute/path/to/repository --profile=refactor
```

The official Cursor, Codex, Claude Code, and Grok Build plugin bundle uses this
extension-only profile to avoid duplicate MCP definitions.

Core methods cannot be called through this package. Use the separate Weavatrix
product for read-only repository intelligence.

This package ships the native Rust binary. Nothing is compiled at install time,
there are no lifecycle scripts, and no runtime dependencies.

Producing a plan is a read. Applying one requires two enforced gates:
`WEAVATRIX_ALLOW_SOURCE_EDITS=1` and a single-use token bound to that exact plan
and repository. Selecting this write-capable package or a narrower tool profile
does not grant write authority.

> **1.0 is the engine change.** Versions 0.1.x were a JavaScript package. That
> package line continues as
> [`weavatrix-refactor-js`](https://www.npmjs.com/package/weavatrix-refactor-js),
> which picks up the version line at 0.1.6 where `weavatrix-refactor@0.1.5` left
> off. The two keep separate state directories, so they never share a lock, a
> token store, or a rollback journal.

Full documentation:
[github.com/sergii-ziborov/weavatrix-refactor](https://github.com/sergii-ziborov/weavatrix-refactor)

MIT.
