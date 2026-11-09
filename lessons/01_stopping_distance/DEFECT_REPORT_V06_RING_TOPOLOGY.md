# Canonical Defect Report V-06 — a directed ring reaches A → B → R

**To:** the Kernel V9 authority (Origin), for disposition.
**From:** the Generator, through `abr-mathematical-foundations` Lesson 1.
**Pin:** Kernel V9 `abr-kernel` `1c1ef70`.

This report proposes no fix to canonical code. Lesson 1 adds no ring-rejection rule of its own.

## 1. Prohibited topology

Fixture: `declarations/negative_tests/neg4_directed_ring.json`. It is the lesson's F3 response-chain declaration with one added relation, Tread → Obstacle, which closes a directed ring:

```
Obstacle → Driver → Brake → Tread → Obstacle
```

## 2. Expected behaviour (Kernel V9)

- `kernel_reference.md` §8: "**Rings and tori are inadmissible.** A closing edge declares a node as its own relational predecessor. Any ring, torus, `np.roll`, or periodic-index operation is a DRIFT SIGNAL. Refuse it and require declared relations with provenance."
- `abr-kernel/src/operators.rs` header: "Ring topology is inadmissible … Refuse it and require declared relations with provenance."

So the declaration should be refused (REJECTED, an established contradiction) before any operator executes.

## 3. Actual behaviour

Reproduce with `cargo run` (all fixtures) or `cargo run -- declarations/negative_tests/neg4_directed_ring.json`. The report is in `records/negative_tests/neg4_directed_ring.report.txt`.

| Stage | Result |
|---|---|
| Declaration evaluation | **OPEN, 15 conditions, 0 violations.** The ring is not detected. The only conditions are "proposed" support, Phase 2 and illustrative parameters |
| Structure construction | accepts the closing edge (asserts only endpoint range, provenance length, and no Persistence class) |
| Execution | **A, ρ, B, R and 𝟙 executed**, and the RPPI re-execution agreed |

Lesson 1 labels this execution a non-authoritative diagnostic, because the record is OPEN. **The defect is unchanged by that label.** The pipeline returns OPEN, not REJECTED, for a declaration that V9 says must be refused. An OPEN record can later be authorized, so the ring would then reach authorized execution.

## 4. Responsible boundary (as identified)

- **Kernel `operators::DeclaredRelations` constructors.** No directed-cycle check.
- **The kernel's only ring check is diagnostic and test-only.** `has_undirected_cycle` sits in the `operators.rs` test module, "DIAGNOSTIC ONLY — ISOLATE", "pending Origin determination of whether either is required in the kernel proper".
  - It tests **undirected** cycles. It would therefore also flag admissible directed acyclic structures: Lesson 1's F5 (the chain plus Onset → Standstill) and the four condition diamonds.
  - V9's prohibition is about a node being "its own relational predecessor", which is a **directed** cycle.

## 5. Proposed regression test (for the canonical repository, not applied)

```rust
// abr-kernel/tests/ (proposed): a directed ring is refused.
#[test]
fn directed_ring_is_rejected() {
    // nodes A, B, C; edges A→B, B→C, C→A (closing edge)
    let result = try_declare_edges(&["A", "B", "C"], &[("A","B"), ("B","C"), ("C","A")]);
    assert!(result.is_err(), "directed ring must be refused");
}

// Companion: a directed acyclic diamond (undirected cycle, no directed cycle)
// must NOT be refused on ring grounds.
#[test]
fn directed_acyclic_diamond_is_not_a_ring() {
    let result = try_declare_edges(&["A","B","C","D"], &[("A","B"), ("A","C"), ("B","D"), ("C","D")]);
    assert!(result.is_ok(), "directed acyclic diamond must be accepted");
}
```

The helpers stand for whatever fixture builder the kernel test suite uses. The error wording is the authority's to choose.

## 6. Requested disposition

1. Confirm that the kernel's `DeclaredRelations` constructor must refuse a directed cycle, and whether a conformance test should be added.
2. Confirm that the test is for **directed** cycles, so that directed acyclic structures with undirected cycles (the diamonds, F5) remain admissible.

*Bounded over D. No claim beyond D.*
