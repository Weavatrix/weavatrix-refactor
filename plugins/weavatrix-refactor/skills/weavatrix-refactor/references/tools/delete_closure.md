# `delete_closure`

Use to plan deletion of one symbol, endpoint, or file together with the callees that nothing outside that closure uses. It does not write. Apply the returned edit plan with `apply_edit_plan`.

## Boundary

A repository returns `BLOCKED`. Exclusive mode returns `BLOCKED` when another file still imports the selected file. A module, a package, or any other symbol with no source span returns `NOT_A_SYMBOL`. A handler on the following line of an unclosed call is part of that route, including a uniquely named handler in another file. A closer line may end with a `//` or `/* */` comment. `scope` defaults to `exclusive`. That mode returns `BLOCKED` when another contract still uses the selection, and it lists shared helpers in `kept` instead of deleting them. An endpoint whose handler is shared still plans removal of that route only. A function also removes the one route that passes it as a handler; two such routes block, so the other endpoint stays. `scope:"connected"` also deletes the selection's direct referrers, and for an endpoint the shared handler plus that handler's direct callers. For a file, connected mode lists the importing files in `affected`. Callers of those referrers are listed in `affected` and are not walked further. More than 200 files returns `BLOCKED`. Completeness is `PARTIAL`: the plan proves the uses the graph recorded, plus a handler named on the route statement when the graph has no edge for that argument.

Select an endpoint to remove that route and a handler no other function calls. Select a function when the function itself is the thing to remove. `delete_readiness` stays a verdict and never plans a deletion.

## Inputs

- `symbol` (string, required) — exact symbol or endpoint id.
- `scope` ("exclusive" | "connected", default "exclusive").
- `output_format` ("text" | "json", default "text").

## Minimal call

```json
{
  "name": "delete_closure",
  "arguments": {
    "symbol": "<exact symbol id>",
    "scope": "exclusive"
  }
}
```

Preview and inspect `deleted`, `kept`, and `affected` before any apply. The live MCP `tools/list` schema remains authoritative for this installed version.
