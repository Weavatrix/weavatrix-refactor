# `edit_symbol`

Use for a source-range anchored replace-body, insert-before, or insert-after edit when the exact symbol is known.

## Boundary

JavaScript and TypeScript output is parse-gated; other languages rely on the parser's exact source range.

## Inputs

- `content` (string, required).
- `operation` ("replace_symbol_body" | "insert_before_symbol" | "insert_after_symbol", required).
- `symbol` (string, required).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "edit_symbol",
  "arguments": {
    "content": "<content>",
    "operation": "replace_symbol_body",
    "symbol": "<exact symbol id>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
