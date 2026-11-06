# Design Specification — Newtonian Stopping Distance and Expanded Relational Calculation

**Lesson 1.** Written 2026-11-05 as a design only. Implemented the same day on Origin direction: replace the marble-ramp Lesson 1; values proposed by the Generator and labelled illustrative; "Run ABR using the IRRS as a guide, report your findings". The implementation is `src/` and the results are in `records/`. The D-SD decisions below are still Origin's. Where the implementation made a provisional choice, the choice is listed in §12.

Authority: Kernel V9 (`abr-kernel` `1c1ef70`) and IRRS (`abr-irrs` `c7410c5`). Content classes ([CANONICAL], [ESTABLISHED], [EXPLANATORY], [OPEN]) are those in the repository `README.md`. Items marked **D-SD-n** need an Origin decision before implementation.

This specification also covers the earlier Origin requirement *Mandatory Test — Interdependent Observable Conditions* (tread × moisture, and the analogous interactions), in §7.

---

## 1. Summary of the design

| Exercise | What runs | Mathematical authority | Status of the result |
|---|---|---|---|
| A. Basic | d = v·t + v²/(2a), by hand and in the program | Newtonian kinematics [EXPLANATORY]. **Not** the kernel | CALCULATED (Newtonian) |
| B. Expanded | Declared fields → IRRS evaluation and admission → canonical A (and B, ρ, R where admissible) → an **Origin-authorized** Newtonian calculation that consumes declared projections of admitted A outputs | Kernel for the operators. The stopping-distance formula is **not** kernel and needs authorization (D-SD-4) | CALCULATED, or OPEN if not authorized |
| C. Reference | A deterministic, fully specified simulated braking event | A declared simulator [EXPLANATORY]. V9: simulated, admitted only as simulated | SIMULATED. Never empirical |

**[SUPERSEDED — wrong; see §13.]** The design's central finding [CANONICAL basis]: **no canonical operator computes a stopping distance, and no canonical mapping exists from A, B, R, ρ or 𝟙 to a stopping distance.** The kernel's role is to admit the expanded observable information and to express its relations: directed differences such as slip velocity, relative air velocity, elevation drop, a change in friction between stations, and latencies. Turning those outputs into a distance takes a declared projection C, "Not a kernel operator" (`kernel_reference.md` §10 table), plus a Newtonian calculation that only Origin can authorize. The lesson will show exactly where the kernel stops and the authorized calculation begins.

---

## 2. Exercise A — Basic Newtonian calculation  [EXPLANATORY]

Inputs (given by Origin): v = 60 mph = 26.8224 m/s; t_r = 1.5 s; a = 6.0 m/s² constant; level road.

```
reaction distance   v·t_r      = 26.8224 × 1.5          = 40.2336 m
braking distance    v²/(2a)    = 719.44115 / 12.0       = 59.95343 m
stopping distance   d                                   = 100.18703 m  ≈ 100.2 m
```

**Assumptions the lesson will state:**
- The vehicle is a point mass moving in a straight line.
- t_r is a single fixed delay from the hazard appearing to full braking. In reality perception, decision, foot movement and brake pressure build-up are separate.
- Deceleration jumps instantly to a constant 6.0 m/s². This hides the friction coefficient, the road surface, moisture, tyre condition, the brake force history and ABS behaviour.
- There is no grade, no wind and no aerodynamic or rolling resistance.
- Mass does not appear. In this model it cancels, because deceleration is given directly.
- Speed is known exactly, and the hazard is detected the moment it appears.

**Limitation:** the result is exact for its inputs, but it says nothing about whether those inputs describe a real event. That has to be established by observation (Exercise C).

The program will print this by-hand arithmetic as a labelled Newtonian result and will never attribute it to the kernel.

---

## 3. Exercise B — Expanded observable information: domain, measurement and topology

### 3.1 How the twelve items are placed

The kernel (§12.2) requires a NodeField `data[c][i]`: **every declared component has a finite value at every declared node**, and A compares one component across the two ends of an edge. "Not evaluated" is never 0.0 (the declared-information rule). The twelve items therefore cannot all share one field. Mixing them would mean either filling missing values or subtracting quantities with different units. The design splits them into **three declared fields** (separate regions, each with its own M, topology and parameters), plus **trial conditions** and **calculation inputs**:

| # | Item | Where it enters | Note |
|---|---|---|---|
| 1 | Tyre–road friction coefficient | F2 (component μ per road station), and trial condition (§7) | Observed per station. No μ formula |
| 2 | Road surface and moisture | Trial condition (§7); its effect is observed through item 1 | |
| 3 | Tyre condition | Trial condition (§7); its effect is observed through item 1 | |
| 4 | Vehicle mass | Calculation input only (Newtonian) | A single value at a single locus has no relation. It is not a kernel input |
| 5 | Brake force and activation history | F3 (brake onset time); persistence over F1 (Phase 2, D-SD-6) | |
| 6 | Wind velocity and direction | F1 (Air locus, along-road component) | Direction → along-road component is a declared projection C (D-SD-3) |
| 7 | Road grade | F2 (elevation component per station) | Elevation drop comes from A; grade = drop / spacing is a projection C |
| 8 | Ambient illumination | Trial condition for F3 (§7.3) | Lux has no counterpart at the other F3 loci |
| 9 | Visibility and obstacle detection | F3 (Obstacle and Driver times); trial condition | |
| 10 | Driver attention and response history | F3 (Driver and Brake times); trial condition | |
| 11 | Vehicle position and velocity observations | F1 velocity; reference trajectory (Exercise C) | With position observed, stopping distance is *observed*, not calculated (§5.2) |
| 12 | Changing deceleration during braking | Persistence over F1 (Phase 2, D-SD-6) | |

### 3.2 Field F1 — Longitudinal velocity at one observation instant

- **D:** one component, along-road velocity, in m/s, over four loci.
- **M:** per-locus readings taken from the reference event (Exercise C) at a declared instant.
- **Loci** (physical entities):
  - **Air:** the wind velocity component along the road.
  - **Body:** the vehicle's centre-of-mass velocity.
  - **Tread:** the tyre's circumferential surface speed, ω·r.
  - **Road:** the contact surface, observed stationary. 0 m/s is a declared observed value, not a default.
- **Proposed relations** (direction basis **D-SD-2**):
  - Air → Body: the wind acts on the body.
  - Body → Tread → Road: the path along which the body's momentum is removed during braking.
  - Interior loci: Body and Tread, so the full A → B → R path applies.
- **What A gives** (formula from §10: A[e] = x[s] − x[t]):
  - A[Air→Body] = w − v (relative air velocity, sign as declared).
  - A[Body→Tread] = v − ωr: **longitudinal slip velocity**.
  - A[Tread→Road] = ωr − 0.

### 3.3 Field F2 — Road profile along the stopping path

- **D:** two components per station: elevation z (m) and observed friction coefficient μ (dimensionless). No component pairs are declared, because z − μ has no meaning; so no cc is required.
- **Loci:** road stations S0 … Sn at declared spacing along the path, including the station where braking begins.
- **Relations:** Sk → Sk+1, with the direction basis "the vehicle's observed direction of travel", the same basis as the Lesson 1 ramp. Interior loci: S1 … Sn−1.
- **What A gives:** per segment, the elevation drop z_k − z_{k+1} and the friction change μ_k − μ_{k+1}.

### 3.4 Field F3 — The perception–response chain

- **D:** one component, the time of a state change in seconds, measured from hazard onset.
- **Loci:**
  - Obstacle: time it first becomes visible.
  - Driver: time of detection or response onset.
  - Brake: time brake pressure begins.
  - Tread: time deceleration reaches its plateau.
- **Relations:** Obstacle → Driver → Brake → Tread, ordered by the observed signal path. Interior loci: Driver and Brake.
- **What A gives:** the negated perception latency, response latency and brake build-up latency. In the basic model these three are lumped together as t_r.

### 3.5 Operator parameters  [CANONICAL rule, OPEN values]

Each field needs ρ_base and χ₀ declared by M. χ₀ "carries the dimension of χ" (m/s in F1, m or dimensionless in F2, s in F3). The kernel "holds no value … and supplies no defaults" (§9). **None exists here (D-SD-5).** Without them:
- only **A** is evaluable (it has no parameters);
- ρ, B-through-R and 𝟙 are **not evaluated**;
- the program stops at the exact missing declaration (§5 Step 2), as Lesson 1 set 3 does.

