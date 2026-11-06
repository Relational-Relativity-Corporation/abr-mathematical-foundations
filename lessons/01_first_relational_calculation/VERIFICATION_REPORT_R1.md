# Lesson 1 — Verification Report

> **Revision 2:** this is the revision-1 verification report, preserved unchanged below this note. It covers the two-locus exercise only, which is now the supporting exercise (run with `cargo run --bin lesson01 -- <file>`). The revision-2 report is [`VERIFICATION_REPORT.md`](VERIFICATION_REPORT.md).

**Prepared by:** Generator (AI), 2026-11-05. **For:** Origin, and an independent Verifier.
**Status:** submitted for review. The Generator does not certify this lesson as verified.

Everything below was observed during execution on 2026-11-05: Windows 11 Home 10.0.26200, `cargo 1.97.1 (c980f4866 2026-06-30)`, `rustc 1.97.1 (8bab26f4f 2026-07-14)`. Nothing has been committed or pushed.

---

## 1. Evidence table

| Requirement | Evidence | Status |
|---|---|---|
| **Canonical authority** | `abr-kernel` `1c1ef70a69b3175a2df0458d0ebf1f71475a3d62`; `abr-irrs` `c7410c590d04a96612619e639afdce7f0751aeca`. Both are pinned by `rev` in `Cargo.toml` and resolved in `Cargo.lock` (§2) | — |
| **Declaration** | `declarations/experiment_1_illustrative.txt`. D: printed scale [0, 300] mm (inclusive). M: Identity, mm → mm, preserves/discards stated. Loci `locus.L1`, `locus.L2`. Topology: one relation `rel.L1_to_L2` = L1 → L2, class Continuation, direction and class bases stated. Kernel selection Primary with basis; ρ_base, χ₀ not declared | DECLARED (illustrative) |
| **Admission (IRRS)** | `relational_gate::RegionBuilder::establish` → region established; `AdmissionGate::submit` → L1 ADMITTED (raw 20.0, gate-computed 20.0 mm), L2 ADMITTED (raw 90.0, gate-computed 90.0 mm) | ADMITTED |
| **Admission (V9)** | `abr_kernel::admission::admit` → `Admitted`, observational status `Synthetic` | ADMITTED (provenance conformance only) |
| **Operator** | `abr_kernel::operators::operator_delta` (`src/operators.rs`), called by `abr_kernel::provenanced_exec::execute_primary` (`src/provenanced_exec.rs`). **Operator A (`operator_a`) was not called** (§4) | — |
| **Execution** | `cargo run -- declarations/experiment_1_illustrative.txt`, exit 0. Output: `EXPECTED_RESULTS.md`, `records/experiment_1_illustrative.report.txt` | CALCULATED: Δ[rel.L1_to_L2] = −70.0 mm |
| **Provenance** | V9 calculation record (inputs, relation traversal, path `[Delta]`); admission entry with contributing declarations `locus.L1, locus.L2, rel.L1_to_L2`; reverse trace terminating in `obs.L1`, `obs.L2` (`SyntheticFixture`, named illustrative record); IRRS gate `ProvenanceRecord` per reading; Σ `NotEvaluated` (ρ_base, χ₀ missing) | Recorded |
| **Repeat experiment** | `declarations/experiment_2_illustrative.txt` (L2 → 140 mm), exit 0: both readings ADMITTED; V9 ADMITTED (`Synthetic`); Δ[rel.L1_to_L2] = −120.0 mm | CALCULATED |
| **Observation** | All runs use **illustrative** values (`observation_status = illustrative`, V9 `OriginKind::SyntheticFixture`). No independent physical measurement was made | Illustrative |
| **Comparison** | No comparison with observation was made (`comparison_outcome = not_compared`) | **OPEN** (demonstration only) |
| **Limitations** | §5 and §6 | — |

## 2. Canonical baseline

**Kernel (`abr-kernel` `1c1ef70`, clean checkout).** `cargo test` passed 61 lib, 3 `provenance_demo`, 12 `v9_admission`, 35 `v9_conformance` and 1 doctest: 112 passed, 0 failed, 0 ignored.

**IRRS (`abr-irrs` `c7410c5`, clean checkout).**

