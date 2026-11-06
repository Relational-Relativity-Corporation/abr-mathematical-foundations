# Curriculum — Table of Contents

**ABR Mathematical Foundations.** First draft, awaiting Origin review.

Mathematical authority: Kernel V9 (`abr-kernel` `1c1ef70`) and IRRS (`abr-irrs` `c7410c5`). Content classes ([CANONICAL], [ESTABLISHED], [EXPLANATORY], [OPEN]) are defined in [`README.md`](README.md).

Only the Preface and Module 1 are drafted. Modules 2–11 are **outlines**: each lists its purpose, the canonical material it will teach, and what it must not do. An outline is not a claim that the material is complete, derived, or verified. Its content and order are proposals for Origin review.

---

## Front matter

| | Title | Status |
|---|---|---|
| Preface | [An Invitation to Mathematicians and Physicists](preface/00_an_invitation.md) | Draft |

## Lessons (executable)

| | Title | Status |
|---|---|---|
| Lesson 1 | [First Relational Calculation](lessons/01_first_relational_calculation/README.md): from a complete bounded observable set toward the Primary condition (revision 2) | Draft, awaiting Origin and Verifier |

## Part I — Foundations

| | Title | Status |
|---|---|---|
| Module 1 | [Observation and Abstraction](modules/01_observation_and_abstraction.md) | Draft |
| Module 2 | The Declared Domain: O → M → D | Outline |
| Module 3 | Directed Relations and Declared Topology | Outline |

## Part II — Formal Definitions and Operators

| | Title | Status |
|---|---|---|
| Module 4 | The Primary Operators: Δ, Σ, and ρ | Outline |
| Module 5 | The Full Operator Path: A → B → R | Outline |
| Module 6 | Persistence and Relational Evolution | Outline |
| Module 7 | Derived Invariants and Their Status | Outline |

## Part III — Proof, Provenance, and Verification

| | Title | Status |
|---|---|---|
| Module 8 | Proof and Derivation in a Bounded Domain | Outline |
| Module 9 | Provenance and Admission: RPPI and IRRS | Outline |
| Module 10 | Observational Verification and Validation | Outline |

## Part IV — Applications

| | Title | Status |
|---|---|---|
| Module 11 | Reproducible Applications | Outline |

## Appendices (planned)

| | Title | Status |
|---|---|---|
| A | Notation | Outline |
| B | Language discipline | Outline |
| C | Register of open conditions (mirrors `kernel_reference.md` §18) | Outline |
| D | Glossary | Outline |

---

## Module outlines

### Module 1 — Observation and Abstraction *(drafted)*

Where mathematics begins, and what abstraction achieved. The central distinction: relation is the mathematical primitive, and the primitive does not itself identify or establish any particular observable relationship. The responsibilities of the human Origin and of AI. See the [module file](modules/01_observation_and_abstraction.md).

### Module 2 — The Declared Domain: O → M → D *(outline)*

- **Purpose.** Give the formal setting every later module uses.
- **Canonical material.** D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }; M : O → D declared before any operator acts; a value in D is a numerical projection, not the observable; operator outputs are calculated results; the declared-information rule (not evaluated ≠ evaluated with result 0.0); admissibility as a provenance condition; declaration correspondence; support classifications (Observed, Inferred, Derived, Downstream projection); the observational resolution principle (ℛ_M, 𝓜_M). Sources: `kernel_reference.md` §1, §2, §6, §9; `operators_notation_and_constraint.md` (Admissibility, Domain, Declaration correspondence, Observational resolution principle).
- **Planned worked material.** Symbolic examples of declared and undeclared information; no numerical values except labelled illustrations.
- **Must not.** Supply a default M, a default parameter, or a value for an undeclared quantity.

### Module 3 — Directed Relations and Declared Topology *(outline)*

- **Purpose.** Teach how relations are declared, and which structures are inadmissible and why.
- **Canonical material.** The two kinds of declared relation (spatial edge; persistence relation); a single admissible direction; the distinctness axiom; inadmissibility of rings, tori, and periodic indices; edge provenance classes (Continuation, Coupling, Persistence); the "before A / before Δ" rule; the M Regional Instantiation Prerequisite (region, observable extent, regional orientation, regional admission rule). Sources: `operators_notation_and_constraint.md` (Relations, Relational direction, Relational evolution direction, Before A); `kernel_reference.md` §8; `role_separation_and_operator_application.md` Step 1.
- **Must not.** Add a topology rule, or present a topology as admissible on mathematical convenience.

### Module 4 — The Primary Operators: Δ, Σ, and ρ *(outline)*

- **Purpose.** Formal definitions of the primary kernel E_primary = Σ(Δ(x)).
- **Canonical material.** Δ(x)[e] = x[s] − x[t]; Σ over immediate neighbours (adj⁺, adj⁻); the node and edge forms of ρ, with ρ_base and χ₀ M-supplied and no kernel defaults; structural checks (relational isolation, declared edge-image admissibility, antisymmetric term). Rust functions: `operator_delta`, `operator_sigma`, `operator_e_primary`, `compute_rho_primary`. Sources: `operators_notation_and_constraint.md`; `kernel_reference.md` §10–§11; `src/operators.rs`.
- **[OPEN] items to carry unchanged.** A-3 (empty adjacency); ρ max-selection rule; ρ saturating form.
- **Must not.** Supply values for ρ_base or χ₀ beyond labelled illustrations, or describe Σ as a new operator (it is R at the Primary Region limit).

