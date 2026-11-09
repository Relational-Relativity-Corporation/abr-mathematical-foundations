# ABR Mathematical Foundations

**An introduction to observable relational mathematics for mathematicians, physicists, and engineers.**

Relational Relativity Corporation · Metatron Dynamics, Inc.

> **Status: FIRST DRAFT — awaiting Origin review.** Nothing in this repository has been through Verifier review or the publication gate (`abr-kernel` `docs/kernel/triad-constraint-methods.md`). Do not cite it as a canonical source.

---

## What this repository is

This repository is a curriculum. It teaches the mathematics of the ABR kernel: directed relations declared from observable information, the operators that act on them, the provenance discipline that governs their use, and the way calculated results are compared with observation.

It is a teaching layer, **not a mathematical authority.**

| Authority | Repository | Pinned commit |
|---|---|---|
| Mathematics: operator forms, invariants, admissibility, roles | [`abr-kernel`](https://github.com/Relational-Relativity-Corporation/abr-kernel), **Kernel V9** | `1c1ef70` |

Where anything in this curriculum differs from that source, **the canonical source governs**, and the difference is a finding to return to Origin. This repository introduces no operator definition, no mathematical primitive, no topology rule, and no admissibility condition of its own. See [`references/CANONICAL_SOURCES.md`](references/CANONICAL_SOURCES.md).

The mathematics taught here is implemented. Kernel V9 is a Rust reference implementation with a conformance test suite. Readers are invited to build it and check every statement against the code.

---

## How to read the material: content classes

Every section of every module carries one of four labels. The labels keep what the kernel declares separate from what this curriculum explains.

| Label | Meaning | Who may change it |
|---|---|---|
| **[CANONICAL]** | A definition, rule, or form stated in Kernel V9. Quoted or closely restated, with a section reference. | No one in this repository. Changes happen in the canonical repository, by Origin. |
| **[ESTABLISHED]** | A result the canonical sources classify as established: an implemented and tested behaviour, a CLOSED finding, or a Part I invariant the canonical record classifies as confirmed. Reported here *as the canonical record states it*; this repository does not re-establish it. | No one in this repository. |
| **[EXPLANATORY]** | Teaching material written for this curriculum: motivation, history, analogy, commentary. It carries **no mathematical authority**. If it appears to add a rule, it is wrong. | Origin, after review. |
| **[OPEN]** | An open condition, HOLD, or undeclared item, either recorded in the canonical sources or raised for Origin review by this curriculum. | Closed only by derivation or Origin declaration in the canonical source, never in this repository. |

Proofs are not invented here. Where a module needs a derivation the canonical sources do not hold, the module says so and marks it **[OPEN]**.

---

## Contents

| Path | What it holds | Status |
|---|---|---|
| [`CURRICULUM.md`](CURRICULUM.md) | Table of contents and outline of every module | Draft |
| [`preface/00_an_invitation.md`](preface/00_an_invitation.md) | Preface: *An Invitation to Mathematicians and Physicists* | Draft |
| [`modules/01_observation_and_abstraction.md`](modules/01_observation_and_abstraction.md) | Module 1: *Observation and Abstraction* | Draft |
| [`lessons/01_stopping_distance/`](lessons/01_stopping_distance/README.md) | Lesson 1: a basic Newtonian stopping-distance calculation beside an expanded relational field (canonical A → B → R), compared with a simulated reference braking event; four interdependent-condition diamonds | Draft (illustrative values), awaiting Origin and Verifier |
| [`references/CANONICAL_SOURCES.md`](references/CANONICAL_SOURCES.md) | Pinned canonical sources and the sections each module draws on | Draft |
| [`review/OPEN_QUESTIONS_FOR_ORIGIN.md`](review/OPEN_QUESTIONS_FOR_ORIGIN.md) | Questions raised while drafting, returned to Origin | Draft |

Modules 2 onward exist only as outlines in `CURRICULUM.md`.

---

## Roles

This curriculum follows the role separation declared in Kernel V9 (`kernel_reference.md` §3; `role_separation_and_operator_application.md`):

- **Origin** (human) declares the domain D, the measurement mapping M, the relations and their provenance, and disposes of findings. Origin approves the content of this curriculum.
- **Generator** proposes candidate structure within what Origin declared. This draft was prepared by an AI Generator.
- **Verifier** detects divergence from the declared criteria, from a frame distinct from the Generator, and returns located findings.

---

## Reproducing the canonical baseline

```
git clone https://github.com/Relational-Relativity-Corporation/abr-kernel.git
cd abr-kernel
git checkout 1c1ef70
cargo test
```

On 2026-11-05 the Generator ran this at `1c1ef70` (Windows, local toolchain): 61 library tests, 3 `provenance_demo` tests, 12 V9 admission tests, 35 V9 conformance tests, and 1 doctest passed, with 0 failed and 0 ignored. That is a statement about implementation conformance only. It makes no claim about correspondence with any observed system (`role_separation_and_operator_application.md`, Verification and Validation Protocol).

---

*Bounded over D. No claim beyond D.*
