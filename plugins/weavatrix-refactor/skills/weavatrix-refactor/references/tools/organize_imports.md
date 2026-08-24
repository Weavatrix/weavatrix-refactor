# `organize_imports`

Use only for conservative removal of provably unused JavaScript or TypeScript named imports.

## Boundary

Default and namespace imports are reported uncertain, not removed; formatting is not performed.

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
