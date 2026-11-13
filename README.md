# Weavatrix Refactor

[![CI](https://github.com/Weavatrix/weavatrix-refactor/actions/workflows/ci.yml/badge.svg)](https://github.com/Weavatrix/weavatrix-refactor/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/weavatrix-refactor.svg)](https://www.npmjs.com/package/weavatrix-refactor)
[![crates.io](https://img.shields.io/crates/v/weavatrix-refactor.svg)](https://crates.io/crates/weavatrix-refactor)
[![docs.rs](https://docs.rs/weavatrix-refactor/badge.svg)](https://docs.rs/weavatrix-refactor)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

The explicit source-write layer of the [Weavatrix ecosystem](https://weavatrix.com/ecosystem); Core remains read-only.

**Refactor across files without letting the agent guess.**

Weavatrix Refactor is a native MCP server for coding agents. It finds a symbol through the
repository graph, previews byte-exact edits, reports every same-named occurrence it could not
prove, and writes only after an explicit, hash-bound confirmation.

The useful difference is not “AI can rename a word.” It is that the agent gets evidence before
the write and a recoverable transaction after it.

## Install

Run the native binary through npm:

```bash
npx -y weavatrix-refactor mcp /absolute/path/to/repository
```

Or install it with Cargo:

```bash
cargo install weavatrix-refactor --locked
weavatrix-refactor mcp /absolute/path/to/repository
```

An MCP client can start it directly. Prefer an absolute path, or Cursor's
`${workspaceFolder}` so the write server never follows the host process cwd:

```json
{
  "mcpServers": {
    "weavatrix-refactor": {
      "command": "npx",
      "args": ["-y", "weavatrix-refactor@1.0.15", "mcp", "${workspaceFolder}", "--profile=refactor"],
      "env": {"WEAVATRIX_ALLOW_SOURCE_EDITS": "1"}
    }
  }
}
```

Leave `WEAVATRIX_ALLOW_SOURCE_EDITS` unset for preview-only sessions. Preview is a read; only a
confirmed apply or rollback requires the gate. One process serves many calls at once. A call
passes `repository` to select a child of the launch root; each child has its own graph, confirm
token, and rollback, and a token from one child does not apply to another. Readers of one
repository share a lock. A writer holds that lock alone, so another process receives `REPO_BUSY`
and keeps its confirm token. Refactor does not expose Core's `open_repo`. A path outside the
launch root needs `--allow-retarget`.

### Install as a plugin

The repository also ships one plugin bundle for Cursor, Codex, Claude Code,
and Grok Build. The plugin starts the extension-only profile, so it contributes
the 12 Refactor methods without repeating Weavatrix Core's 43 read-only method
definitions. Install the separate Weavatrix plugin when both surfaces are
needed.

The bundled skill is optional and activates only for explicitly requested
refactoring work. Its entry point stays short; each method has an individual
reference card that is loaded only when needed.

- Cursor: search for **Weavatrix Refactor** after marketplace approval.
- Codex: add the `Weavatrix/weavatrix-refactor` marketplace from
  `.agents/plugins`, then add `weavatrix-refactor@weavatrix-refactor`.
- Claude Code: add the same repository marketplace from `.claude-plugin`, then
  install `weavatrix-refactor@weavatrix-refactor`.
- Grok Build: run `grok plugin marketplace add
  Weavatrix/weavatrix-refactor`, open `/marketplace`, and install it.

The plugin opens `WEAVATRIX_ALLOW_SOURCE_EDITS=1`, but every write still needs
the exact preview's short-lived, plan-bound single-use token.

For a rename task, expose only the two tools the agent needs:

```bash
weavatrix-refactor mcp /absolute/path/to/repository --profile=rename
```

The `rename` profile keeps `rename_symbol` and `rollback_last_apply`; every
unrelated graph and refactor tool is absent and cannot be called. Omit the
option (or use `--profile=refactor`) for all 12 Refactor methods. The narrower
catalog reduces repeated agent context; it does not weaken preview hashes, the
write gate, the confirmation token, or rollback checks.

When Weavatrix Core is already installed as a separate MCP, expose only the 12
methods that Refactor adds:

```bash
weavatrix-refactor mcp /absolute/path/to/repository --profile=refactor
```

The `refactor` profile is the default for MCP and `list-tools`, including the
marketplace plugin. The former `full` compatibility profile has been removed
and is rejected by both commands: Refactor no longer republishes Core methods.
Install Weavatrix Core as its own MCP when read-only repository intelligence is
also needed.

## A safe cross-file rename in three calls

Suppose two files declare `resolveTarget`. A bare name is ambiguous, so the server refuses to
pick one and returns exact ids:

```json
// rename_symbol
{"symbol":"resolveTarget","new_name":"locateTarget","output_format":"json"}

{
  "status": "NOT_FOUND",
  "reason": "the name matches more than one symbol; pass one of the candidate ids",
  "candidates": [
    "symbol:src/core.ts#function:resolveTarget@2:1",
    "symbol:src/shadow.ts#function:resolveTarget@3:1"
  ]
}
```

Retry with the intended id. The result is still read-only: it contains the edit plan, content
hashes, the proven edit count, and the references the backend refused to guess at.

```json
// rename_symbol
{
  "symbol": "symbol:src/core.ts#function:resolveTarget@2:1",
  "new_name": "locateTarget",
  "output_format": "json"
}

{
  "status": "PLANNED",
  "completeness": "PARTIAL",
  "renamedEdits": 7,
  "uncertainReferences": ["... three named locations ..."],
  "confirmToken": "single-use-token",
  "plan": {"schemaVersion":"weavatrix.edit-plan.v1","files":["..."]}
}
```

After reviewing the plan, repeat the same operation with the same inputs and its token:

```json
// rename_symbol
{
  "symbol": "symbol:src/core.ts#function:resolveTarget@2:1",
  "new_name": "locateTarget",
  "mode": "apply",
  "confirm_token": "single-use-token",
  "output_format": "json"
}

{"status":"APPLIED"}
```

This exact flow is exercised against the real stdio MCP server. The repository fixture requires
seven edits across three files, preserves four traps (a longer identifier, string, comment, and
unrelated shadow), and must still compile; it scores **12/12**.

## What is automatic today

The twelve tools do not all claim the same level of automation:

| Level | Tools | What the agent receives |
| --- | --- | --- |
| Complete preview/apply workflow | `rename_symbol`, `rename_related_symbols` | A generated plan and token, then a confirmed write by repeating the same operation |
| Plan-producing | `change_signature`, `edit_symbol`, `bulk_replace`, `organize_imports`, `move_file`, `move_symbol`, `delete_closure` | Byte-exact edits or a review plan; use `apply_edit_plan` where an edit-plan envelope is returned |
| Advisory | `delete_readiness` | A deletion verdict. A private unused symbol can be `safe: true`. The verdict stays `REVIEW_REQUIRED` and nothing is written |
| Safety infrastructure | `apply_edit_plan`, `rollback_last_apply` | Verification/application of an edit-plan envelope and drift-safe restoration |

Refactor uses the Rust Weavatrix graph internally for evidence but never
exports Core methods. Install the separate Weavatrix plugin for read-only
repository intelligence. The Refactor MCP owns exactly its 12 extension
methods; `--profile=rename` is an optional narrower two-method workflow.

## Safety model and exact limits

- Ambiguous names fail with candidate ids instead of selecting the first match.
- `PARTIAL` means the graph proved the edits in the plan but did **not** prove that no other
  semantic references exist. `uncertainReferences` names the places requiring review.
- Every file in a plan carries a content hash. A changed working tree makes the preview stale
  instead of applying the old coordinates.
- Apply requires `WEAVATRIX_ALLOW_SOURCE_EDITS=1` plus a short-lived, single-use token bound to
  that repository and exact recomputed plan.
- `rollback_last_apply` refuses to overwrite files that drifted after the transaction.
- Multi-file writes are journaled and crash-recoverable. Another Refactor process that is
  reading or writing the same repository receives `REPO_BUSY` until the call finishes. The
  confirm token stays usable.

Only edits with proven provenance are applied. A lexical or graph backend is deliberately more
conservative than a language server and must not be read as a claim of full compiler semantics.

## MCP methods

The default profile exposes these twelve methods. `--profile=rename` keeps only
`rename_symbol` and `rollback_last_apply`. A planning method returns an edit plan.
`apply_edit_plan` is the write for every plan except the two rename methods, which
apply themselves when `mode` is `"apply"`.

### `rename_symbol`

Renames one symbol from the graph. A bare name that matches more than one symbol
returns `NOT_FOUND` and the candidate ids. The preview lists the declaration and
every call or reference the graph proves, plus import lines of files the graph
proves import the declaring file. A same-named occurrence the graph does not
prove stays in `uncertainReferences` and is left unchanged. Completeness stays
`PARTIAL`. Repeat the same symbol and `new_name` with `mode:"apply"` and the
preview token to write. A missing source span is `NOT_A_SYMBOL`. A route label
is renamed as a path literal.

### `rename_related_symbols`

Renames several symbols in one transaction. The sites are the same ones
`rename_symbol` can prove, in every indexed language. Two symbols given one new
name, or two edits of the same site, return `CONFLICT`. A chain or a swap is
applied together and reported in `ordering`. Completeness stays `PARTIAL`.
Preview returns the token; `mode:"apply"` with that token writes the whole batch.

### `change_signature`

Plans an added or removed parameter in every indexed language. Kotlin is read
with the Java tokenizer. A typed language copies the previous parameter's type.
When the new parameter has a default, existing calls stay. Without a default,
proven calls are edited when the call supplies a value. A call the graph missed
is edited only when that name is declared once and its argument list can be
read; otherwise the call is listed. The result stays `PARTIAL`. Spread arguments
are reported and left unchanged. Apply the returned plan with `apply_edit_plan`.

### `edit_symbol`

Plans a replacement or insertion inside one declaration. The range runs from the
declaration through the matching brace or the indent block. When only the
signature line can be located, the plan is `PARTIAL` and warns
`DECLARATION_END_UNPROVEN`. Every plan warns that syntax was not checked. Apply
it with `apply_edit_plan`.

### `bulk_replace`

Plans a literal or regular-expression replacement over indexed code, in two
stages: list the occurrences, then plan the ones you accept. Comments and strings
are included only when `include_trivia` is true. An extension the tokenizer does
not know still skips `//` comments, block comments, quotes, and a `#` that starts
a line. The caps are 5000 files, 500 matches in one file, and 5000 matches in
the plan. Apply the plan with `apply_edit_plan`.

### `organize_imports`

Plans removal of a named binding whose identifier occurs once in the file. A
Rust snake_case `use`, a Python `from` import or alias, a Go path segment, a
Java import, a C# `using`, a C++ `using` name, and a Kotlin `import` are removed
when that is true. A CamelCase Rust import such as `use std::io::Write`, a
`pub use`, a bare Python `import`, a Go blank or dot import, a star import, a
C++ `#include`, and a `using namespace` stay. JavaScript and TypeScript default
and namespace imports stay uncertain. A file whose extension has no recognised
import syntax returns `NO_UNUSED_IMPORTS`. Apply the plan with `apply_edit_plan`.

### `move_file`

Plans the filesystem rename and the relative import rewrites in one transaction.
A missing parent directory is created at apply and removed on rollback when it
is empty. A non-relative specifier whose last segment is the moved file is
reported, left unchanged, and the plan stays `PARTIAL`. Apply the plan with
`apply_edit_plan`.

### `move_symbol`

Reviews whether the move would cycle, then returns an edit plan. The status
stays `EVALUATED`. The plan moves the declaration, retargets the imports it can
prove, and copies the imports the declaration uses. Apply that plan with
`apply_edit_plan`.

### `delete_readiness`

Reports whether anything still depends on a symbol. It writes nothing. A private
symbol with no incoming use can be `safe: true`. The verdict stays
`REVIEW_REQUIRED`.

### `delete_closure`

Plans deletion of one symbol, endpoint, or file together with the callees that
nothing outside that closure uses. `scope:"exclusive"` returns `BLOCKED` when
another contract still uses the selection, including another file that imports
the selected file. `scope:"connected"` also removes direct referrers, one hop.
A repository returns `BLOCKED`. A module, a package, or any other symbol with
no source span returns `NOT_A_SYMBOL`. A handler on the following line of an
unclosed call is part of that route, including a uniquely named handler in
another file. A closer line may end with a `//` or `/* */` comment. The plan
stays `PARTIAL`. Apply it with `apply_edit_plan`.

### `apply_edit_plan`

Verifies a `weavatrix.edit-plan.v1` envelope and, with `mode:"apply"`, writes it.
Preview returns `PREVIEW_OK` and a single-use token bound to that plan and
repository. Apply requires `WEAVATRIX_ALLOW_SOURCE_EDITS=1` and that token. A
rename entry missing `from`, `to`, or `sha256`, or a delete entry missing `path`
or `sha256`, refuses the whole plan as `INVALID_PLAN`. A plan whose hashes do
not match the working tree is `PREVIEW_BLOCKED` or `STALE`. Another process
reading or writing that repository returns `REPO_BUSY`, and the confirm token
stays usable.

### `rollback_last_apply`

Restores the newest retained apply for that repository. An empty directory a
rename created is removed. A later edit of the same path invalidates the older
receipt, so two applies roll back in order only when they touch different paths.
The call requires the write gate. Another process using the repository returns
`REPO_BUSY`. When nothing has been applied, the status is `NO_CHANGE`.

## Measured benchmark, with unlike layers kept separate

The checked-in August 2026 benchmark uses equivalent adversarial TypeScript, Rust, and Python
fixtures. Each has seven required edits, four traps, and a build/test gate. Protocol runs count
exact `o200k_base` initialization, catalog, request, and response tokens. Agent runs use the
actual cumulative usage reported by Codex CLI with pinned `gpt-5.6-sol` / medium reasoning.
No byte proxy or estimated tokens-per-second conversion is used.

The new rename-only profile kept all nine protocol runs at 12/12 while reducing fixed MCP context
from 9,331 to 485 tokens. Its guided TypeScript agent A/B is:

| Arm | Correctness | Median end-to-end | Median input tokens |
| --- | ---: | ---: | ---: |
| Naked Codex | **12/12 x3** | **42,602 ms** | **72,169** |
| Weavatrix 1.0.6 `--profile=rename` | **12/12 x3** | 48,389 ms | 108,155 |
| Weavatrix 1.0.5 full surface | **12/12 x3** | 58,719 ms | 275,784 |
| Serena | **12/12 x3** | 68,728 ms | 388,271 |

The full agent matrix is 12/12 in all 27 TypeScript/Rust/Python runs; every run also completed
operationally and passed the sanitized edit-channel audit. The deterministic protocol
matrix also exposed a language-specific Serena defect: its suggested `find_symbol` →
`rename_symbol` flow scored 7/12 on TypeScript in all three runs, while Rust and Python passed.
It also found and drove fixes for Rust macro calls and Python f-string/module-level call evidence.

On this toy TypeScript repository naked Codex is still faster and cheaper than the narrowed
profile. Weavatrix's demonstrated value is preview-before-write, named uncertainty, stale-plan
refusal, explicit authorization, and rollback—not a universal small-repository token win.

See the [full methodology, all language medians, raw-result links, and limitations](docs/benchmarks/refactor-vs-competitors-2026-08.md).

## Contract

The 12 Refactor tool names, schemas, and result states are frozen in
[`contract/refactor-tools.v1.json`](contract/refactor-tools.v1.json). All refactor operations are
native Rust; the host uses the read-only
[`weavatrix-rust`](https://github.com/Weavatrix/weavatrix-rust) engine
internally for graph evidence without exporting its methods on the default
surface.

Versions 0.1.x used the JavaScript engine. That line continues as
[`weavatrix-refactor-js`](https://github.com/Weavatrix/weavatrix-refactor-js) and keeps a
separate state directory, token store, and rollback journal.

## License

MIT.