If Origin authorizes labelled illustrative values, R and ρ describe those parameters, not the vehicle (as in Q-R2-3). Values are never carried over from another domain.

### 3.6 IRRS submission

- Each field is written in the `irrs_eval::Declaration` schema, plus a `lesson` block, as in Lesson 1 revision 2. It goes through `Evaluator::evaluate`, then gate admission (`relational-gate`), then `irrs_session::execute` (A, ρ, B, R, 𝟙 and RPPI re-execution).
- Readings come from the simulator, so input provenance is `simulated_input`, and V9 admits them "only as such" (`v9_specification.md`, Simulated observations).
- **Known gap [OPEN, D-SD-6]:** `irrs-session` at `c7410c5` executes the spatial path only, with no persistence. `irrs-eval` *evaluates* a persistence declaration (`PersistenceDeclaration`, `persistence_path` A_p → B_p → R_p), but executing it would call the kernel's `operator_e_v5` directly with `PersistenceState::from_prior`. That is canonical code, but an execution pathway the IRRS session does not itself provide.
- Rings (Q-R2-4): none of the proposed topologies closes, and §7's condition diamond is acyclic.

---

## 4. Canonical operators: applicability and output

| Operator | F1 | F2 | F3 | Requires | Output's relation to stopping distance |
|---|---|---|---|---|---|
| A | yes | yes | yes | admitted field and topology | Directed differences: slip, relative air velocity, elevation drop, μ change, latencies. **Becomes a calculation input only through a declared projection C (D-SD-3)** |
| ρ | if D-SD-5 | if D-SD-5 | if D-SD-5 | ρ_base, χ₀ | **None canonical.** OPEN |
| B | if D-SD-5 | if D-SD-5 | if D-SD-5 | interior locus; ρ | **None canonical.** OPEN. Immediate successors only, not accumulation along a path |
| R | if D-SD-5 | if D-SD-5 | if D-SD-5 | B, ρ, (cc: none) | **None canonical.** OPEN |
| 𝟙 | if D-SD-5 | if D-SD-5 | if D-SD-5 | R − B and tolerance | Review signal only (§11.4) |
| Δ, Σ (primary) | not selected | not selected | not selected | — | Interior loci exist, so ABR is selected (§10 kernel selection) |
| A_p, B_p, R_p | if D-SD-6 | — | — | a declared observation sequence; the simulator declared as a transducer (§8); ρ_persistence | Changes in E across steps: the *relational* record of changing deceleration. **No canonical mapping to distance.** OPEN |

**Not used:** derived invariants (τ, ι, θ and the rest). Their domains are particle-scale and do not apply to this D.

---

## 5. The exact relationship between kernel outputs and stopping distance

### 5.1 What the canonical mathematics establishes

Only A outputs map to physical inputs of a stopping-distance calculation, and only through projections C that Origin declares. Each projection states what it preserves and what it discards (§5 Step 2):

| Calculation input | Admitted source | Proposed projection C (D-SD-3) | Preserves | Discards |
|---|---|---|---|---|
| Relative air velocity | A[Air→Body] in F1 | identity (already along-road) | the directed difference | crosswind component; spatial variation |
| Grade per segment | A[Sk→Sk+1].z in F2 | drop ÷ declared horizontal spacing | the directed elevation difference | curvature within a segment |
| Friction per segment | the μ_k observed values (node values, not A) | none: a node reading | the observed μ | its variation within a segment |
| Perception, response and build-up times | A on the F3 edges | sign reversal | the latencies | their variability between trials |
| Slip velocity | A[Body→Tread] in F1 | none (reported, not used in the basic formula) | — | — |

### 5.2 What it does not establish  [OPEN]