- `ABR_KERNEL_DIR=<kernel checkout> cargo test --workspace` **does not compile.** `examples/irrs-e2e-tests` imports `irrs_session` items that are no longer present (`error[E0432]: unresolved imports irrs_session::admission::DeclaredObservable, …`, `unresolved import irrs_session::session`, and 8 more errors). This is consistent with commit `73b0b12` ("rewrite execution interface"). It is a finding about IRRS at the pinned commit, returned to Origin. Nothing in `abr-irrs` was modified.
- With that one package excluded (`--exclude irrs-e2e-tests`), 189 passed and 0 failed. By test target: irrs-align records 16; irrs-eval authorization 3, continuous_process 4, continuous_process_extended 15, kernel_conformance 5, v9_semantics 9; irrs-kernel 61 + 3 + 12 + 35 + 1 doctest; irrs-session execute_tests 18; relational-gate gate_v9 7.
- `cargo run -p irrs-eval -- verify-kernel --kernel-dir <kernel checkout>` exited 0: "V9 conformance complete: …abr-kernel.git at 1c1ef70… (Origin-authorized); 10 canonical files equal." `identity_verified: true`, `source_equal: true`.

**Lesson executable.** `cargo build` finished with no warnings or errors. Resolved sources:

```
abr-kernel:      git+https://github.com/Relational-Relativity-Corporation/abr-kernel?rev=1c1ef70a69b3175a2df0458d0ebf1f71475a3d62#1c1ef70a69b3175a2df0458d0ebf1f71475a3d62
relational-gate: git+https://github.com/Relational-Relativity-Corporation/abr-irrs?rev=c7410c590d04a96612619e639afdce7f0751aeca#c7410c590d04a96612619e639afdce7f0751aeca
```

**Reproduction instructions.** The lesson files were copied to an empty directory and built with an empty `CARGO_HOME`, which forced a fresh fetch of both pinned repositories from GitHub. `cargo run -- declarations/experiment_1_illustrative.txt` exited 0. Its report was identical to `records/experiment_1_illustrative.report.txt` except for the two file-path lines.

## 3. Admission pathway identified

| Stage | Canonical code | Why this one |
|---|---|---|
| IRRS observation admission | `relational_gate::{RegionBuilder, AdmissionGate}` (`abr-irrs` `relational-gate/src/region.rs`, `gate.rs`, `admitted.rs`) | This is the IRRS enforcement primitive. Only the gate constructs `AdmittedObservation<T>`, and the lesson passes only gate-admitted values (`gate_result()`) into the kernel records |
| Canonical execution | `abr_kernel::provenanced_exec::execute_primary` → `operator_delta` | The Generator execution path that writes a V9 calculation record. It builds `DeclaredRelations` and `NodeField` through the canonical constructors |
| V9 provenance admission | `abr_kernel::admission::admit` | Independent reconstruction from the declaration registry, with the termini traced. It is the only canonical admission that accepts a calculation record |

Not used, and why:

- `irrs_session::execute` runs A → B → R. It requires ρ_base, χ₀ and a detection tolerance, none of which this lesson declares, and it does not check for an interior locus.
- `irrs-eval evaluate` is a pre-build declaration evaluator that executes no operator. Without a Phase 2 persistence block it reports the continuous-process posture as OPEN.

See question Q-2.

## 4. Operator A

The brief asks for operator A. Under the canonical rules A is **not evaluable** for this declaration:

- `kernel_reference.md` §10: A → B → R is admissible only when M establishes an interior locus (s → t → u). Two loci with one relation have none, so the Primary kernel Δ → Σ applies.
- `operators_notation_and_constraint.md`: "Δ is A under a declared renaming … Formula identical to A."
- `src/admission.rs` (supported scope): "OPEN: ABR kernel A → B → R". `reconstruct_primary` returns `UnsupportedScope` for `KernelSelection::Abr`.
- Observed in run 3: `execute_primary` returned `NotExecutable { reason: "kernel selection is not Primary" }`. The program reports operator A as OPEN and makes no substitute calculation.

The lesson does not call `operator_a` directly. Doing so would bypass kernel selection and both admission stages. See question Q-1.

## 5. What the lesson's own code does (for Verifier review)

`src/main.rs` contains no operator, admission rule or comparison rule. It does the following, all of which should be reviewed:

1. Parses `key = value` declaration files.
2. Supplies the **declared M** to the IRRS gate as `AuthorizedMapping` closures: `execute = |p, _| Ok(p)` (Identity) and `to_scalar = |v| *v`. The gate API requires them. They encode the M kind the student declares, matching V9 `MMappingKind::Identity`.
3. Establishes the IRRS region with the source-locus reading as its observational evidence (`establish(observed_locus)`).
4. Fills the V9 `DeclarationRegistry` from the declared text and the gate-admitted values, with relation class `Continuation` fixed by the lesson.
5. Sets `observable_source` of the relation to the target-locus reading (Q-3).
6. Derives the status lines by matching outcome variants and declared strings. It never compares numbers. **VERIFIED** is printed only when `observation_status = measured`, every comparison key is declared, and `comparison_outcome = agrees`. **DIVERGENT** is printed for `disagrees`. Otherwise the line reads **OPEN**.
7. Converts declared text to `&'static str` with `Box::leak`, because the relational-gate API takes static strings.

## 6. Limitations and conditions not exercised

- **No independent observation.** The Generator cannot make a physical measurement. Observational verification is **OPEN** for every run, and nothing in this report is experimentally verified.
- The **VERIFIED and DIVERGENT branches were not executed.** Exercising them would need a `measured` declaration, and writing one without an actual measurement would fabricate an observation. The measured branch was run only with the empty template, which stopped at Step 1 with the essential keys reported missing.
- Generator checks outside the lesson set: a reading outside the declared extent (L2 = 350 mm) gave IRRS `OPEN: value 350 outside declared extent [0, 300] — recorded, not admitted`, and the kernel was not reached. A reversed relation (L2 → L1) gave Δ = +70.0 mm, admitted.
- V9 `admit` is reconstruction **within the kernel crate**, not independent IRRS verification from a distinct frame (`src/admission.rs` header; V9 milestone 7 OPEN).
- IRRS automated correspondence evaluation (M4) is planned and not operational (`abr-irrs` `README.md`). The lesson's comparison is the student's recorded judgement.
- Kernel OPEN conditions apply but are not reached in Lesson 1, because Σ, ρ and R are not evaluated: A-3 empty adjacency, ρ max-selection, ρ saturating form.

## 7. Open questions for Origin

| # | Question |
|---|---|
| Q-1 | **Operator A vs Δ.** The brief names operator A. The canonical rules make A non-evaluable for two loci, so the lesson runs canonical Δ ("A under a declared renaming") and reports A as OPEN. Is this the intended treatment, or should Lesson 1 be redesigned, for example with three loci and a declared interior locus, ρ_base and χ₀? That would bring in B and R, which the brief reserves for later lessons. |
| Q-2 | **IRRS pathway.** Is relational-gate observation admission plus V9 `admit` the IRRS admission pathway Origin intends, rather than `irrs_session::execute` or `irrs-eval`? |
| Q-3 | **Relation `observable_source`.** The V9 relation record names one observation. The lesson names the target-locus reading and lists both readings in the bases. Is that acceptable, or should the relation cite a separate observation? |
| Q-4 | **Direction basis for a static pair.** The example declares L1 → L2 from "the direction in which the printed scale increases between them". Is this an admissible observable basis for direction? The canonical examples of direction come from processes. |
| Q-5 | **VERIFIED by the student's comparison.** The lesson records VERIFIED from the student's own comparison of the admitted result with a direct separation reading. GD-8 reserves VERIFIED for an executed correspondence on measured data, which is planned M4. Should Lesson 1 instead report "Origin comparison recorded" and keep VERIFIED for M4? |
| Q-6 | **IRRS at `c7410c5` does not compile `irrs-e2e-tests`.** Should IRRS be repaired under a separate authorization, and should the lesson stay pinned to `c7410c5` regardless? |
| Q-7 | **Generated records.** Should `records/` (the run reports) and `Cargo.lock` be committed as evidence? The draft keeps both and ignores `target/`. |

## 8. Generator recommendation

The lesson is **ready for independent Verifier review** of its conformance, with Q-1 decided by Origin first. Q-1 determines whether the lesson as built (canonical Δ, with A reported OPEN) meets the brief.

It is **not** ready to be described as observationally verified. That needs a student's actual measurement and comparison, followed by the Verifier's review of that record.

*Bounded over D. No claim beyond D.*
