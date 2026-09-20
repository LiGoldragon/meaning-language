# Retention contract proposal

A linked item is retained by virtue of the link; when its last retaining link
goes, it is released, in the Nix-like sense ruled for this graph. A retained
item is a completed root statement, annotation, or linked subpart identified
by `ContentLink`. Its checksum is intended to cover the item and the links it
contains. The locator is an on-demand index hint, never identity.

The collection mechanism, archive representation, canonical text
representation, cycle handling, and retention algorithm remain open. This
document specifies no refcount-only policy and no live store behavior.

The checked library can verify Datom structure only. Hash verification,
content retrieval, retention reachability, and semantic interpretation require
a later runtime and remain outside this proposal.