- Any stopping-distance formula. The expanded formula in §6 is Newtonian and needs Origin authorization (**D-SD-4**). It is not kernel.
- Any use of B, R, ρ, 𝟙 or A_p in a distance. There is no canonical mapping, and the lesson must not invent one.
- **Observed versus calculated.** If item 11 (position observations) is admitted, stopping distance is *observed*: x(stop) − x(hazard onset), a "Downstream projection … by a declared reduction" (§6). The lesson therefore separates two questions:
  - *predictive:* inputs available before the event → a calculated distance;
  - *reconstructive:* the observed trajectory → the observed distance.

  Only the predictive calculation can be compared with the reference.

---

## 6. Expanded calculation  [EXPLANATORY, needs D-SD-4]

Proposed for authorization, with its inputs drawn only from §5.1 projections and declared calculation inputs:

```
d_exp = v0·(t_perc + t_resp) + d_buildup + Σ_segments d_k
segment k:  constant deceleration a_k = g·(μ_k·cosθ_k + sinθ_k) + F_aero(v, w)/m  (sign by direction)
            integrated piecewise until v = 0
```

**Its physical basis** is Newton's second law with Coulomb friction, gravity along the grade and aerodynamic drag. It brings in g, a drag coefficient, frontal area and air density. These are **imported physical constants and vehicle parameters**, which the no-imported-values rule forbids unless Origin declares them (**D-SD-7**). If D-SD-4 is not authorized, Exercise B reports the admitted relational outputs, and the expanded distance is **OPEN**.

---

## 7. Interdependent observable conditions (the Mandatory Test)

### 7.1 Design

Four controlled trials. Vehicle, v0, driver response, grade and wind are held at the baseline.

| Trial | Tread | Moisture |
|---|---|---|
| GD | good | dry |
| WD | worn | dry |
| GW | good | wet |
| WW | worn | wet |

**No interaction formula.** The reference simulator takes **one declared μ value per trial**, a table of four entries rather than a function μ(tread, water). Any interaction is therefore in the declared observations, not in an imposed formula. In the reference data the combined effect is neither assumed to be the sum of the two separate effects nor forced away from it.

### 7.2 Relational expression (proposed; admissibility is D-SD-8)

- **Field F4:** loci GD, WD, GW and WW. Components: the observed stopping distance (m) and the observed μ for each trial.
- **Relations:**
  - GD → WD and GW → WW: the tread change, at fixed moisture.
  - GD → GW and WD → WW: the moisture change, at fixed tread.
  - The result is an acyclic diamond with interior loci WD and GW.
- **What A gives:**
  - A[GD→WD] = the tread effect on a dry road.
  - A[GW→WW] = the tread effect on a wet road.
- **The teaching result:** the lesson places these two A values side by side. If they differ, the significance of tread depends on moisture. This is shown by comparing two admitted relational outputs, not by any formula. Whether to state the comparison as a further number (their difference) is a projection C for Origin.
- **[OPEN] admissibility:** these relations link *separate trials*, not an observed passage or signal path. Whether a controlled-condition contrast is an admissible declared relation is for Origin to decide (D-SD-8). Until then, F4 runs only as a labelled exercise, or does not run.

### 7.3 Same structure for the other interactions

| Interaction | Factor 1 × Factor 2 | Observed per trial | Field |
|---|---|---|---|
| Visibility × driver response | illumination (day / dusk) × attention (attentive / distracted) | detection time, response time | F3 per trial, plus a condition diamond |
| Wind × vehicle motion | wind (calm / head) × speed (two declared v0) | relative air velocity, stopping distance | F1 per trial, plus a diamond |
| Grade × braking | grade (level / downgrade) × surface (dry / wet) | segment deceleration, stopping distance | F2 per trial, plus a diamond |

Each one uses one declared reference value per trial and no interaction formula.

---

## 8. Exercise C — Reference simulated event

### 8.1 Simulator specification (proposed; values D-SD-7)

- **Model:** a deterministic, fixed-step (1 ms), single-track longitudinal model. It is written in the lesson crate, labelled [EXPLANATORY] simulator, and **never attributed to the kernel**. Inputs:
  - hazard onset at t = 0;
  - declared perception, response and brake build-up delays for each trial;
  - a brake pressure ramp;
  - a tyre force from a declared μ–slip table for each trial (tabulated, not a formula);
  - a grade profile and wind;
  - aerodynamic and rolling resistance.
