//! Kernel V9 provenance admission of the stopping-distance relation.
//!
//! The stopping distance is the directed difference on the one relation
//! Onset → Standstill. Declared by itself, that relation has two loci and no
//! interior locus, so Kernel V9 selects the Primary kernel, Δ(x)[e] =
//! x[s] − x[t]. Δ has the same formula as A (operators_notation_and_constraint.md:
//! "A under a declared renaming"). Primary is the scope that V9 admission
//! supports at abr-kernel 1c1ef70 (src/admission.rs; A → B → R is OPEN there).
//!
//! Calls only canonical code: relational_gate (admission of the two position
//! readings), abr_kernel::provenanced_exec::execute_primary, and
//! abr_kernel::admission::admit, which independently reconstructs the
//! calculation from the declaration records (RPPI) and traces it back to the
//! external record. The terminus is Simulated: V9 admits it "only as such".
//! This is a provenance-conformance statement, not empirical verification.
//! (Adapted from the removed Lesson 1 revision-1 program, commit 5ebe041.)

use std::fmt::Write as _;

use abr_kernel::admission::{admit, AdmissionOutcome as V9Outcome};
use abr_kernel::operators::EdgeProvenance;
use abr_kernel::provenance::{
    CalculatedOutput, DeclarationRegistry, DeclaredDomain, ExternalRecord, KernelSelection,
    LocusDeclaration, MMappingDeclaration, MMappingKind, ObservableDeclaration,
    ObservableRelationDeclaration, ObservationalBasis, OriginKind, RelationalDeclaration,
};
use abr_kernel::provenanced_exec::execute_primary;
use relational_gate::{
    AdmissionGate, AdmissionOutcome as GateOutcome, AuthorizedMapping, CrossFamilyOrientation,
    DeclarationPackage, DeclaredEdge, Observable, ObservableExtent, ProvenanceChain, RegionBuilder,
    RegionalAdmissionRule, TopologySpec,
};

const M_ID: &str = "m.lesson1.v9.position.identity";
const DOMAIN_ID: &str = "domain.lesson1.vehicle_position";
const STRUCTURE_ID: &str = "st.lesson1.stopping_relation";
const TOPOLOGY_ID: &str = "lesson1.v9.stopping_relation";
const UNIT: &str = "m";

struct Reading {
    id: String,
    value: f64,
}

impl Observable for Reading {
    fn source_id(&self) -> &str { &self.id }
    fn primary_value(&self) -> f64 { self.value }
}

fn basis(obs: &[&str], statement: &str) -> Option<ObservationalBasis> {
    Some(ObservationalBasis {
        observables: obs.iter().map(|o| o.to_string()).collect(),
        statement: statement.to_string(),
    })
}

pub struct V9Result {
    pub report: String,
    pub status: String,
    /// The admitted Δ value, only when V9 returned ADMITTED.
    pub delta: Option<f64>,
}

