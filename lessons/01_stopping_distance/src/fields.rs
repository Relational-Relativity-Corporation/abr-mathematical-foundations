//! The declared relational fields of Exercise B (DESIGN_SPECIFICATION.md §3)
//! and the condition diamonds of §7, written as irrs-eval declarations.
//!
//! This module only *writes declarations*: D (extent, unit), M (mapping,
//! preserved / discarded), loci, relations with their bases, interior loci and
//! the operator parameters. It computes nothing. Readings are filled in from
//! the reference observation (sim.rs). Every declaration is saved to records/
//! exactly as it was submitted, and its SHA-256 is in the report.

use serde_json::{json, Value};

use crate::sim::Observation;

pub const TOLERANCE: f64 = 1e-9;

pub struct NodeSpec {
    pub id: String,
    pub about: String,
    pub value: f64,
}

pub struct EdgeSpec {
    pub from: String,
    pub to: String,
    pub class: &'static str,
    pub basis: String,
}

pub struct FieldSpec {
    pub stem: String,
    pub domain: String,
    pub description: String,
    pub unit: &'static str,
    pub floor: f64,
    pub ceiling: f64,
    pub preserved: String,
    pub discarded: String,
    pub rule: String,
    pub mapping_input: String,
    pub nodes: Vec<NodeSpec>,
    pub edges: Vec<EdgeSpec>,
    /// (node, incoming, continuing, basis)
    pub interior: Vec<(String, String, String, String)>,
    pub chi_0: f64,
    pub acquisition: String,
}

/// ρ_base for every field. ILLUSTRATIVE: chosen by the Generator for this
/// lesson (Origin direction: "Run ABR using the IRRS as a guide"); no
/// observational basis; not carried from any other domain.
pub const RHO_BASE: f64 = 0.5;

impl FieldSpec {
    pub fn to_json(&self) -> String {
        let obs = |n: &NodeSpec| format!("obs.{}", n.id);
        let mut inputs: Vec<Value> = self.nodes.iter().map(|n| json!({
            "name": obs(n),
            "classification": "simulated_input",
            "source_identity": "SIMULATED — Lesson 1 reference simulator (src/sim.rs), illustrative declaration declarations/simulator_illustrative.json; no measurement was made",
            "acquisition_method": self.acquisition,
            "relationship_to_problem": n.about,
            "known_limitations": "simulated reading from Generator-chosen illustrative values; admitted only as simulated (V9); not empirical evidence",
            "provenance_status": "declared",
            "submitted_to_gate": true
        })).collect();
        for (name, rel) in [
            ("rho_base", "dimensionless ceiling of ρ used by operator_r"),
            ("chi_0", "reference of the saturating map in ρ, in the dimension of χ"),
        ] {
            inputs.push(json!({
                "name": name,
                "classification": "declared_parameter",
                "source_identity": "ILLUSTRATIVE — chosen for this field by the Generator; taken from no other domain or repository",
                "acquisition_method": "declared by M for this field (kernel_reference.md §9: M-supplied, no kernel value)",
                "relationship_to_problem": rel,
                "known_limitations": "no observational basis; ρ, B and R depend on it",
                "provenance_status": "declared",
                "submitted_to_gate": false
            }));
        }
        let mapping = format!("m.{}.identity", self.stem);
        let interior_node = self.interior.first().map(|i| i.0.clone()).unwrap_or_default();
        let readings: serde_json::Map<String, Value> =
            self.nodes.iter().map(|n| (obs(n), json!(n.value))).collect();
        let doc = json!({
            "domain": self.domain,
            "topology_id": format!("lesson1.{}", self.stem),
            "version": "lesson1-v0.3-simulated",
            "description": self.description,
            "persistence": {
                "process_entry": "One declared instant of one simulated braking event. No sequence of observations is declared: the simulator is not declared as a transducer from observable inputs (kernel_reference.md §8; DESIGN D-SD-6), so Phase 2 is not entered.",
                "prior_evidence": "open",
                "persistence_path": ["A_persistence", "B_persistence", "R_persistence"]
            },
            "inputs": inputs,
            "regional_prerequisite": {
                "region_name": format!("lesson1.{}", self.stem),
                "extent_floor": self.floor,
                "extent_ceiling": self.ceiling,
                "extent_unit": self.unit,
                "orientation_preserved": self.preserved,
                "orientation_discarded": self.discarded,
                "admission_rule_name": format!("lesson1.{}.extent", self.stem),
                "admission_rule_description": self.rule,
                "observed_locus": self.nodes[0].value
            },
            "mappings": [{
                "name": mapping,
                "input_description": self.mapping_input,
                "output_description": format!("the same value in D, in {}", self.unit),
                "preserves": self.preserved,
                "discards": self.discarded
            }],
            "abr_topology": {
                "domain_observables": self.nodes.iter().map(obs).collect::<Vec<_>>(),
                "nodes": self.nodes.iter().map(|n| json!({
                    "id": n.id, "observable": obs(n), "observational_support": "proposed"
                })).collect::<Vec<_>>(),
                "edges": self.edges.iter().map(|e| json!({
                    "from_node": e.from, "to_node": e.to, "class": e.class,
                    "description": e.basis, "observational_support": "proposed",
                    "mapping_name": mapping
                })).collect::<Vec<_>>(),
                "interior_locus": { "node_id": interior_node, "observational_support": "proposed" },
                "operator_parameters": {
                    "rho_base": { "name": "rho_base", "value": RHO_BASE, "provenance": "illustrative" },
                    "chi_0": { "name": "chi_0", "value": self.chi_0, "provenance": "illustrative" }
                }
            },
            "lesson": {
                "observation_status": "SIMULATED — reference simulator readings (illustrative values); not an observation",
                "readings": readings,
                "interior_loci": self.interior.iter().map(|(n, i, c, b)| json!({
                    "node": n, "incoming": i, "continuing": c, "basis": b
                })).collect::<Vec<_>>(),
                "kernel_selection": "abr",
                "kernel_selection_basis": "the declared interior loci each have a declared incoming and a declared continuing relation (kernel_reference.md §10)",
                "detection_tolerance": TOLERANCE,
                "detection_tolerance_basis": "implementation-only numerical tolerance for finite-precision arithmetic (kernel_reference.md §11.5)",
                "provenance_links": [
                    { "step": "simulate", "description": "The Lesson 1 reference simulator integrates the declared illustrative braking event" },
                    { "step": "read_instrument", "description": "The simulated instrument reading for this locus is taken at the declared instant" },
                    { "step": "record", "description": "The reading is written into this field declaration" },
                    { "step": "apply_declared_M", "description": "The gate executes the declared identity M" }
                ]
            }
        });
        serde_json::to_string_pretty(&doc).expect("serialisable") + "\n"
    }
}