### Module 5 — The Full Operator Path: A → B → R *(outline)*

- **Purpose.** Formal definitions of the ABR kernel and of kernel selection.
- **Canonical material.** The full-operator admissibility condition (an interior locus t = target(e) = source(f), minimal s → t → u; structural, not numerical); A, B (immediate successors only), R (with cc[p] per declared component pair); E = R(B(A(x)), ρ(A(x))); "R is not optional"; the antisymmetric expression rule (R_anti ≠ 0 is a review signal); C as declared projection, not a kernel operator. Sources: `kernel_reference.md` §10–§11; `operators_notation_and_constraint.md`; `abr_operators_plain.md`; `src/operators.rs`.
- **Must not.** Describe B as transitive accumulation, or present any projection without what it preserves and discards.

### Module 6 — Persistence and Relational Evolution *(outline)*

- **Purpose.** Phase 2: how the operators act across a declared sequence of actual observations.
- **Canonical material.** The sequential observation requirement; the relational step as the system's own declared process step; the relational evolution direction declared from an observable property of the process (index order is not a direction declaration; temporal order is a projection); A_persistence = E_current − E_prior; B_persistence, R_persistence; `PersistenceState::from_prior`; no persistence at the first declared observation. Sources: `operators_notation_and_constraint.md` (Domain, Relational evolution direction, A); `kernel_reference.md` §8, §11.6; `role_separation_and_operator_application.md` Step 1–2. IRRS rule W-1 (whole field) from `abr-irrs` `docs/GOVERNING_DECLARATIONS.md`.
- **Must not.** Treat a model trajectory, a parameter sweep, or a repeated static configuration as an observed sequence.

### Module 7 — Derived Invariants and Their Status *(outline)*

- **Purpose.** Present what the operators produce, with each quantity's status stated exactly as the canonical record states it.
- **Canonical material.** Part I (classified confirmed in form): ι[v], ε[e], 𝟙[e], ε[e] at photon edges, τ[v], N[v], θ[e], Φ[v], Φ[S], J[v]; declared measurement references. Part II (open derivation, framework-level necessity statements): I-T, I-S, I-E, I-RE, joint necessity. Sources: `kernel_reference.md` §13–§15; `src/derived_invariants.rs`; `docs/kernel/validation_record.md`.
- **[OPEN] items to carry unchanged.** Every OC-* condition and HOLD in `kernel_reference.md` §18.
- **Must not.** Cite a Part II statement as a proven consequence of the operators, or restate a confirmation count without its canonical source.

### Module 8 — Proof and Derivation in a Bounded Domain *(outline)*

- **Purpose.** Teach what counts as a derivation in this framework, and collect the derivations the canonical sources hold.
- **Canonical material.** The Derived support classification; the separation of verification (conformance) from validation (correspondence); [MATH]-tagged tests as machine-checked statements of operator properties; movement from Part II to Part I only by derivation, "not by additional agreement" (`kernel_reference.md` §15). Sources: `kernel_reference.md` §6, §15, §17; `tests/v9_conformance.rs`; `docs/process/kernel_self_consistency_test.md`.
- **Must not.** Invent a proof. Where a module needs a derivation the canonical sources do not hold, it records an [OPEN] item for Origin.

### Module 9 — Provenance and Admission: RPPI and IRRS *(outline)*

- **Purpose.** How the canonical implementation enforces admissibility.
- **Canonical material.** The Relational Primitive Provenance Invariant; provenance label vs. declaration record vs. calculation record; RPPI failure modes (missing, fabricated, disconnected, misattributed); outcomes OPEN / REJECTED / FAILED / Divergent; `AdmittedObservation<T>` and the seven-step gate; IRRS governing declarations GD-1 to GD-8. Sources: `abr-kernel` `README.md`, `docs/kernel/v9_specification.md`, `src/admission.rs`, `src/provenance.rs`; `abr-irrs` `README.md`, `docs/GOVERNING_DECLARATIONS.md`.
- **Must not.** Describe the IRRS as having capabilities its README marks planned (OPEN), for example M2–M5 candidate alignment stages.

### Module 10 — Observational Verification and Validation *(outline)*

- **Purpose.** How calculated results are compared with observation, and how outcomes are classified.
- **Canonical material.** The four-stage protocol; the Verifier criterion (provenance traceability); legacy mathematical intrusion classes; the execution order (declare → execute unchanged → preserve → compare → return); the dual trace; the five-way correspondence classification (§16.3); "divergence is data"; no silent repair; the seven verification methods and the publication gate. Sources: `kernel_reference.md` §16–§17, §20; `role_separation_and_operator_application.md`; `docs/kernel/triad-constraint-methods.md`; `docs/process/*`.
- **Must not.** Collapse outcomes into PASS/FAIL, or present a conformance PASS as a correspondence claim.

### Module 11 — Reproducible Applications *(outline)*

- **Purpose.** Worked applications a reader can rebuild and check.
- **Selection.** Origin selects which applications are taught. Each needs a recorded declaration (D, M, topology, parameters), a pinned repository commit, and an outcome classified under §16.3. Candidate sources are the canonical illustrative examples (`abr-irrs` `examples/abr-simple`, `bin/provenance_demo.rs` in `abr-kernel`); application repositories are referenced, never modified.
- **Must not.** Present an illustrative or synthetic fixture as empirical evidence (V9 admission: a Simulated or SyntheticFixture terminus is not empirical evidence), or carry a declared value from one domain into another.

---

*Bounded over D. No claim beyond D.*