- **Outputs:**
  - a time series of x, v, ωr and deceleration;
  - the F1, F2 and F3 readings at declared instants;
  - the reference stopping distance x(v = 0) − x(0).
- **Provenance:** every reading is `simulated_input`, and the report states "SIMULATED — not empirical evidence".
- **§8 boundary:** the time series is a model-generated trajectory. It becomes a Phase 2 observation sequence only if Origin declares the simulator as "a transducer from observable inputs" (D-SD-6). Otherwise only single-instant spatial fields are used.

### 8.2 Scenarios

1. **Baseline.** Exactly Exercise A: dry, level, no wind, a single 1.5 s delay, a constant 6.0 m/s² step. This checks the simulator's conformance: it must reproduce 100.187 m to within the integration step. It is not a check of physical accuracy.
2. **Expanded.** At least one non-baseline trial from §7, for example worn and wet on a 3 % downgrade with a headwind, at dusk, with a distracted driver. All values are D-SD-7.

### 8.3 Guard against manufactured improvement

If the expanded calculation (§6) and the simulator use the same physics, they will agree **by construction**, and that agreement shows nothing. The design therefore proposes (**D-SD-9**) that the simulator contains effects the §6 calculation does not model:
- a gradual brake pressure ramp rather than a step;
- a μ–slip curve rather than a single μ;
- μ varying along the path, finer than the declared stations;
- speed-dependent drag within each segment.

The residual is then reported as it comes out. Whatever the outcome, the report states that agreement with a simulator is agreement with a model that Origin declared, not with the world.

### 8.4 Report format (side by side)

| | Basic (Exercise A) | Expanded (Exercise B) | Reference (simulated) |
|---|---|---|---|
| Stopping distance | 100.187 m | value or OPEN | value |
| Difference from reference | d_A − d_ref | d_B − d_ref or OPEN | — |
| Inputs used | v, t_r, a | admitted A outputs via C, plus declared inputs | full simulator declaration |
| Relationships that materially changed the result | — | listed by perturbation: each projected input reverted to its baseline value, one at a time, with the change in d_B reported | — |
| Unmeasured or unresolved | — | every OPEN item in §§4–6 | — |

"Materially affected" is decided **by perturbation of the authorized calculation's inputs**, not by any kernel output. A threshold for "material" is D-SD-10.

---

## 9. Files (as implemented)

```
lessons/01_stopping_distance/
  DESIGN_SPECIFICATION.md        this file
  README.md, 01_…05_*.md         student documents
  VERIFICATION_REPORT.md         Generator self-audit and findings
  declarations/simulator_illustrative.json   every simulator and calculation value (illustrative)
  src/main.rs                    orchestration and the side-by-side report
  src/sim.rs                     Exercise C reference simulator   [EXPLANATORY]
  src/calc.rs                    Exercise A and the expanded calculation  [EXPLANATORY]
  src/fields.rs                  field declarations (irrs-eval schema)
  src/pipeline.rs                IRRS evaluation → gate → irrs_session::execute (canonical calls only)
  records/                       LESSON1_REPORT.txt, expanded_event/, diamonds/
```

The marble-ramp Lesson 1 was removed on Origin instruction. It is recoverable from commit `5ebe041`.

---

## 10. Decisions returned to Origin

| ID | Decision |
|---|---|
| D-SD-1 | Placement: this lesson's crate as a new bin, or a separate crate under Lesson 1 or a new Lesson 2? The prior instruction limited work to Lesson 1. The title of this one says Lesson 1 |
| D-SD-2 | The direction basis for F1 (momentum-removal path; air acting on the body) and F3 (signal path). Are these physically justified declarations? |
| D-SD-3 | Authorize the projections C in §5.1 (wind direction → along-road component; drop ÷ spacing; sign reversal of the latencies) |
| D-SD-4 | Authorize the expanded Newtonian formula of §6 as a non-kernel calculation, or leave the expanded distance OPEN |
| D-SD-5 | ρ_base and χ₀ for each field: declare them, authorize labelled illustrative values, or evaluate A only |
| D-SD-6 | Phase 2: declare the simulator as a transducer (§8) so that persistence over the braking sequence is admissible, executing through kernel `operator_e_v5` directly, since `irrs-session` has no persistence path. Or no Phase 2 |
| D-SD-7 | All simulator and calculation values: μ per trial and its μ–slip table, delays, grade, wind, mass, C_d·A, air density, g. Origin-declared, or Generator-proposed with an illustrative label. None may be imported from another repository or from the literature without declaration |
| D-SD-8 | Is a relation between controlled trials (F4 diamond) an admissible declared relation? |
| D-SD-9 | The simulator has strictly more physics than §6, to avoid agreement by construction |
| D-SD-10 | The threshold for "materially affected" |
| D-SD-11 | Tension with the revision-2 instruction ("not … a comparison with legacy mathematical approaches"). This exercise sets a Newtonian calculation beside the relational one. The design presents it as a basic calculation that is then *extended*, not as a contest. Please confirm |

