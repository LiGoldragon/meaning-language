# Initial reservation deviation

The repository was created and the initial proposal files were written before
a valid Orchestrate lock receipt. Two obsolete request forms returned an
unreadable protocol error. After review, the current client was observed and
the exact path was locked as receipt `2754`, name `MeaningLanguageBase`, flow
`e0ba37`. The earlier writes are not represented as retroactively reserved.

The checked-in generated Rust was produced twice by local execution of a
cached `nix run` Ethos Zero tool before the resource-policy correction. It is
not evidence of remote-builder generation or of a successful roundtrip test.
A future refresh must run generation and tests inside a derivation assigned to
the configured Prometheus remote builder, with no local fallback.

The first remote-only `path:.` build was cancelled after input fetching because
review found its captured flake did not seed a valid project lock or wire Cargo
to the vendor closure. It produced no generation, compilation, or test receipt.
