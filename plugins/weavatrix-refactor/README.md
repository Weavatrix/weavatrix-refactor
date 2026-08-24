# Weavatrix Refactor plugin

Weavatrix Refactor adds 11 evidence-backed refactoring methods to Cursor,
Codex, Claude Code, and Grok Build. The plugin deliberately uses the
`--profile=refactor` extension-only surface, so installing the separate
Weavatrix core plugin does not load its 43 read-only tools twice.

The launcher starts `weavatrix-refactor@1.0.8` from npm. Node.js 18 or newer
is required.

The bundled skill is optional. It activates only for an explicitly requested
source change that benefits from a proven refactor plan. The skill entry point
stays compact; one small method card is loaded only when needed.

## Safety

The plugin opens the process write gate, but that alone cannot write source.
Every applying operation still requires a preview, review of the exact plan,
and a short-lived single-use token bound to that plan and repository.
Uncertain references are reported rather than guessed. Rollback refuses to
overwrite files that drifted after the apply.

## Install

### Cursor

Search for **Weavatrix Refactor** in Cursor's Plugins view after marketplace
approval. For local testing, copy or link this directory to
`~/.cursor/plugins/local/weavatrix-refactor`, then reload Cursor.

### Codex

```text
codex plugin marketplace add sergii-ziborov/weavatrix-refactor --sparse .agents/plugins plugins/weavatrix-refactor
codex plugin add weavatrix-refactor@weavatrix-refactor
```

### Claude Code

```text
claude plugin marketplace add sergii-ziborov/weavatrix-refactor --sparse .claude-plugin plugins
claude plugin install weavatrix-refactor@weavatrix-refactor
```

### Grok Build

```text
grok plugin marketplace add sergii-ziborov/weavatrix-refactor
```

Open `/marketplace` and install Weavatrix Refactor.

## Source and support

- Website: https://weavatrix.com
- Repository: https://github.com/sergii-ziborov/weavatrix-refactor
- Issues: https://github.com/sergii-ziborov/weavatrix-refactor/issues
