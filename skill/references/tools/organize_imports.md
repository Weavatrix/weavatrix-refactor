# `organize_imports`

Use only for conservative removal of a named import whose identifier occurs once.

## Boundary

JavaScript and TypeScript default and namespace imports stay uncertain, and formatting is not performed. A Rust snake_case `use`, a Python `from` import or alias, a Go path segment, a Java import, a C# `using`, a C++ `using` name, and a Kotlin `import` are removed when unused. A CamelCase Rust import such as `use std::io::Write`, a `pub use`, a bare Python import, a Go blank or dot import, a star import, a C++ `#include`, and a `using namespace` stay. A file whose extension has no recognised import syntax returns `NO_UNUSED_IMPORTS`.

## Inputs

- `file` (string, required).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "organize_imports",
  "arguments": {
    "file": "<file>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
