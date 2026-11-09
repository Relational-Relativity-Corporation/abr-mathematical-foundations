# Topology Review (V-05) — every declared relation, its basis, and its authorization

**No relation in this lesson is authorized.** Every node and edge is "proposed — not yet observationally established". No Origin authorization exists for any of them.

For each relation, the table separates:
- what the **simulator actually produces** (the simulated relationship);
- what the **Generator proposes** about direction and class.

Direction is never taken from array order or from label order. Where the only available basis is the order of events in time, the table says so.

| Field | Relation(s) | Simulated relationship (`src/sim.rs`) | Generator's proposed interpretation | Authorization status |
|---|---|---|---|---|
| F1 | Air → Body (coupling) | Drag force on the body from the air speed relative to it: −½ρC_dA(v − w)\|v − w\| | Direction from the air to the body it acts on | OPEN, D-SD-2 |
| F1 | Body → Tread (continuation) | Tread speed v(1 − s·b) and brake force b·μ·N acting on the body through the tyre | Direction along the path by which the body's momentum is removed | OPEN, D-SD-2 |
| F1 | Tread → Road (coupling) | Road velocity is 0; the tyre transmits brake force at the contact patch | Same momentum-removal path | OPEN, D-SD-2 |
| F2a, F2b | Sk → Sk+1, 16 each (continuation) | The vehicle's simulated position passes station k (at 25·k m) before station k+1 | Direction declared from the observed direction of travel (position increases) | OPEN, D-SD-2 |
| F3 | Obstacle → Driver, Driver → Brake (coupling); Brake → Tread (continuation) | **Only four event times, in increasing order.** The simulator models no signal path | "The observed signal path". **This is a Generator interpretation. The only simulated basis is the time order, which by itself is not a direction declaration** (§8: "temporal order is a projection") | OPEN, D-SD-2. **Flagged:** needs an observable basis beyond time order |
| F5 position, F5 speed | Onset → ResponseStart → BrakeOnset → FullBraking → Standstill (continuation) | The simulated position track increases from each state to the next | Direction declared from that position increase (corrected in this revision; previously "the observed order of the event") | OPEN, D-SD-2 |
| F5 position, F5 speed | Onset → Standstill (continuation) | The position increases from onset to standstill | Declared so that the stopping distance is the directed difference on one relation | OPEN, **D-SD-12** |
| V9 stopping relation | Onset → Standstill (Primary declaration, `src/v9.rs`) | As above | As above | OPEN, D-SD-12. V9 admission is provenance conformance, not authorization of the relation |
| F4 diamonds (4 × 4 relations, coupling) | a0_b → a1_b and a_b0 → a_b1 | **None.** The trials are separate simulations; nothing passes between them | Controlled-condition contrast: one factor changed, everything else held | OPEN, **D-SD-8**. Whether a relation between controlled trials is an observed passage is an Origin decision |
| Negative fixture neg4 | Tread → Obstacle (closing) | None | Deliberately inadmissible (a directed ring) | Must be refused; it is not (V-06, `DEFECT_REPORT_V06_RING_TOPOLOGY.md`) |

**Structure.** All declared topologies except neg4 are directed acyclic. F5 and the diamonds contain undirected cycles. That is not a ring under V9, which prohibits a node being its own relational predecessor.

*Bounded over D. No claim beyond D.*
