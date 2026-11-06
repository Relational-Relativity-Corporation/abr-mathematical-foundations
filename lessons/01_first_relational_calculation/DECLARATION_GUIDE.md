# Lesson 1 — Declaration Guide

> **Revision 2:** this guide covers the supporting two-locus exercise ([`SUPPORTING_PRIMARY_PAIR.md`](SUPPORTING_PRIMARY_PAIR.md)). Run its program with `cargo run --bin lesson01 -- <file>`. For the principal exercise see [`02_OBSERVABLE_SET.md`](02_OBSERVABLE_SET.md).

How to write a Lesson 1 declaration file, and where each line goes in the canonical code.

A declaration file is plain text, one `key = value` per line, with comments beginning `#`. An empty value counts as **not declared**. Nothing is filled in for it, and the program reports what the gap leaves OPEN. Start from `declarations/my_measurement_TEMPLATE.txt`.

You are Origin for your declaration. The program places your text in the canonical records exactly as written. It does not check whether your statements are accurate descriptions of what you did. That is your responsibility, and an independent Verifier's to examine.

---

## 1. Observations and their record

| Key | What to write | Canonical destination |
|---|---|---|
| `observation_status` | `measured` for your own readings; `illustrative` only for supplied demonstration values | V9 `ObservableDeclaration.origin_kind`: `Measured`, or `SyntheticFixture` for illustrative |
| `instrument` | The instrument the readings come from, e.g. "30 cm steel ruler, 1 mm graduations, my desk" | `ExternalRecord.source` for both readings |
| `L1_record_entry`, `L2_record_entry` | Where each reading is written, e.g. "notebook p. 4, line 1" | `ExternalRecord.entry` |
| `measurement_conditions` | How you read the marks, e.g. "ruler flat on the strip, read from directly above, room light" | `ObservableDeclaration.declared_conditions` |
| `unit` | `mm` | Unit of the observable, of M, and of the domain |
| `L1`, `L2` | Your two readings on the printed scale, in `unit` | Raw input to the IRRS gate; after admission, `ObservableDeclaration.reported_value` |
| `L1_description`, `L2_description` | What each mark is | Shown in the report (description only) |

V9 admission requires every observation to name an external record and its measurement conditions. Without them the result is OPEN: `NoExternalTerminus` or `MissingMeasurementConditions` (`src/admission.rs`). Demonstration 4 shows this.

## 2. Domain D: the instrument's scale

| Key | What to write | Canonical destination |
|---|---|---|
| `scale_floor`, `scale_ceiling` | The ends of your instrument's printed scale, e.g. `0` and `300` | IRRS `ObservableExtent` (region extent, inclusive); V9 `DeclaredDomain` floor and ceiling |
| `scale_basis` | What you observed that establishes this extent, e.g. "printed scale runs 0–300 mm" | V9 `DeclaredDomain.basis` |

The extent comes from the instrument, not from the mathematics (M Regional Instantiation Prerequisite, `role_separation_and_operator_application.md`). A reading outside it is recorded but not admitted. The IRRS gate returns OPEN.

## 3. Measurement mapping M

| Key | What to write | Canonical destination |
|---|---|---|
| `mapping_preserves` | What the mapping keeps, e.g. "each mark's position as read on the scale, in mm" | IRRS `CrossFamilyOrientation.preserved`; V9 `MMappingDeclaration.basis` |
| `mapping_discards` | What it drops, e.g. "mark width, colour, reading uncertainty" | IRRS `CrossFamilyOrientation.discarded`; V9 `MMappingDeclaration.basis` |

Lesson 1 declares M of kind **Identity**: "the reported value enters D unchanged in the declared unit" (`src/provenance.rs`, `MMappingKind::Identity`). Kernel V9 admits an identity mapping before Δ ("Admissible: uniform shift, declared unit scale", `kernel_reference.md` §8). The IRRS gate executes it. Your program never computes it.

## 4. Loci and the directed relation (topology)

