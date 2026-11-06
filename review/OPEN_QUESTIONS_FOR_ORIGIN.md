# Open Questions for Origin Review

**ABR Mathematical Foundations, first draft.** Prepared by the Generator on 2026-11-05.

These questions came up while drafting. Each needs an Origin decision. None has been resolved by the Generator. Where a choice had to be made to produce a draft at all, the draft's current choice is stated, so Origin can confirm or replace it.

---

## Source and authority

**OQ-1 — "Approved conceptual material" not located.** The task asks for Module 1 to use "the approved conceptual material". No document with that designation was found. The Generator searched the Metatron_Dynamics workspace for "Observation and Abstraction" and "Invitation to Mathematicians" in `.md`, `.txt`, and `.html` files. *Draft's choice:* the conceptual core of Module 1 is taken entirely from canonical text (`kernel_reference.md` §1–§2, `abr_operators_plain.md`, `role_separation_and_operator_application.md`), and the historical and explanatory text is new. *Needed:* the approved material, or confirmation that the canonical text is the approved base.

**OQ-2 — The "Executor" role is not a Kernel V9 role.** The task names Origin, Generator, Executor, and Verifier. Kernel V9 declares three roles (`kernel_reference.md` §3), and it assigns execution of the applicable kernel to the Generator (§5, Step 2). *Draft's choice:* the curriculum describes AI as implementing and executing authorized calculations within the Generator role, and does not name an Executor role. Naming one would add a role the canonical sources do not hold. *Needed:* should "Executor" appear? If so, it must first be declared in the canonical source, not in this curriculum.

**OQ-3 — Wording of the central distinction.** Three wordings are in play:
- the task: "relation is the mathematical primitive";
- Kernel V9: "the foundational observable primitive is a directed relation declared from observable information";
- IRRS GD-1: "Relation is the observable primitive".

*Draft's choice:* Module 1 §1.5 uses the task's wording as its headline and quotes both canonical wordings beneath it. *Needed:* which wording should govern the curriculum's own sentences?

**OQ-4 — Content-class labels.** The draft introduces four labels: [CANONICAL], [ESTABLISHED], [EXPLANATORY], and [OPEN] (`README.md`). These are editorial conventions for this curriculum, not kernel classifications. They sit alongside, and do not replace, the kernel's support classifications (Observed, Inferred, Derived, Downstream projection) and its five-way correspondence classification (§16.3). *Needed:* approve, rename, or replace them.

## Mathematical framing

**OQ-5 — How to present "a primitive not previously existing in mathematics".** `kernel_reference.md` §1 states that the framework "introduces one primitive that has not previously existed in mathematics". Mathematicians reading the curriculum may bring prior relation-first traditions to the comparison, such as the nineteenth-century logic of relations (De Morgan, Peirce) or category theory, where morphisms are primary. *Draft's choice:* Module 1 quotes the canonical contrast ("standard mathematics constructs relations from objects") but does not repeat the "not previously existed" sentence, and makes no comparison with other traditions. *Needed:* should the curriculum state the canonical claim in full? Should a later module address how the primitive, a relation *declared from observable information*, differs from relation-first formalisms? Any such comparison would be [EXPLANATORY] and would need Verifier review.

**OQ-6 — The choice of primitive is stated, not derived.** The canonical record says the primitive "is not derived. It is the starting point". It also says the three characteristics "jointly determine" it (`kernel_reference.md` §1). *Draft's choice:* Module 1 §1.7 says the module reports the starting point and does not prove it. *Needed:* confirm that this framing is right, so that mathematicians do not read "determine" as a claim of derivation.

## Explanatory content

**OQ-7 — Historical citations.** The history in Module 1 §1.2 (Babylonian place value and Plimpton 322; Euclid; al-Khwārizmī; Descartes; Newton and Leibniz; Lobachevsky, Bolyai, and Riemann; Einstein; Cantor and Hilbert; Maxwell; Noether; Wigner 1960) comes from the general history of mathematics. The Generator wrote it, and it is not in the canonical sources. *Needed:* should a bibliography with primary and secondary citations be added, and should a Verifier check every date and attribution before publication?

**OQ-8 — Audience and prerequisites.** *Draft's assumption:* readers are comfortable with proof, with set-theoretic notation, and with basic linear algebra and calculus; no physics beyond the undergraduate level is required. *Needed:* confirm or set the intended level, and whether engineers are a primary audience (the GitHub repository description names them).

**OQ-9 — Physics-domain content.** The canonical sources include physics-domain material: the classical/quantum resolving-power finding, Part I invariants confirmed against PDG and NIST records, and Part II statements I-T, I-S, I-E, and I-RE. *Draft's choice:* this material is deferred to Module 7, with each item's canonical status carried unchanged. *Needed:* where and how much of it to teach, and whether "framework-level finding" items (for example OC-CQ-1) belong in an introductory curriculum.

**OQ-10 — External reference.** The kernel `README.md` cites `arXiv:2601.22389`. *Draft's choice:* it is not cited. *Needed:* should it be cited, and with what status relative to Kernel V9?

## Process and publication

**OQ-11 — The repository is PUBLIC.** `abr-mathematical-foundations` is a public GitHub repository. It has no commits yet. Under `triad-constraint-methods.md`, nothing becomes public without the publication gate: O1 and O2 discharged by Origin, plus the seven methods, including the Adversarial Gate. *Draft's choice:* nothing has been committed or pushed. *Needed:* confirm that the gate applies before the first push, and whether the repository should be private until it passes.

**OQ-12 — License.** No license file has been added. The kernel repository uses a custom ("Other") license. The exact-compute benchmark in the same organisation uses Apache-2.0. *Needed:* a license choice, which is Origin's.

**OQ-13 — Format.** *Draft's choice:* GitHub-flavoured Markdown, with mathematics in Unicode inline text, not LaTeX, which matches the canonical documents. *Needed:* confirm, or choose LaTeX/MathJax or a book toolchain for later modules.

**OQ-14 — Module order and scope.** The outline in `CURRICULUM.md` (Modules 2–11, Appendices A–D) is a Generator proposal. *Needed:* approve, reorder, or cut it. Module 11 especially needs Origin's choice of applications.

---

## Observations returned without action

- **Reference clone state.** The local reference clone of `abr-irrs` is checked out on branch `exact-representation` (`2d926f0`), not on `main`. The Generator read `origin/main` (`c7410c5`) without changing the checkout. Nothing was modified.
- **IRRS kernel labelling.** An earlier record held that `abr-irrs/irrs-kernel` described itself as "Kernel V8 canonical". At `c7410c5`, the IRRS `README.md` and `docs/GOVERNING_DECLARATIONS.md` state that `irrs-kernel` carries the canonical V9 files byte for byte. The Generator did not run `irrs-eval verify-kernel` to confirm this.
