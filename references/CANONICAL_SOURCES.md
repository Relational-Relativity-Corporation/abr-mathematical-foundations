# Canonical Source References

**ABR Mathematical Foundations.** First draft, awaiting Origin review.

This curriculum has no mathematical authority of its own. Every definition, rule, operator form, and invariant it teaches comes from the two repositories below, at the pinned commits. Where the curriculum and a source differ, the source governs. Within `abr-kernel`, where `kernel_reference.md` and a source file differ, the source file governs (`kernel_reference.md`, preamble).

---

## Pinned authorities

| Repository | Role | Branch | Commit (full) | Inspected |
|---|---|---|---|---|
| [`Relational-Relativity-Corporation/abr-kernel`](https://github.com/Relational-Relativity-Corporation/abr-kernel) | Kernel V9: sole canonical declaration of the kernel (package `abr-kernel` 9.0.0) | `main` | `1c1ef70a69b3175a2df0458d0ebf1f71475a3d62` | 2026-11-05 |
| [`Relational-Relativity-Corporation/abr-irrs`](https://github.com/Relational-Relativity-Corporation/abr-irrs) | IRRS: admission and enforcement; carries the canonical Kernel V9 files byte for byte in `irrs-kernel/` | `main` | `c7410c590d04a96612619e639afdce7f0751aeca` | 2026-11-05 |

Both commits were read from the GitHub remotes (`origin/main`) on 2026-11-05. The IRRS governing declarations name `abr-kernel` `1c1ef70` as the kernel they apply, which matches the kernel pin above.

**Conformance baseline (Generator run, 2026-11-05, `abr-kernel` `1c1ef70`).** `cargo test` passed 61 library, 3 `provenance_demo`, 12 `v9_admission`, 35 `v9_conformance`, and 1 doctest: 112 in total, 0 failed, 0 ignored. This matches the counts stated in the kernel `README.md`. It is an implementation-conformance statement only.

---

## `abr-kernel` documents read for this draft

| File | Used for |
|---|---|
| `README.md` | Foundational primitive; RPPI; build and test; repository contents |
| `docs/kernel/kernel_reference.md` | Single reference: §1 Foundation, §2 Observational order, §3 Roles, §5 Workflow, §6 Language discipline, §9 Domain and measurement, §10 Kernel selection, §11 Operators, §13–§15 Derived invariants, §16 Execution discipline, §17 Verification, §18 Open conditions, §19 Suspended quantities, §20 Legacy intrusion |
| `docs/kernel/operators_notation_and_constraint.md` | **Operator authority:** formal operator definitions and constraints |
| `docs/kernel/abr_operators_plain.md` | Plain-language statement of the operators; "change is the primary observable" |
| `docs/kernel/role_separation_and_operator_application.md` | Roles and constraints; workflow; Verification and Validation Protocol; legacy mathematical intrusion |
| `docs/kernel/triad-constraint-methods.md` (headings and publication gate only) | Seven verification methods; Origin obligations O1, O2; publication gate |

Not yet read in full; needed for later modules: `docs/kernel/v9_specification.md` (Module 9), `docs/kernel/validation_record.md` and `observable_variable_sets.md` (Module 7), `cross_region_energy_expression.md` (Module 7), `docs/process/*` (Modules 8–10), `src/*.rs` and `tests/*.rs` (Modules 4–9).

## `abr-irrs` documents read for this draft

| File (at `c7410c5`) | Used for |
|---|---|
| `README.md` | Enforcement by construction; `AdmittedObservation<T>`; Regional M prerequisite; gate steps; admission outcomes; stage status (M1 executable; M2–M5 planned, OPEN) |
| `docs/GOVERNING_DECLARATIONS.md` | GD-1 to GD-8; rule W-1 (whole field); per-domain parameters; items not yet declared |

---

## Sources by module

| Module | Primary canonical sources |
|---|---|
| Preface | `kernel_reference.md` §1, §3, §16, §18, §20; `role_separation_and_operator_application.md`; `abr_operators_plain.md` |
| 1 Observation and Abstraction | `kernel_reference.md` §1–§3, §5, §9, §16, §20; `operators_notation_and_constraint.md`; `abr_operators_plain.md`; `role_separation_and_operator_application.md`; `abr-irrs` `README.md`, `docs/GOVERNING_DECLARATIONS.md` |
| 2 The Declared Domain | `kernel_reference.md` §1, §2, §6, §9; `operators_notation_and_constraint.md` |
| 3 Directed Relations and Topology | `operators_notation_and_constraint.md`; `kernel_reference.md` §8; `role_separation_and_operator_application.md` Step 1 |
| 4 Primary Operators | `operators_notation_and_constraint.md`; `kernel_reference.md` §10–§11; `src/operators.rs` |
| 5 Full Operator Path | `operators_notation_and_constraint.md`; `kernel_reference.md` §10–§11; `abr_operators_plain.md`; `src/operators.rs` |
| 6 Persistence | `operators_notation_and_constraint.md`; `kernel_reference.md` §8, §11.6; `abr-irrs` `docs/GOVERNING_DECLARATIONS.md` (W-1) |
| 7 Derived Invariants | `kernel_reference.md` §13–§15, §18; `src/derived_invariants.rs`; `validation_record.md` |
| 8 Proof and Derivation | `kernel_reference.md` §6, §15, §17; `tests/v9_conformance.rs`; `docs/process/kernel_self_consistency_test.md` |
| 9 Provenance and Admission | `abr-kernel` `README.md`, `v9_specification.md`, `src/admission.rs`, `src/provenance.rs`; `abr-irrs` `README.md`, `docs/GOVERNING_DECLARATIONS.md` |
| 10 Verification and Validation | `kernel_reference.md` §16–§17, §20; `role_separation_and_operator_application.md`; `triad-constraint-methods.md`; `docs/process/*` |
| 11 Applications | To be selected by Origin |

---

## Non-canonical material in this curriculum

- The historical account in Module 1 §1.2 comes from the general history of mathematics, not from the canonical sources. It needs independent citation checks before publication (open question OQ-7).
- The kernel `README.md` cites `arXiv:2601.22389`. This draft does not cite or rely on it (open question OQ-10).
- No numerical value appears in this draft except values quoted, with their source, from the canonical record.

---

*Bounded over D. No claim beyond D.*
