#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
schema="$root/ethos/meaning.ethos"
glossary="$root/glossary.json"
mapping="$root/mappings.json"

python3 - "$schema" "$glossary" "$mapping" <<'PY'
import json, re, sys
schema, glossary, mapping = map(open, sys.argv[1:])
text = schema.read()
declarations = re.findall(r'^\s+([A-Z][A-Za-z0-9]*)\.', text, re.M)
variants = [f'{name}.{token.split(".", 1)[0]}'
            for name, body in re.findall(r'^\s+([A-Z][A-Za-z0-9]*)\.\[([^\]]*)\]', text, re.M)
            for token in body.split()]
expected = set(declarations + variants)
glossary_entries = json.load(glossary)['entries']
mapping_entries = json.load(mapping)['records']
glossary_ids = {entry.get('schema_id') for entry in glossary_entries}
mapping_ids = {entry.get('schema_id') for entry in mapping_entries}
glossary_list = [entry.get('schema_id') for entry in glossary_entries]
mapping_list = [entry.get('schema_id') for entry in mapping_entries]
for label, identifiers in [('glossary', glossary_list), ('mappings', mapping_list)]:
    duplicates = sorted({identifier for identifier in identifiers if identifiers.count(identifier) > 1})
    if duplicates:
        raise SystemExit(f'duplicate {label} schema_id values: {duplicates}')
for entry in glossary_entries:
    if not isinstance(entry.get('english'), str) or not entry['english'].strip():
        raise SystemExit(f'empty English metadata: {entry.get("schema_id")}')
    if entry.get('iast') is not None and (not isinstance(entry['iast'], str) or not entry['iast'].strip()):
        raise SystemExit(f'empty IAST metadata: {entry.get("schema_id")}')
for label, actual in [('glossary', glossary_ids), ('mappings', mapping_ids)]:
    missing = sorted(expected - actual)
    if missing:
        raise SystemExit(f'missing {label} entries: {missing}')
print(f'coverage: {len(declarations)} declarations, {len(variants)} variants')
PY