fn chain_interior(ids: &[String], basis: &str) -> Vec<(String, String, String, String)> {
    (1..ids.len() - 1).map(|k| (
        ids[k].clone(),
        format!("{} -> {}", ids[k - 1], ids[k]),
        format!("{} -> {}", ids[k], ids[k + 1]),
        basis.to_string(),
    )).collect()
}

/// F1 — along-road velocity of air, body, tread and road at one instant.
pub fn f1_velocity(obs: &Observation, instant: &str, values: [f64; 4]) -> FieldSpec {
    let ids: Vec<String> = ["Air", "Body", "Tread", "Road"].iter().map(|s| s.to_string()).collect();
    let about = [
        "wind velocity component along the direction of travel (+ = tailwind), from the simulated anemometer speed and direction",
        "vehicle body (centre of mass) velocity along the road",
        "tyre tread circumferential surface speed ω·r",
        "road surface velocity at the contact patch (the road is observed stationary)",
    ];
    FieldSpec {
        stem: format!("F1_velocity_{instant}"),
        domain: "lesson1.stopping_event".into(),
        description: format!("SIMULATED (illustrative values). Field F1: along-road velocity of the air, the vehicle body, the tyre tread surface and the road, at the instant '{instant}' of the simulated braking event '{}'.", obs.trial),
        unit: "m/s",
        floor: -60.0,
        ceiling: 60.0,
        preserved: "the velocity component along the direction of travel at each locus, in m/s".into(),
        discarded: format!("crosswind and vertical components (wind {} m/s toward {} deg from the direction of travel is projected to its along-road component by M); everything else about each locus", obs.wind_speed_m_s, obs.wind_direction_deg),
        rule: "A velocity reading is admitted only within the simulated instruments' range, −60 m/s to 60 m/s inclusive.".into(),
        mapping_input: "for Air: wind speed × cos(direction from travel), the declared along-road projection (DESIGN D-SD-3); for Body, Tread, Road: the along-road velocity reading in m/s".into(),
        nodes: ids.iter().zip(about).zip(values).map(|((id, a), v)| NodeSpec { id: id.clone(), about: a.into(), value: v }).collect(),
        edges: vec![
            EdgeSpec { from: "Air".into(), to: "Body".into(), class: "coupling",
                basis: "the air acts on the vehicle body through aerodynamic force; direction declared from the air to the body it acts on (DESIGN D-SD-2)".into() },
            EdgeSpec { from: "Body".into(), to: "Tread".into(), class: "continuation",
                basis: "the body's momentum is removed through the wheel and tyre during braking; direction declared along that momentum-removal path (DESIGN D-SD-2)".into() },
            EdgeSpec { from: "Tread".into(), to: "Road".into(), class: "coupling",
                basis: "the tyre tread transmits the braking force to the road at the contact patch; direction declared along the momentum-removal path (DESIGN D-SD-2)".into() },
        ],
        interior: chain_interior(&ids, "declared: momentum-removal path air/body → tread → road (DESIGN D-SD-2)"),
        chi_0: 1.0,
        acquisition: format!("simulated instrument reading at instant '{instant}' of the reference event"),
    }
}

