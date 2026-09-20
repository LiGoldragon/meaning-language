# Validation status

Status: **UNVALIDATED**.

The checked-in Rust projection was generated locally before the remote-only
resource correction. It is stale after later schema changes and is not a
buildability or Datom-roundtrip claim. The single Prometheus-only validation
invocation is still blocked in Nix input processing before remote dispatch.

The follow-up is to run the `generation-and-datom-roundtrip` Nix check from
the final source snapshot with `--max-jobs 0` and `fallback=false`, then retain
its remote generation, canonical-example, and test receipt.