/// `onset`, `standstill`: the simulated vehicle positions (m) at the two states.
pub fn admit_stopping_relation(trial: &str, onset: f64, standstill: f64) -> V9Result {
    let mut r = String::new();
    let status: String;
    let _ = writeln!(r, "Kernel V9 provenance admission — stopping relation Onset → Standstill, event '{trial}'");

    // ── IRRS observation admission of the two readings ────────────────────
    let extent = ObservableExtent::new(0.0, 600.0, UNIT).expect("finite extent");
    let region = match RegionBuilder::new()
        .name("lesson1.v9.vehicle_position")
        .extent(extent)
        .orientation(CrossFamilyOrientation {
            preserved: "the vehicle's position along the road from the hazard-onset position, in m",
            discarded: "the vehicle between the two states; lateral and vertical motion",
        })
        .admission_rule(RegionalAdmissionRule {
            name: "lesson1.v9.position_extent",
            description: "A position reading is admitted only within 0 m to 600 m inclusive.",
        })
        .establish(onset)
    {
        Ok(reg) => reg,
        Err(e) => { status = format!("OPEN — region not established: {e}"); let _ = writeln!(r, "  {status}"); return V9Result { report: r, status, delta: None }; }
    };
    let gate: AdmissionGate<Reading, f64> = AdmissionGate::new(
        TopologySpec::new(TOPOLOGY_ID, region, vec![DeclaredEdge {
            from_family: "Onset",
            to_family: "Standstill",
            provenance: "Continuation: the vehicle travels from its hazard-onset state to its standstill state (D-SD-12)",
            mapping_name: M_ID,
        }]),
        vec![AuthorizedMapping::new(M_ID, "Declared M, kind Identity: the position reading enters D unchanged in m",
            |primary, _reference| Ok(primary), |v: &f64| *v)],
    );
    let mut admitted = Vec::new();
    for (locus, value) in [("Onset", onset), ("Standstill", standstill)] {
        let package = DeclarationPackage {
            source: Reading { id: format!("obs.{locus}"), value },
            mapping_name: M_ID,
            topology_id: TOPOLOGY_ID,
            provenance: ProvenanceChain::new()
                .link("simulate", "The Lesson 1 reference simulator integrates the declared illustrative braking event")
                .link("read_position", "The simulated position track is read at the declared vehicle state")
                .link("apply_declared_M", "The gate executes the declared identity M"),
        };
        match gate.submit(package) {
            GateOutcome::Admitted { observation, .. } => {
                let _ = writeln!(r, "  gate: {locus} ADMITTED, {:?} m", observation.gate_result());
                admitted.push(observation.gate_result());
            }
            other => {
                status = format!("OPEN — gate did not admit {locus}: {:?}", std::mem::discriminant(&other));
                let _ = writeln!(r, "  {status}");
                return V9Result { report: r, status, delta: None };
            }
        }
    }

    // ── Canonical declaration records ─────────────────────────────────────
    let both = ["obs.Onset", "obs.Standstill"];
    let mut reg = DeclarationRegistry::default();
    for (locus, value) in [("Onset", admitted[0]), ("Standstill", admitted[1])] {
        reg.observables.push(ObservableDeclaration {
            id: format!("obs.{locus}"),
            origin_kind: OriginKind::Simulated,
            record: Some(ExternalRecord {
                source: "Lesson 1 reference simulator (src/sim.rs) with declarations/simulator_illustrative.json".into(),
                entry: format!("records/expanded_event/reference_observation.json — trial '{trial}', state_positions_m, state {locus}"),
            }),
            declared_conditions: Some("simulated braking event; Generator-chosen illustrative values".into()),
            reported_value: value,
            unit: UNIT.into(),
        });
        reg.loci.push(LocusDeclaration {
            id: format!("locus.{locus}"),
            observable_ref: format!("obs.{locus}"),
            m_mapping_ref: M_ID.into(),
            domain_ref: DOMAIN_ID.into(),
        });
    }
    reg.mappings.push(MMappingDeclaration {
        id: M_ID.into(),
        kind: MMappingKind::Identity,
        input_unit: UNIT.into(),
        output_unit: UNIT.into(),
        basis: basis(&both, "Identity M. Preserves: the vehicle's position along the road from the hazard-onset position. Discards: everything else about the vehicle."),
    });
    reg.domains.push(DeclaredDomain {
        id: DOMAIN_ID.into(),
        floor: 0.0,
        ceiling: 600.0,
        unit: UNIT.into(),
        basis: basis(&both, "the simulated position track covers 0 m to 600 m from the hazard-onset position"),
    });
    reg.relations.push(ObservableRelationDeclaration {
        id: "rel.Onset_to_Standstill".into(),
        src_locus: "locus.Onset".into(),
        tgt_locus: "locus.Standstill".into(),
        observable_source: "obs.Standstill".into(),
        m_mapping_ref: M_ID.into(),
        declared_domain: DOMAIN_ID.into(),
        direction_basis: basis(&both, "the simulated position track increases from the onset state to the standstill state: the vehicle travels forward along the road"),
        class: Some(EdgeProvenance::Continuation),
        class_basis: basis(&both, "one vehicle, one continuous stopping event, from its onset state to its standstill state (D-SD-12)"),
    });
    reg.structures.push(RelationalDeclaration {
        id: STRUCTURE_ID.into(),
        loci: vec!["locus.Onset".into(), "locus.Standstill".into()],
        relations: vec!["rel.Onset_to_Standstill".into()],
        kernel_selection: KernelSelection::Primary,
        selection_basis: basis(&both, "two loci and one relation: no interior locus, so the Primary kernel applies (kernel_reference.md §10)"),
        rho_base_ref: None,
        chi_0_ref: None,
    });

    // ── Execution and admission ───────────────────────────────────────────
    let _ = writeln!(r, "  Called: abr_kernel::provenanced_exec::execute_primary → operator_delta (abr-kernel 1c1ef70)");
    let submitted = match execute_primary(&reg, STRUCTURE_ID) {
        Ok(s) => s,
        Err(e) => { status = format!("OPEN — not executed: {e:?}"); let _ = writeln!(r, "  {status}"); return V9Result { report: r, status, delta: None }; }
    };
    let delta = match &submitted.output {
        CalculatedOutput::Primary { delta, .. } => Some(delta.field[0][0]),
        _ => None,
    };
    let _ = writeln!(r, "  Called: abr_kernel::admission::admit (abr-kernel 1c1ef70, src/admission.rs)");
    match admit(&submitted, &reg) {
        V9Outcome::Admitted(ev) => {
            status = format!("ADMITTED — observational status {:?} (provenance conformance; a Simulated terminus is not empirical evidence)", ev.observational_status);
            let _ = writeln!(r, "  {status}");
            let _ = writeln!(r, "  Comparison rule: {}", ev.comparison_rule);
            for e in &ev.entries {
                let _ = writeln!(r, "  Entry {:?}[{}] component {} = {:?} m; contributing: {}", e.step, e.relation, e.component, e.value, e.contributing.join(", "));
            }
            for t in &ev.reverse_traces {
                for x in &t.termini {
                    let _ = writeln!(r, "  Reverse trace {} → {} ({:?}): {} / {}", t.entry, x.observable, x.origin_kind, x.record.source, x.record.entry);
                }
            }
            for n in &ev.not_evaluated {
                let _ = writeln!(r, "  Not evaluated (not 0.0): {n:?}");
            }
            V9Result { report: r, status, delta }
        }
        V9Outcome::Open(conds) => {
            status = format!("OPEN — {} condition(s); not admitted", conds.len());
            let _ = writeln!(r, "  {status}");
            for c in &conds { let _ = writeln!(r, "    {c:?}"); }
            V9Result { report: r, status, delta: None }
        }
        V9Outcome::Rejected(v) => {
            status = format!("REJECTED — {v:?}");
            let _ = writeln!(r, "  {status}");
            V9Result { report: r, status, delta: None }
        }
    }
}
