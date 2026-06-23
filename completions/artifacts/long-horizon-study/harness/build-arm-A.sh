#!/usr/bin/env bash
# build-arm-A.sh — the jigc twin template: baseline @ 542b320 + jigc setup +
# the 8-component managed arch-doc (finalize-validated) + the BLOCKING pre-commit hook.
# Emits the promoted docs/architecture/core-public-api.md, copied byte-identical to
# the other arms by build-templates.sh.
set -euo pipefail
SRC="${SRC:-$HOME/Projects/gherrink-ui-doc}"
A="${A:-$HOME/lh-study/templates/A}"
BASELINE=542b320

rm -rf "$A"; mkdir -p "$A"
git -C "$SRC" archive "$BASELINE" | tar -x -C "$A"
cd "$A"
git init -q && git add -A && git -c user.email=s@s -c user.name=s commit -qm "baseline @ $BASELINE"
jigc setup >/dev/null

TASK=$(jigc start --workflow architecture-documentation "Document the core public API surface" \
        2>&1 | grep -oE 'task [a-z0-9-]+' | head -1 | awk '{print $2}')
jigc doc create arch-doc --title "Core Public API" >/dev/null
printf 'The public entry points of @ui-doc/core: the documentation engine, the parsers that turn source comments into a documentation context, and the error types they raise.' \
  | jigc doc set-slot "arch-doc:core-public-api#overview" --from-file - >/dev/null

# 7 real classes + 1 never-edited CONTROL (the BlockParser interface). Order = doc order.
COMPS=(
"UIDoc|packages/core/src/UIDoc.ts#UIDoc|The main documentation engine: ingests sources and emits the rendered documentation."
"CommentBlockParser|packages/core/src/CommentBlockParser.ts#CommentBlockParser|The default block parser: extracts comment blocks from source and builds the context via tag transformers."
"MarkdownDescriptionParser|packages/core/src/MarkdownDescriptionParser.ts#MarkdownDescriptionParser|Parses block descriptions written in Markdown into HTML."
"BlockParseError|packages/core/src/errors/BlockParseError.ts#BlockParseError|Raised when a comment block cannot be parsed."
"CSSParseError|packages/core/src/errors/CSSParseError.ts#CSSParseError|Raised when CSS input fails to parse."
"ColorParseError|packages/core/src/errors/ColorParseError.ts#ColorParseError|Raised when a color value cannot be parsed; a specialization of CSSParseError."
"TagTransformerError|packages/core/src/errors/TagTransformerError.ts#TagTransformerError|Raised when a tag transformer rejects its input."
"BlockParser|packages/core/src/BlockParser.types.ts#BlockParser|The interface every block parser implements."
)
for c in "${COMPS[@]}"; do
  IFS='|' read -r title anchor desc <<< "$c"
  ADDR=$(jigc doc add-item "arch-doc:core-public-api#components" --title "$title" 2>&1 | tail -1)
  printf '%s' "$desc" | jigc doc set-slot "$ADDR/description" --from-file - >/dev/null
  jigc doc set-field "$ADDR/implemented-by" --value "$anchor" >/dev/null
done

jigc doc set-field "commit:$TASK#type" --value docs >/dev/null
jigc doc set-field "commit:$TASK#scope" --value arch-doc >/dev/null
printf 'document the core public API surface' | jigc doc set-slot "commit:$TASK#summary" --from-file - >/dev/null
jigc task finalize "$TASK" >/dev/null
echo "arm A finalized; promoted doc:"; ls -l docs/architecture/core-public-api.md
