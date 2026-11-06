# Canonical Defect Report V-06 — a directed ring reaches A → B → R

**To:** the IRRS and Kernel V9 authority (Origin), for disposition.
**From:** the Generator, through `abr-mathematical-foundations` Lesson 1.
**Pins:** `abr-irrs` `c7410c5`; Kernel V9 `abr-kernel` `1c1ef70`, and its byte-identical IRRS copy `irrs-kernel`.

This report proposes no fix to canonical code. Lesson 1 adds no ring-rejection rule of its own.

## 1. Prohibited topology

Fixture: `declarations/negative_tests/neg4_directed_ring.json`. It is the lesson's F3 response-chain declaration with one added relation, Tread → Obstacle, which closes a directed ring:

```
Obstacle → Driver → Brake → Tread → Obstacle
```

## 2. Expected behaviour (Kernel V9)

- `kernel_reference.md` §8: "**Rings and tori are inadmissible.** A closing edge declares a node as its own relational predecessor. Any ring, torus, `np.roll`, or periodic-index operation is a DRIFT SIGNAL. Refuse it and require declared relations with provenance."
- `irrs-kernel/src/operators.rs` header (identical in `abr-kernel`): "Ring topology is inadmissible … Refuse it and require declared relations with provenance."

So the declaration should be refused (REJECTED, an established contradiction) before any operator executes.

## 3. Actual behaviour

Reproduce with `cargo run` (all fixtures) or `cargo run -- declarations/negative_tests/neg4_directed_ring.json`. The report is in `records/negative_tests/neg4_directed_ring.report.txt`.

| Stage | Canonical call | Result |
|---|---|---|
| IRRS declaration evaluation | `irrs_eval::Evaluator::evaluate` | **OPEN, 15 conditions, 0 violations.** The ring is not detected. The only conditions are "proposed" support, Phase 2 and illustrative parameters |
| IRRS observation admission | `relational_gate::AdmissionGate::submit` | all 4 readings ADMITTED |
| Structure construction | `irrs_kernel::operators::DeclaredRelations::from_edges_with_provenance` | accepts the closing edge (it asserts only endpoint range, provenance length, and no Persistence class) |
| Execution | `irrs_session::execute::execute` | **A, ρ, B, R and 𝟙 executed**, and the RPPI re-execution agreed |

Since the V-01 correction, Lesson 1 labels this execution a non-authoritative diagnostic, because the IRRS record is OPEN. **The defect is unchanged by that label.** IRRS returns OPEN, not REJECTED, for a declaration that V9 says must be refused. An OPEN record can later be completed and authorized, so the ring would then reach authorized execution.

## 4. Responsible boundary (as identified)

- **`irrs-eval` Stage 2 ABR topology admission** (`irrs-eval/src/evaluator.rs`). It checks that edge endpoints are in the declared node inventory (GR-4; see fixture `neg3`, which is REJECTED). It has no check that a relation closes a directed cycle. This is the natural place to refuse the declaration before admission.
- **`irrs_kernel::operators::DeclaredRelations` constructors.** No cycle check.
- **The kernel's only ring check is diagnostic and test-only.** `has_undirected_cycle` sits in the `operators.rs` test module, "DIAGNOSTIC ONLY — ISOLATE", "pending Origin determination of whether either is required in the kernel proper".
  - It tests **undirected** cycles. It would therefore also flag admissible directed acyclic structures: Lesson 1's F5 (the chain plus Onset → Standstill) and the four condition diamonds.
  - V9's prohibition is about a node being "its own relational predecessor", which is a **directed** cycle.

## 5. Proposed regression test (for the canonical repository, not applied)

```rust
// irrs-eval/tests/ (proposed): a directed ring is refused before admission.
#[test]
fn directed_ring_is_rejected() {
    // nodes A, B, C; edges A→B, B→C, C→A (closing edge)
    let decl = declaration_with_edges(&["A", "B", "C"], &[("A","B"), ("B","C"), ("C","A")]);
    let rec = Evaluator::evaluate(decl, digest_of(&decl));
    assert_eq!(rec.status, EvaluationStatus::Rejected);
    assert!(rec.violations.iter().any(|v| v.to_string().contains("ring")));
}

// Companion: a directed acyclic diamond (undirected cycle, no directed cycle)
// must NOT be refused on ring grounds.
#[test]
fn directed_acyclic_diamond_is_not_a_ring() {
    let decl = declaration_with_edges(&["A","B","C","D"], &[("A","B"), ("A","C"), ("B","D"), ("C","D")]);
    let rec = Evaluator::evaluate(decl, digest_of(&decl));
    assert!(rec.violations.iter().all(|v| !v.to_string().contains("ring")));
}
```

The helpers `declaration_with_edges` and `digest_of` stand for whatever fixture builder the IRRS test suite uses. The violation wording is the authority's to choose.

## 6. Requested disposition

1. Confirm that a directed cycle must be REJECTED at IRRS Stage 2, and whether the kernel's structure constructor should refuse it too.
2. Confirm that the test is for **directed** cycles, so that directed acyclic structures with undirected cycles (the diamonds, F5) remain admissible.

*Bounded over D. No claim beyond D.*
