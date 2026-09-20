# meaning-language

**Proposal, not adopted Vision.** This is the base library proposal for a
bilingual Sanskrit–English lower ontology. It is deliberately a map to BFO
and UFO/gUFO, not an import of either ontology.

`ethos/meaning.ethos` is the schema authority. `src/generated/meaning.rs` is
only an Ethos Zero projection. The glossary and mapping are machine-readable;
run `tests/check-glossary-coverage.sh` to ensure every declared public semantic
name has both an IAST and English name and an outbound mapping record.

The repository specifies a content-addressable graph boundary. It does not
implement a store, hashing policy, garbage collector, reference counting, or
cycle policy.

The ASCII identifiers in the Ethos wire schema are stable schema names. The
corresponding IAST and PascalCase English names are first-class metadata;
`schema_id` is the unique stable identity, while Sanskrit names may be shared.
They are not parser aliases. Render the bilingual
table from that metadata with `scripts/render-bilingual-table.sh`.

The expressible-kind root and One/Many main shape remain open. The placement of
`Padartha` relative to that future expression boundary is unresolved.

`docs/PROVENANCE.md` records the initial path-reservation deviation before the
valid lock receipt. It is factual process provenance, not ontology authority.