fn station_ids(n: usize) -> Vec<String> {
    (0..n).map(|k| format!("S{k:02}")).collect()
}

fn station_edges(ids: &[String], spacing: f64) -> Vec<EdgeSpec> {
    ids.windows(2).map(|w| EdgeSpec {
        from: w[0].clone(), to: w[1].clone(), class: "continuation",
        basis: format!("the vehicle travels from station {} directly to station {} ({spacing} m along the same lane, no station between); direction is the observed direction of travel", w[0], w[1]),
    }).collect()
}

/// F2a — road elevation at each station.
pub fn f2_elevation(obs: &Observation, spacing: f64) -> FieldSpec {
    let ids = station_ids(obs.stations.len());
    FieldSpec {
        stem: "F2_road_elevation".into(),
        domain: "lesson1.stopping_event".into(),
        description: format!("SIMULATED (illustrative values). Field F2a: elevation of the road surface at each declared station along the stopping path of '{}', relative to station S00.", obs.trial),
        unit: "m",
        floor: -50.0,
        ceiling: 50.0,
        preserved: "the elevation of the road surface at each station relative to S00, in m".into(),
        discarded: "the road surface between stations; cross-slope; surface texture".into(),
        rule: "An elevation reading is admitted only within −50 m to 50 m of station S00 inclusive.".into(),
        mapping_input: "survey elevation reading at the station, in m relative to S00".into(),
        nodes: ids.iter().zip(&obs.stations).map(|(id, s)| NodeSpec {
            id: id.clone(), about: format!("road elevation at station {id}, {} m along the path from the hazard-onset position", s.position_m), value: s.elevation_m,
        }).collect(),
        edges: station_edges(&ids, spacing),
        interior: chain_interior(&ids, "the vehicle arrives at the station from the previous one and continues to the next"),
        chi_0: 0.5,
        acquisition: "simulated survey reading at the station".into(),
    }
}

/// F2b — tyre–road friction coefficient at each station.
pub fn f2_friction(obs: &Observation, spacing: f64) -> FieldSpec {
    let ids = station_ids(obs.stations.len());
    FieldSpec {
        stem: "F2_road_friction".into(),
        domain: "lesson1.stopping_event".into(),
        description: format!("SIMULATED (illustrative values). Field F2b: peak tyre–road friction coefficient for this vehicle's tyres on the road surface at each station of '{}'. Tread and moisture enter through these readings, not through a formula.", obs.trial),
        unit: "1",
        floor: 0.0,
        ceiling: 1.5,
        preserved: "the peak friction coefficient at each station (dimensionless)".into(),
        discarded: "friction between stations; the shape of the friction–slip curve; temperature".into(),
        rule: "A friction reading is admitted only within 0 to 1.5 inclusive.".into(),
        mapping_input: "friction-trailer reading at the station (dimensionless)".into(),
        nodes: ids.iter().zip(&obs.stations).map(|(id, s)| NodeSpec {
            id: id.clone(), about: format!("peak tyre–road friction coefficient at station {id}"), value: s.friction,
        }).collect(),
        edges: station_edges(&ids, spacing),
        interior: chain_interior(&ids, "the vehicle arrives at the station from the previous one and continues to the next"),
        chi_0: 0.05,
        acquisition: "simulated friction-trailer reading at the station".into(),
    }
}

