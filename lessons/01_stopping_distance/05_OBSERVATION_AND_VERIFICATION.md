# 5 — Observation and Verification (Exercise C)

Content classes are as in [Document 1](01_BASIC_CALCULATION.md).

## 5.1 The reference event  [EXPLANATORY: a simulator, not the kernel]

`src/sim.rs` is a deterministic, fixed-step (0.1 ms) longitudinal model. It includes:
- gravity along the grade and the normal force;
- rolling resistance;
- aerodynamic drag on the speed relative to the air, varying continuously;
- brake force limited by the tyres, at peak friction μ(x), with friction interpolated *between* stations;
- a linear brake build-up after perception and response.

All its values are in `declarations/simulator_illustrative.json`. [CANONICAL] V9 admits a simulated observation "only as such". It is not empirical evidence (`v9_specification.md`, Simulated observations).

**Guard against agreement by construction (D-SD-9).** The simulator deliberately includes effects the expanded calculation leaves out:
- rolling resistance;
- drag that varies within a segment;
- friction that varies within a segment;
- grade and drag acting during the reaction phase.

So agreement is not built in.

## 5.2 Side by side

| | Basic prediction (Exercise A) | Expanded prediction (Exercise B) | Relational calculation from the observed states |
|---|---|---|---|
| Stopping distance | 100.187 m | 180.109 m | **174.953 m** = −A[Onset → Standstill] |
| Minus relational | −74.766 m | +5.156 m | — |
| Inputs | v, t, a (assumed) | condition fields (F1, F2, F3) via declared projections; declared mass, drag area, air density, g | the vehicle's observed states (F5), admitted by IRRS |
| Authority | Newtonian, by hand | a Newtonian formula **pending authorization** (D-SD-4) | Kernel V9 operator A, executed through IRRS |

The relational value agrees with the simulator's own stopping distance to 0.000 m, because it is calculated from the same observation. It is a calculation of observable data, not a prediction. The two predictions are what observation tests.

**Observed result.** Here the expanded prediction is closer to the relational value: |difference| 5.156 m against 74.766 m.

In the sixteen controlled trials, the expanded prediction differs from the relational value by between −1.088 m (downgrade, dry) and +5.973 m (worn tread, wet). The basic calculation differs by between −32.193 m (worn tread, wet) and +25.024 m (headwind, 60 mph). The expanded calculation tends to run long: it leaves out rolling resistance. It runs short on downgrades: it ignores the grade during the reaction phase. See Document 4 and the report.

These are agreements and disagreements **with a declared model**. They say nothing about the accuracy of either calculation for a real car.

## 5.3 Which relationships materially affected the expanded result

Each input was reverted to its base-trial value on its own, and the expanded calculation was rerun. The threshold for "material" is 1 m, provisional (D-SD-10).

| Reverted | Change in the expanded result |
|---|---|
| friction (tread × moisture, patchy surface; F2b) | +67.355 m, material |
| perception delay (visibility × attention; F3) | +33.346 m, material |
| road grade (F2a) | +9.355 m, material |
| aerodynamic drag at all | −5.897 m, material |
| brake build-up (F3) | +3.461 m, material |
| wind (F1 Air) | −1.874 m, material |

Mass enters only through drag divided by mass. With braking limited by friction, it otherwise cancels.

## 5.4 Unmeasured or unresolved  [OPEN]

- **No measurement exists.** Every value is illustrative and the reference is simulated, so observational verification is OPEN. A real test would replace the simulator readings with instrumented measurements and rerun the same declarations.
- **Phase 2** (changing deceleration, brake activation history): not entered (§8; D-SD-6). `irrs-session` has no persistence path.
- **ρ_base and χ₀** are illustrative, so R and ρ on the condition fields are reported only. On the stopping relation, R rests on empty sums (A-3 OPEN).
- **The Onset → Standstill relation** awaits Origin confirmation (D-SD-12).
- **V9 provenance admission for A → B → R:** OPEN, outside the `abr-kernel` `1c1ef70` admission scope.
- **Origin decisions:** D-SD-2 (direction bases), D-SD-3 (projections), D-SD-4 (expanded formula), D-SD-8 (relations between trials).

## 5.5 What this lesson establishes

**Established:**
- With the declared information, the canonical operators execute through IRRS.
- On the vehicle's observed states they calculate the stopping distance and its reaction and braking parts.
- On the condition fields they calculate latencies, grade, friction change, relative air speed and slip, which a declared Newtonian prediction can use.
- In the condition diamonds, two admitted A outputs show one condition's significance depending on another.

**Not established:** that any of these numbers describe a real vehicle. Accuracy has to be established through observation.

*Bounded over D. No claim beyond D.*
