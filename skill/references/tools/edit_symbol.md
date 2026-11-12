# `edit_symbol`

Use for a source-range anchored replace-body, insert-before, or insert-after edit when the exact symbol is known.

## Boundary

The declaration end is the matching brace, the indent block, or the signature line. A signature-line end is `PARTIAL` and warns `DECLARATION_END_UNPROVEN`. The plan warns that syntax was not checked. An unclosed body stops at the signature instead of consuming the next function.

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