/// F3 — perception–response chain event times.
pub fn f3_response(obs: &Observation) -> FieldSpec {
    let ids: Vec<String> = ["Obstacle", "Driver", "Brake", "Tread"].iter().map(|s| s.to_string()).collect();
    let about = [
        "time the obstacle becomes visible (hazard onset, t = 0)",
        "time the driver's response begins (foot leaves the accelerator): end of perception",
        "time brake pressure begins to rise",
        "time full braking deceleration is reached at the tyre tread",
    ];
    FieldSpec {
        stem: "F3_response_chain".into(),
        domain: "lesson1.stopping_event".into(),
        description: format!("SIMULATED (illustrative values). Field F3: the times of four state changes in the perception–response chain of '{}', from hazard onset.", obs.trial),
        unit: "s",
        floor: 0.0,
        ceiling: 10.0,
        preserved: "the time of each state change after hazard onset, in s".into(),
        discarded: "how each state change happens; illumination and attention themselves (they act through these times)".into(),
        rule: "An event time is admitted only within 0 s to 10 s after hazard onset inclusive.".into(),
        mapping_input: "event time from the synchronised event recorder, in s after hazard onset".into(),
        nodes: ids.iter().zip(about).zip(obs.event_times_s).map(|((id, a), v)| NodeSpec { id: id.clone(), about: a.into(), value: v }).collect(),
        edges: vec![
            EdgeSpec { from: "Obstacle".into(), to: "Driver".into(), class: "coupling",
                basis: "light from the obstacle reaches the driver, who detects it; direction is the observed signal path (DESIGN D-SD-2)".into() },
            EdgeSpec { from: "Driver".into(), to: "Brake".into(), class: "coupling",
                basis: "the driver's foot applies the brake pedal; direction is the observed signal path".into() },
            EdgeSpec { from: "Brake".into(), to: "Tread".into(), class: "continuation",
                basis: "brake pressure acts through the wheel to the tyre tread; direction is the observed signal path".into() },
        ],
        interior: chain_interior(&ids, "declared: observed signal path obstacle → driver → brake → tread"),
        chi_0: 0.25,
        acquisition: "simulated event-recorder time".into(),
    }
}

/// F4 — a condition diamond: four controlled trials, observed stopping distance.
pub fn diamond(id: &str, fa: &str, la: [&str; 2], fb: &str, lb: [&str; 2], d: [f64; 4]) -> FieldSpec {
    let n = |i: usize, j: usize| format!("{}_{}", la[i], lb[j]);
    let ids = [n(0, 0), n(1, 0), n(0, 1), n(1, 1)];
    let held = "same vehicle, same hazard, every other declared condition held at its base value";
    let ea = |j: usize| EdgeSpec {
        from: n(0, j), to: n(1, j), class: "coupling",
        basis: format!("controlled-condition contrast: {fa} changed from {} to {} with {fb} fixed at {}; {held}. Admissibility of a relation between separate trials is OPEN (DESIGN D-SD-8)", la[0], la[1], lb[j]),
    };
    let eb = |i: usize| EdgeSpec {
        from: n(i, 0), to: n(i, 1), class: "coupling",
        basis: format!("controlled-condition contrast: {fb} changed from {} to {} with {fa} fixed at {}; {held}. Admissibility of a relation between separate trials is OPEN (DESIGN D-SD-8)", lb[0], lb[1], la[i]),
    };
    FieldSpec {
        stem: format!("F4_{id}"),
        domain: "lesson1.condition_trials".into(),
        description: format!("SIMULATED (illustrative values). Field F4: the reference stopping distance observed in four controlled trials, {fa} × {fb}. The relations contrast the trials; they are not an observed passage."),
        unit: "m",
        floor: 0.0,
        ceiling: 600.0,
        preserved: "the stopping distance observed in each trial, in m".into(),
        discarded: "everything about each trial except its stopping distance".into(),
        rule: "A stopping distance is admitted only within 0 m to 600 m inclusive.".into(),
        mapping_input: "reference stopping distance of the trial, in m".into(),
        nodes: ids.iter().zip(d).map(|(i, v)| NodeSpec { id: i.clone(), about: format!("stopping distance in trial {i}"), value: v }).collect(),
        edges: vec![ea(0), ea(1), eb(0), eb(1)],
        interior: vec![
            (n(1, 0), format!("{} -> {}", n(0, 0), n(1, 0)), format!("{} -> {}", n(1, 0), n(1, 1)),
                format!("declared: trial {} is reached by a {fa} change and continues by a {fb} change (DESIGN D-SD-8)", n(1, 0))),
            (n(0, 1), format!("{} -> {}", n(0, 0), n(0, 1)), format!("{} -> {}", n(0, 1), n(1, 1)),
                format!("declared: trial {} is reached by a {fb} change and continues by a {fa} change (DESIGN D-SD-8)", n(0, 1))),
        ],
        chi_0: 5.0,
        acquisition: "simulated reference stopping distance of the trial".into(),
    }
}

