# `change_signature`

Use for adding or removing a parameter in any indexed language. A typed language copies the previous parameter's type. Always review the partial result.

## Boundary

The declaration is edited in every indexed language, including Kotlin, which is read with the Java tokenizer. With a default, existing calls stay. Without one, those calls are listed and left unchanged unless a call-site value is given. A call the graph missed is edited only when the name is declared once and its argument list can be read; otherwise it is listed. The result stays `PARTIAL`. Spread calls stay uncertain.

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