| Key | What to write | Canonical destination |
|---|---|---|
| `relation` | `L1 -> L2` or `L2 -> L1` | V9 `ObservableRelationDeclaration` source and target loci; the edge `(s, t)` in `DeclaredRelations` |
| `direction_basis` | What you observed that establishes the direction | V9 `ObservableRelationDeclaration.direction_basis` |
| `continuation_basis` | Why both loci belong to one continuous structure, e.g. "both marks lie on one ruler scale" | Relation class `Continuation` ("declared from observable or device geometry", `src/operators.rs`), with its basis |

**Topology is not array order.** The loci are listed as `[locus.L1, locus.L2]` in that fixed order, and the order is only a label. The relation, and therefore the sign of the result, comes from your `relation` line alone. Swap it to `L2 -> L1` and the result changes sign while the list stays the same.

**Why the relation is admissible here.** Kernel V9 requires every declared relation to have one direction, traceable to an observable through M (`operators_notation_and_constraint.md`, Relational direction). In this lesson, both loci are marks you read on one instrument scale. The direction is the one you state in `direction_basis` from what you observed. The supplied example uses the direction in which the printed scale increases from L1 to L2. The reverse direction is not declared, and nothing supplies it.

**One recorded observation for the relation.** The V9 relation record names a single `observable_source`. This lesson names the target-locus reading, and lists both readings in the direction and class bases. This is a lesson-level choice, returned to Origin as open question Q-3 in `VERIFICATION_REPORT.md`.

## 5. Kernel selection

| Key | What to write | Canonical destination |
|---|---|---|
| `kernel_selection` | `primary` for two loci and one relation | V9 `RelationalDeclaration.kernel_selection` |
| `kernel_selection_basis` | Why, e.g. "two loci, one relation; no interior locus" | V9 `RelationalDeclaration.selection_basis` |

The full A → B → R kernel needs an interior locus, declared by M; "the kernel does not compute it" (`kernel_reference.md` §10). Two loci have none, so the Primary kernel applies, with Δ and then Σ. ρ_base and χ₀ are not declared in Lesson 1, so Σ is reported as **not evaluated**, never as 0.0. Writing `abr` instead is Demonstration 3: the canonical code declines.

## 6. Comparison (after the calculation)

These keys record your independent comparison observation. The program prints them and computes nothing from them. See [`EXPERIMENT.md`](EXPERIMENT.md), Part C.

| Key | Meaning |
|---|---|
| `comparison_separation` | The separation you measured directly, e.g. with the ruler's zero placed on L1 |
| `comparison_uncertainty` | The reading uncertainty you declare, in `unit` |
| `comparison_order` | `L2_further_along_scale` or `L1_further_along_scale` |
| `comparison_method` | How you made the comparison observation |
| `comparison_outcome` | `agrees` or `disagrees` (your judgement); `not_compared` if you did not compare |

---

## What the program does with your declaration

1. **IRRS observation admission.** `relational_gate::RegionBuilder::establish` establishes the region from your scale and from a recorded reading. Each reading is then submitted with `relational_gate::AdmissionGate::submit`. The gate executes M itself and checks the declared extent. Only a resulting `AdmittedObservation<f64>`, a type no other code can construct, carries a value forward (`abr-irrs` `relational-gate/src/gate.rs`, `admitted.rs`).
2. **Canonical records.** The admitted values and your statements fill the V9 `DeclarationRegistry` (`abr-kernel` `src/provenance.rs`).
3. **Canonical execution.** `abr_kernel::provenanced_exec::execute_primary` builds `DeclaredRelations` and `NodeField` through the canonical constructors and calls `abr_kernel::operators::operator_delta`, producing a calculation record.
4. **V9 provenance admission.** `abr_kernel::admission::admit` reconstructs the calculation independently from your declarations. It traces every input to its record and returns **ADMITTED**, **OPEN** or **REJECTED**.

*Bounded over D. No claim beyond D.*