/// F5 — the vehicle's own observed states during the stopping event.
///
/// The stopping distance is a relational quantity: the difference between
/// the vehicle's position at hazard onset and its position at standstill.
/// A on the declared relations between these states calculates it:
///   A[Onset->Standstill] = x(onset) − x(standstill) = −(stopping distance)
///   A on each chain relation = −(distance travelled in that phase)
///   B on a chain relation (immediate successors) combines a phase with the
///   next one, e.g. B[Onset->ResponseStart] = −(perception + response
///   distance) = −(reaction distance).
/// With component "speed", A gives the speed lost in each phase.
pub fn f5_states(obs: &Observation, component: &str) -> FieldSpec {
    let ids: Vec<String> = ["Onset", "ResponseStart", "BrakeOnset", "FullBraking", "Standstill"]
        .iter().map(|s| s.to_string()).collect();
    let what = [
        "the vehicle at hazard onset (obstacle first visible)",
        "the vehicle when the driver's response begins",
        "the vehicle when brake pressure begins",
        "the vehicle when full braking deceleration is reached",
        "the vehicle at standstill",
    ];
    let (unit, floor, ceiling, values, chi_0, quantity, instrument) = match component {
        "position" => ("m", 0.0, 600.0, obs.state_positions_m, 5.0,
            "position along the road from the hazard-onset position", "simulated position track (wheel odometry / GNSS)"),
        _ => ("m/s", 0.0, 60.0, obs.state_speeds_m_s, 1.0,
            "speed along the road", "simulated speed track"),
    };
    let next = |k: usize| EdgeSpec {
        from: ids[k].clone(), to: ids[k + 1].clone(), class: "continuation",
        basis: format!("SIMULATED observation: the vehicle's position track increases from state {} to state {} (the vehicle travels forward along the road), with no declared state between; direction is declared from that observed position increase, not from the order of the state labels. Generator-proposed; awaits Origin confirmation (D-SD-2)", ids[k], ids[k + 1]),
    };
    let mut edges: Vec<EdgeSpec> = (0..4).map(next).collect();
    edges.push(EdgeSpec {
        from: "Onset".into(), to: "Standstill".into(), class: "continuation",
        basis: "SIMULATED observation: the vehicle's position track increases from its hazard-onset state to its standstill state (one vehicle, one continuous stopping event). Generator-proposed so that the stopping distance is the directed difference on one relation; awaits Origin authorization (D-SD-12)".into(),
    });
    FieldSpec {
        stem: format!("F5_vehicle_states_{component}"),
        domain: "lesson1.stopping_event".into(),
        description: format!("SIMULATED (illustrative values). Field F5: the {quantity} of the vehicle at five observed states of the stopping event '{}'.", obs.trial),
        unit,
        floor,
        ceiling,
        preserved: format!("the vehicle's {quantity} at each declared state, in {unit}"),
        discarded: "the vehicle between the declared states; lateral and vertical motion; everything else about the vehicle".into(),
        rule: format!("A reading is admitted only within {floor} {unit} to {ceiling} {unit} inclusive."),
        mapping_input: format!("{instrument} reading at the declared state, in {unit}"),
        nodes: ids.iter().zip(what).zip(values).map(|((id, w), v)| NodeSpec {
            id: id.clone(), about: format!("{quantity} of {w}"), value: v,
        }).collect(),
        edges,
        interior: chain_interior(&ids, "the vehicle arrives at this state from the previous one and continues to the next in the observed course of the event"),
        chi_0,
        acquisition: format!("{instrument}, read at the declared state of the reference event"),
    }
}