## 11. Questions for the Verifier

- Confirm that no row of §4 or §5 attributes a distance mapping to a kernel operator.
- Confirm that the three-field split (§3.1) respects NodeField completeness and the declared-information rule.
- Confirm that the §8.3 guard is enough to prevent a manufactured improvement.

*Bounded over D. No claim beyond D.*

---

## 12. Implementation notes (provisional choices, all subject to the D-SD decisions)

- **Values (D-SD-7).** All are Generator-proposed and illustrative, in `declarations/simulator_illustrative.json`. The tread × moisture friction and the visibility × attention perception delays are tables, so any interaction in those two diamonds is the one the Generator wrote in. The wind × speed and grade × surface interactions come from the simulator's declared mechanics (drag law; gravity along the grade).
- **ρ_base = 0.5 for every field; χ₀ per field.** Velocity 1.0 m/s, elevation 0.5 m, friction 0.05, time 0.25 s, distance 5.0 m. All are illustrative (Origin: "Run ABR using the IRRS as a guide"). IRRS reports each one as OPEN.
- **F2 split.** F2 became two single-component fields, elevation and friction, because no component pair (z, μ) is meaningful.
- **F1 at two instants.** F1 is declared at hazard onset and at full braking. These are two separate spatial declarations, not a Phase 2 sequence (D-SD-6 not taken).
- **Integration step.** 0.1 ms rather than the 1 ms in §8.1, so that the baseline check matches the hand calculation to 4 decimal places.
- **Expanded calculation (D-SD-4).** Executed and reported as "pending Origin authorization", never as a kernel result.
- **"Material" threshold (D-SD-10).** 1 m, provisional.
- **Condition diamonds (D-SD-8).** Executed as declared relations of class `coupling`. IRRS raised no violation for them. Their admissibility is still Origin's decision.

---

## 13. Correction (Origin, 2026-11-05)

§1, §4 and §5 above state that "no canonical operator computes a stopping distance". **That is wrong.** The operators calculate the directed difference between elements of a system on declared relations. The stopping distance is the difference between two observed states of the vehicle.

The design left those states out of the field. They are now declared as **F5, the vehicle's observed states**:
- loci: Onset, ResponseStart, BrakeOnset, FullBraking, Standstill;
- components: position, and separately speed;
- relations: the chain in the observed order of the event, plus Onset → Standstill.

The kernel then calculates:
- **stopping distance:** −A[Onset → Standstill];
- each phase distance: −A on the chain;
- **reaction distance:** −B[Onset → ResponseStart];
- **braking distance:** −B[BrakeOnset → FullBraking].

The expanded Newtonian formula (§6) remains a *prediction* from the condition fields. It is compared with the relational calculation from the observed states.

**D-SD-12 (new).** Confirm the declared Onset → Standstill relation (the stopping event as a whole).

## 14. Verifier revision (V-01 to V-06, 2026-11-05)

See `VERIFICATION_REPORT.md` §2 for the dispositions:
- No authorized execution on OPEN; diagnostics are labelled.
- The B outputs are relabelled by the states they span.
- The prediction inputs are audited: V-03a, V-03b and V-03c are OPEN.
- Persistence is OPEN: no IRRS execution interface exists for it.
- Topology review: `TOPOLOGY_REVIEW.md`.
- The ring defect is reported to the canonical authority: `DEFECT_REPORT_V06_RING_TOPOLOGY.md`.

