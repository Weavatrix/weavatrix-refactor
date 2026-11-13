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

## MCP methods

`rename_symbol` renames one symbol from the graph. An ambiguous name returns
candidate ids. Proven sites are planned; a same-named occurrence the graph does
not prove stays in `uncertainReferences`. Completeness stays `PARTIAL`.
`mode:"apply"` with the preview token writes.

`rename_related_symbols` renames several symbols in one transaction. A collision
returns `CONFLICT`. A chain or a swap is applied together. Completeness stays
`PARTIAL`. Preview, then `mode:"apply"` with the token.

`change_signature` plans an added or removed parameter in every indexed
language. Kotlin is read with the Java tokenizer. A default leaves existing
calls. Without one, proven calls are edited when a value is given. A missed
call is edited only when the name is declared once. The result stays `PARTIAL`.
Apply the plan with `apply_edit_plan`.

`edit_symbol` plans a replacement or insertion inside one declaration. The end
is the matching brace or the indent block. A signature-line end is `PARTIAL`
and warns `DECLARATION_END_UNPROVEN`. Apply the plan with `apply_edit_plan`.

`bulk_replace` plans a literal or regex replacement of indexed code. Comments
and strings are included only with `include_trivia`. An unknown extension still
skips comments, quotes, and a `#` that starts a line. Apply the plan with
`apply_edit_plan`.

`organize_imports` removes a named binding that occurs once: a Rust snake_case
`use`, a Python `from` import, a Go path segment, a Java import, a C# `using`,
a C++ `using` name, or a Kotlin `import`. A trait import, `pub use`, a bare
Python import, a Go blank or dot import, a star import, a C++ `#include`, and a
`using namespace` stay. An unrecognised extension returns `NO_UNUSED_IMPORTS`.

`move_file` plans the filesystem rename and the relative import rewrites. A
missing parent directory is created at apply and removed on rollback when it is
empty. A non-relative specifier whose last segment is the moved file keeps the
plan `PARTIAL`.

`move_symbol` returns a cycle review and an edit plan that moves the
declaration and retargets the imports it can prove. The status stays
`EVALUATED`. Apply the plan with `apply_edit_plan`.

`delete_readiness` reports whether anything still depends on a symbol. It
writes nothing. A private unused symbol can be `safe: true`. The verdict stays
`REVIEW_REQUIRED`.

`delete_closure` plans deletion of a symbol, endpoint, or file and the callees
only that closure uses. `scope:"exclusive"` returns `BLOCKED` when another
contract still uses it. `scope:"connected"` also removes direct referrers. A
repository returns `BLOCKED`. A symbol with no source span returns
`NOT_A_SYMBOL`. Apply the plan with `apply_edit_plan`.

`apply_edit_plan` previews a `weavatrix.edit-plan.v1` envelope and writes it
when `mode` is `"apply"`. Apply needs the write gate and the preview token. A
rename missing `from`, `to`, or `sha256`, or a delete missing `path` or
`sha256`, refuses the whole plan as `INVALID_PLAN`. Another process reading or
writing that repository returns `REPO_BUSY`, and the confirm token stays usable.

`rollback_last_apply` restores the newest retained apply and removes an empty
directory a rename created. It requires the write gate. Another process using
the repository returns `REPO_BUSY`.

> **1.0 is the engine change.** Versions 0.1.x were a JavaScript package. That
> package line continues as
> [`weavatrix-refactor-js`](https://www.npmjs.com/package/weavatrix-refactor-js),
> which picks up the version line at 0.1.6 where `weavatrix-refactor@0.1.5` left
> off. The two keep separate state directories, so they never share a lock, a
> token store, or a rollback journal.

Full documentation:
[github.com/Weavatrix/weavatrix-refactor](https://github.com/Weavatrix/weavatrix-refactor)

MIT.
