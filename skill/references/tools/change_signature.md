# `change_signature`

Use for adding or removing a parameter in any indexed language. A typed language copies the previous parameter's type. Always review the partial result.

## Boundary

The declaration is edited in every indexed language. With a default, existing calls stay. Without one, those calls are listed and left unchanged unless a call-site value is given. The result stays `PARTIAL` when a site is uncertain. Spread and value-add sites remain uncertain.

## Inputs

- `operation` (object, required).
- `symbol` (string, required).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "change_signature",
  "arguments": {
    "operation": {},
    "symbol": "<exact symbol id>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
