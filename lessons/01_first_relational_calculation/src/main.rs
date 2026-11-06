//! ABR Mathematical Foundations — Lesson 1: First Relational Calculation.
//!
//! Observation → Declaration → IRRS admission → canonical Kernel V9 execution
//! → calculated result → comparison with observation.
//!
//! This program implements NO operator, NO admission rule and NO comparison
//! rule. Every mathematical step is a call into a pinned canonical crate:
//!
//!   IRRS observation admission  relational_gate::AdmissionGate::submit
//!                               (abr-irrs c7410c5, relational-gate/src/gate.rs)
//!   Canonical execution         abr_kernel::provenanced_exec::execute_primary
//!                               (abr-kernel 1c1ef70, src/provenanced_exec.rs),
//!                               which calls abr_kernel::operators::operator_delta
//!                               (src/operators.rs)
//!   V9 provenance admission     abr_kernel::admission::admit
//!                               (abr-kernel 1c1ef70, src/admission.rs)
//!
//! What this program does itself: read the student's declaration file, place
//! its text into the canonical declaration records, print what the canonical
//! code returns, and record the student's own comparison statement. It never
//! computes a difference, a magnitude, an agreement or a status from numbers.
//!
//! Usage:  cargo run -- <declaration file>

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use abr_kernel::admission::{admit, AdmissionOutcome as V9Outcome};
use abr_kernel::operators::EdgeProvenance;
use abr_kernel::provenance::{
    CalculatedOutput, DeclarationRegistry, DeclaredDomain, ExternalRecord, KernelSelection,
    LocusDeclaration, MMappingDeclaration, MMappingKind, ObservableDeclaration,
    ObservableRelationDeclaration, ObservationalBasis, OriginKind, RelationalDeclaration,
};
use abr_kernel::provenanced_exec::execute_primary;
use relational_gate::{
    AdmissionGate, AdmissionOutcome as GateOutcome, AdmittedObservation, AuthorizedMapping,
    CrossFamilyOrientation, DeclarationPackage, DeclaredEdge, Observable, ObservableExtent,
    ProvenanceChain, RegionBuilder, RegionalAdmissionRule, TopologySpec,
};

/// The lock file Cargo resolved for this build: the pinned canonical revisions
/// are read from it, not restated by hand.
const CARGO_LOCK: &str = include_str!("../Cargo.lock");

const TOPOLOGY_ID: &str = "lesson1.two_locus_ruler";
const REGION_NAME: &str = "lesson1.ruler_scale";
const M_ID: &str = "m.lesson1.identity";
const DOMAIN_ID: &str = "domain.lesson1.ruler_scale";
const STRUCTURE_ID: &str = "st.lesson1";

/// Keys a complete Lesson 1 declaration contains (DECLARATION_GUIDE.md).
const DECLARATION_KEYS: &[&str] = &[
    "observation_status", "instrument", "measurement_conditions", "unit",
    "scale_floor", "scale_ceiling", "scale_basis",
    "L1", "L1_description", "L1_record_entry",
    "L2", "L2_description", "L2_record_entry",
    "relation", "direction_basis", "continuation_basis",
    "mapping_preserves", "mapping_discards",
    "kernel_selection", "kernel_selection_basis",
];

/// Without these the canonical records cannot be built at all.
const ESSENTIAL_KEYS: &[&str] = &[
    "observation_status", "unit", "scale_floor", "scale_ceiling", "L1", "L2", "relation",
    "kernel_selection",
];

// ── Declaration file ──────────────────────────────────────────────────────

struct Declaration {
    path: PathBuf,
    map: BTreeMap<String, String>,
}

impl Declaration {
    fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("could not read declaration file '{}': {e}", path.display()))?;
        let mut map = BTreeMap::new();
        for (n, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((k, v)) = line.split_once('=') else {
                return Err(format!("line {}: expected 'key = value', found '{line}'", n + 1));
            };
            let key = k.trim().to_string();
            let mut val = v.trim();
            if val.len() >= 2 && val.starts_with('"') && val.ends_with('"') {
                val = &val[1..val.len() - 1];
            }
            if map.insert(key.clone(), val.to_string()).is_some() {
                return Err(format!("line {}: key '{key}' declared twice", n + 1));
            }
        }
        Ok(Declaration { path: path.to_path_buf(), map })
    }

    /// A declared, non-empty value. An empty value is not a declaration.
    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.trim()).filter(|s| !s.is_empty())
    }

    fn number(&self, key: &str) -> Result<Option<f64>, String> {
        match self.get(key) {
            None => Ok(None),
            Some(s) => s.parse::<f64>().map(Some)
                .map_err(|_| format!("'{key}' must be a number in the declared unit, found '{s}'")),
        }
    }
}

/// relational-gate takes `&'static str` labels. Declared text is leaked once
/// for the life of this short program.
fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

// ── IRRS observable source ────────────────────────────────────────────────

/// One raw ruler reading, as the student recorded it.
struct Reading {
    id: String,
    value: f64,
}

impl Observable for Reading {
    fn source_id(&self) -> &str { &self.id }
    fn primary_value(&self) -> f64 { self.value }
}

fn basis(observables: &[&str], statement: Option<&str>) -> Option<ObservationalBasis> {
    statement.map(|s| ObservationalBasis {
        observables: observables.iter().map(|o| o.to_string()).collect(),
        statement: s.to_string(),
    })
}

/// Resolved source lines of the canonical packages, read from Cargo.lock.
fn locked_sources() -> Vec<String> {
    let mut out = Vec::new();
    let mut name = String::new();
    for line in CARGO_LOCK.lines() {
        if let Some(n) = line.strip_prefix("name = ") {
            name = n.trim_matches('"').to_string();
        }
        if let Some(src) = line.strip_prefix("source = ") {
            if name == "abr-kernel" || name == "relational-gate" {
                out.push(format!("{name}: {}", src.trim_matches('"')));
            }
        }
    }
    out
}

// ── Report ────────────────────────────────────────────────────────────────

struct Status {
    declaration: String,
    irrs_admission: String,
    v9_admission: String,
    calculation: String,
    operator_a: String,
    observation: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: cargo run -- <declaration file>\n\
                   e.g.   cargo run -- declarations/experiment_1_illustrative.txt");
        std::process::exit(2);
    }
    let decl = match Declaration::load(Path::new(&args[1])) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("INPUT ERROR (nothing evaluated): {e}");
            std::process::exit(2);
        }
    };

    let mut r = String::new();
    let status = run(&decl, &mut r);

    let _ = writeln!(r, "\n== Status summary ==");
    let _ = writeln!(r, "  Code execution            : completed (this report was produced)");
    let _ = writeln!(r, "  Declaration               : {}", status.declaration);
    let _ = writeln!(r, "  IRRS observation admission: {}", status.irrs_admission);
    let _ = writeln!(r, "  V9 provenance admission   : {}", status.v9_admission);
    let _ = writeln!(r, "  Canonical calculation     : {}", status.calculation);
    let _ = writeln!(r, "  Operator A (ABR kernel)   : {}", status.operator_a);
    let _ = writeln!(r, "  Observational verification: {}", status.observation);
    let _ = writeln!(r, "\nThese statuses are not interchangeable. Successful execution, admission and");
    let _ = writeln!(r, "calculation are conformance statements; only the last line concerns agreement");
    let _ = writeln!(r, "with observation (kernel_reference.md §16.3, §17).");

    print!("{r}");

    // Preserve the report next to the lesson (records/<declaration>.report.txt).
    let stem = decl.path.file_stem().map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "declaration".into());
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("records");
    let out = dir.join(format!("{stem}.report.txt"));
    match std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&out, &r)) {
        Ok(()) => eprintln!("Report preserved: {}", out.display()),
        Err(e) => eprintln!("WARNING: report not preserved ({e})"),
    }
}

fn run(decl: &Declaration, r: &mut String) -> Status {
    let mut st = Status {
        declaration: "OPEN".into(),
        irrs_admission: "not reached".into(),
        v9_admission: "not reached".into(),
        calculation: "OPEN — not evaluated".into(),
        operator_a: "not evaluated".into(),
        observation: "OPEN".into(),
    };

    let _ = writeln!(r, "ABR Mathematical Foundations — Lesson 1: First Relational Calculation");
    let _ = writeln!(r, "Declaration file: {}", decl.path.display());
    let _ = writeln!(r, "\n== Canonical authorities (resolved by Cargo.lock) ==");
    for s in locked_sources() {
        let _ = writeln!(r, "  {s}");
    }

    // ── Step 1: Declaration ──────────────────────────────────────────────
    let _ = writeln!(r, "\n== Step 1 — Declaration (Origin: the student) ==");
    let missing: Vec<&str> = DECLARATION_KEYS.iter().copied().filter(|k| decl.get(k).is_none()).collect();
    for k in DECLARATION_KEYS {
        let _ = writeln!(r, "  {k:24} = {}", decl.get(k).unwrap_or("<not declared>"));
    }
    if missing.is_empty() {
        st.declaration = "DECLARED (every Lesson 1 key present; admission decides admissibility)".into();
    } else {
        st.declaration = format!("INCOMPLETE — not declared: {}", missing.join(", "));
        let _ = writeln!(r, "  Not declared: {}. Nothing is substituted; the canonical admission", missing.join(", "));
        let _ = writeln!(r, "  below reports what these absences leave OPEN.");
    }
    let essential_missing: Vec<&str> = ESSENTIAL_KEYS.iter().copied().filter(|k| decl.get(k).is_none()).collect();
    if !essential_missing.is_empty() {
        let _ = writeln!(r, "  OPEN: the canonical records cannot be built without: {}.", essential_missing.join(", "));
        return st;
    }

    let origin_kind = match decl.get("observation_status").unwrap() {
        "illustrative" => OriginKind::SyntheticFixture,
        "measured" => OriginKind::Measured,
        other => {
            let _ = writeln!(r, "  OPEN: observation_status must be 'illustrative' or 'measured', found '{other}'.");
            return st;
        }
    };
    let numbers = (|| -> Result<(f64, f64, f64, f64), String> {
        Ok((decl.number("scale_floor")?.unwrap(), decl.number("scale_ceiling")?.unwrap(),
            decl.number("L1")?.unwrap(), decl.number("L2")?.unwrap()))
    })();
    let (floor, ceiling, l1, l2) = match numbers {
        Ok(v) => v,
        Err(e) => {
            let _ = writeln!(r, "  OPEN: {e}");
            return st;
        }
    };
    let relation = decl.get("relation").unwrap();
    let (src, tgt) = match relation.split_once("->").map(|(a, b)| (a.trim(), b.trim())) {
        Some(("L1", "L2")) => ("L1", "L2"),
        Some(("L2", "L1")) => ("L2", "L1"),
        _ => {
            let _ = writeln!(r, "  OPEN: relation must be 'L1 -> L2' or 'L2 -> L1', found '{relation}'.");
            return st;
        }
    };
    let selection = match decl.get("kernel_selection").unwrap() {
        "primary" => KernelSelection::Primary,
        "abr" => KernelSelection::Abr,
        other => {
            let _ = writeln!(r, "  OPEN: kernel_selection must be 'primary' or 'abr', found '{other}'.");
            return st;
        }
    };
    let unit = decl.get("unit").unwrap();
    let _ = writeln!(r, "  Observation status: {}", match origin_kind {
        OriginKind::Measured => "MEASURED — the student declares these readings were independently observed",
        _ => "ILLUSTRATIVE — supplied demonstration values, NOT an observation (V9 OriginKind::SyntheticFixture)",
    });

    // ── Step 2: IRRS observation admission (relational-gate) ─────────────
    let _ = writeln!(r, "\n== Step 2 — IRRS observation admission ==");
    let _ = writeln!(r, "  Called: relational_gate::RegionBuilder::establish, relational_gate::AdmissionGate::submit");
    let _ = writeln!(r, "          (abr-irrs c7410c5: relational-gate/src/region.rs, relational-gate/src/gate.rs)");
    let extent = match ObservableExtent::new(floor, ceiling, leak(unit)) {
        Ok(e) => e,
        Err(e) => {
            st.irrs_admission = format!("{e}");
            let _ = writeln!(r, "  {e}");
            return st;
        }
    };
    let mut builder = RegionBuilder::new()
        .name(REGION_NAME)
        .extent(extent)
        .admission_rule(RegionalAdmissionRule {
            name: "lesson1.on_printed_scale",
            description: "A reading is admitted into evaluation only when it lies on the instrument's \
                          declared printed scale [scale_floor, scale_ceiling], inclusive.",
        });
    // Orientation: declared text only. Absent text leaves the region OPEN.
    if let (Some(p), Some(d)) = (decl.get("mapping_preserves"), decl.get("mapping_discards")) {
        builder = builder.orientation(CrossFamilyOrientation { preserved: leak(p), discarded: leak(d) });
    }
    // Region identified from a recorded observation: the source-locus reading.
    let region_evidence = if src == "L1" { l1 } else { l2 };
    let region = match builder.establish(region_evidence) {
        Ok(reg) => reg,
        Err(e) => {
            st.irrs_admission = format!("{e}");
            let _ = writeln!(r, "  Region not established — {e}");
            return st;
        }
    };
    let _ = writeln!(r, "  Region '{}' established; extent [{:?}, {:?}] {}; evidence reading {:?} {}",
        region.name, region.extent.floor, region.extent.ceiling, region.extent.unit, region.observed_locus, unit);
    let topology = TopologySpec::new(TOPOLOGY_ID, region, vec![DeclaredEdge {
        from_family: leak(src),
        to_family: leak(tgt),
        provenance: "Continuation (both marks lie on one continuous ruler scale)",
        mapping_name: M_ID,
    }]);
    let gate: AdmissionGate<Reading, f64> = AdmissionGate::new(topology, vec![AuthorizedMapping::new(
        M_ID,
        "Declared M, kind Identity: the reading enters D unchanged in the declared unit",
        |primary, _reference| Ok(primary),
        |v: &f64| *v,
    )]);

    let mut admitted: BTreeMap<&str, AdmittedObservation<f64>> = BTreeMap::new();
    let mut gate_lines = Vec::new();
    for (locus, value) in [("L1", l1), ("L2", l2)] {
        let package = DeclarationPackage {
            source: Reading { id: format!("obs.{locus}"), value },
            mapping_name: M_ID,
            topology_id: TOPOLOGY_ID,
            provenance: ProvenanceChain::new()
                .link("read_scale", "The student reads the mark on the instrument's printed scale")
                .link("record", "The reading is written into the declaration file")
                .link("apply_declared_M", "The gate executes the declared identity M"),
        };
        match gate.submit(package) {
            GateOutcome::Admitted { observation, trace } => {
                let p = &trace.provenance;
                gate_lines.push(format!("  {locus}: ADMITTED — source {}, mapping {}, raw input {:?}, gate-computed {:?} {unit}",
                    p.source_id, p.mapping_applied, p.primary_input, p.gate_computed_result));
                admitted.insert(locus, observation);
            }
            GateOutcome::Divergent { observation, divergence, .. } => {
                gate_lines.push(format!("  {locus}: DIVERGENT (admitted, preserved) — {}", divergence.description));
                admitted.insert(locus, observation);
            }
            GateOutcome::Open { condition, .. } => gate_lines.push(format!("  {locus}: {condition}")),
            GateOutcome::Failed { error, .. } => gate_lines.push(format!("  {locus}: FAILED — {error}")),
            GateOutcome::Rejected { violation, .. } => gate_lines.push(format!("  {locus}: {violation}")),
        }
    }
    for l in &gate_lines {
        let _ = writeln!(r, "{l}");
    }
    if admitted.len() != 2 {
        st.irrs_admission = "OPEN — not every reading was admitted (see Step 2)".into();
        let _ = writeln!(r, "  Only admitted observations may enter the kernel. Stopping here.");
        return st;
    }
    st.irrs_admission = "ADMITTED — both readings (AdmittedObservation<f64>, identity M, inside declared extent)".into();

    // ── Step 3: Canonical declaration records (Kernel V9 RPPI) ───────────
    // Only gate-admitted values enter the registry.
    let obs_id = |l: &str| format!("obs.{l}");
    let locus_id = |l: &str| format!("locus.{l}");
    let rel_id = format!("rel.{src}_to_{tgt}");
    let both = [obs_id("L1"), obs_id("L2")];
    let both_ref: Vec<&str> = both.iter().map(|s| s.as_str()).collect();
    let mut reg = DeclarationRegistry::default();
    for locus in ["L1", "L2"] {
        let record = match (decl.get("instrument"), decl.get(&format!("{locus}_record_entry"))) {
            (Some(source), Some(entry)) => Some(ExternalRecord { source: source.into(), entry: entry.into() }),
            _ => None,
        };
        reg.observables.push(ObservableDeclaration {
            id: obs_id(locus),
            origin_kind: origin_kind.clone(),
            record,
            declared_conditions: decl.get("measurement_conditions").map(String::from),
            reported_value: admitted[locus].gate_result(),
            unit: unit.into(),
        });
        reg.loci.push(LocusDeclaration {
            id: locus_id(locus),
            observable_ref: obs_id(locus),
            m_mapping_ref: M_ID.into(),
            domain_ref: DOMAIN_ID.into(),
        });
    }
    let m_statement = match (decl.get("mapping_preserves"), decl.get("mapping_discards")) {
        (Some(p), Some(d)) => Some(format!("Identity M. Preserves: {p} Discards: {d}")),
        _ => None,
    };
    reg.mappings.push(MMappingDeclaration {
        id: M_ID.into(),
        kind: MMappingKind::Identity,
        input_unit: unit.into(),
        output_unit: unit.into(),
        basis: basis(&both_ref, m_statement.as_deref()),
    });
    reg.domains.push(DeclaredDomain {
        id: DOMAIN_ID.into(),
        floor,
        ceiling,
        unit: unit.into(),
        basis: basis(&both_ref, decl.get("scale_basis")),
    });
    reg.relations.push(ObservableRelationDeclaration {
        id: rel_id.clone(),
        src_locus: locus_id(src),
        tgt_locus: locus_id(tgt),
        // The kernel record names one observation for the relation; this
        // lesson names the target-locus reading (see DECLARATION_GUIDE.md).
        observable_source: obs_id(tgt),
        m_mapping_ref: M_ID.into(),
        declared_domain: DOMAIN_ID.into(),
        direction_basis: basis(&both_ref, decl.get("direction_basis")),
        class: Some(EdgeProvenance::Continuation),
        class_basis: basis(&both_ref, decl.get("continuation_basis")),
    });
    reg.structures.push(RelationalDeclaration {
        id: STRUCTURE_ID.into(),
        loci: vec![locus_id("L1"), locus_id("L2")],
        relations: vec![rel_id.clone()],
        kernel_selection: selection.clone(),
        selection_basis: basis(&both_ref, decl.get("kernel_selection_basis")),
        // ρ_base and χ₀ are not declared in Lesson 1: Σ is not evaluated.
        rho_base_ref: None,
        chi_0_ref: None,
    });
    let _ = writeln!(r, "\n== Step 3 — Canonical declaration records (abr_kernel::provenance) ==");
    let _ = writeln!(r, "  Loci (node index = position; labels only, NOT topology): [{}, {}]", locus_id("L1"), locus_id("L2"));
    let _ = writeln!(r, "  Declared relation {rel_id}: {} → {} (class Continuation), M {M_ID} (Identity), domain {DOMAIN_ID}",
        locus_id(src), locus_id(tgt));
    let _ = writeln!(r, "  Kernel selection: {selection:?}; ρ_base, χ₀: not declared");

    // ── Step 4: Canonical execution ───────────────────────────────────────
    let _ = writeln!(r, "\n== Step 4 — Canonical execution ==");
    let _ = writeln!(r, "  Called: abr_kernel::provenanced_exec::execute_primary(&registry, \"{STRUCTURE_ID}\")");
    let _ = writeln!(r, "          (abr-kernel 1c1ef70, src/provenanced_exec.rs). For a Primary selection it");
    let _ = writeln!(r, "          calls abr_kernel::operators::operator_delta (src/operators.rs): Δ(x)[e] = x[s] − x[t]");
    let submitted = match execute_primary(&reg, STRUCTURE_ID) {
        Ok(out) => out,
        Err(e) => {
            let _ = writeln!(r, "  NOT EXECUTED — canonical return value: {e:?}");
            if selection == KernelSelection::Abr {
                st.operator_a = "OPEN — not evaluated. ABR kernel (A → B → R) requested: canonical \
                    execute_primary returns NotExecutable; V9 admission supports the Primary kernel \
                    only (admission.rs: 'OPEN: ABR kernel A → B → R'); A → B → R also requires an \
                    interior locus s → t → u (kernel_reference.md §10), which two loci cannot supply".into();
                let _ = writeln!(r, "  Operator A is not evaluated. No substitute calculation is made.");
            }
            st.v9_admission = "not reached (nothing executed to submit)".into();
            return st;
        }
    };
    let delta_value = match &submitted.output {
        CalculatedOutput::Primary { delta, .. } => Some(delta.field[0][0]),
        _ => None,
    };
    let _ = writeln!(r, "  Calculation record (Generator's account, to be checked by admission):");
    for i in &submitted.record.inputs {
        let _ = writeln!(r, "    input  {} ← {} via {}: observed {:?}, projected {:?}",
            i.binding_ref, i.observable_ref, i.m_mapping_ref, i.observed_value, i.projected_value);
    }
    for rr in &submitted.record.relations {
        let _ = writeln!(r, "    relation {}: node {} → node {}, class {:?}", rr.relation_ref, rr.src_index, rr.tgt_index, rr.class);
    }
    let _ = writeln!(r, "    path   {:?}", submitted.record.path);

    // ── Step 5: V9 provenance admission ───────────────────────────────────
    let _ = writeln!(r, "\n== Step 5 — V9 provenance admission ==");
    let _ = writeln!(r, "  Called: abr_kernel::admission::admit(&submitted, &registry) (abr-kernel 1c1ef70, src/admission.rs)");
    match admit(&submitted, &reg) {
        V9Outcome::Admitted(ev) => {
            st.v9_admission = format!("ADMITTED — observational status {:?} (a provenance-conformance statement, not a correspondence claim)",
                ev.observational_status);
            let _ = writeln!(r, "  ADMITTED. Observational status: {:?}", ev.observational_status);
            let _ = writeln!(r, "  Comparison rule: {}", ev.comparison_rule);
            for e in &ev.entries {
                let _ = writeln!(r, "  Entry {:?}[{}] component {} = {:?} {unit}", e.step, e.relation, e.component, e.value);
                let _ = writeln!(r, "    contributing declarations: {}", e.contributing.join(", "));
            }
            for t in &ev.reverse_traces {
                let _ = writeln!(r, "  Reverse trace {} terminates in:", t.entry);
                for x in &t.termini {
                    let _ = writeln!(r, "    {} ({:?}) — {} / {}", x.observable, x.origin_kind, x.record.source, x.record.entry);
                }
            }
            for n in &ev.not_evaluated {
                let _ = writeln!(r, "  Not evaluated (not 0.0): {n:?}");
            }
            if let Some(v) = delta_value {
                st.calculation = format!("CALCULATED — Δ[{rel_id}] = {v:?} {unit} (operator_delta, admitted)");
            }
            st.operator_a = "not evaluated under this declaration: kernel selection is Primary (no interior \
                locus). The canonical operator applied is Δ, 'A under a declared renaming', formula identical \
                (operators_notation_and_constraint.md); B, R and Σ are not evaluated".into();
        }
        V9Outcome::Open(conds) => {
            st.v9_admission = format!("OPEN — {} condition(s); not admitted", conds.len());
            let _ = writeln!(r, "  OPEN — not admitted. Conditions returned to Origin:");
            for c in &conds {
                let _ = writeln!(r, "    {c:?}");
            }
            st.calculation = "OPEN — executed but not admitted; the value is withheld, not reported as a result".into();
            let _ = writeln!(r, "  The executed value is withheld: an unadmitted value is not a result.");
            return st;
        }
        V9Outcome::Rejected(v) => {
            st.v9_admission = format!("REJECTED — {v:?}");
            let _ = writeln!(r, "  REJECTED — {v:?}");
            st.calculation = "REJECTED — not a result".into();
            return st;
        }
    }

    // ── Step 6: Comparison with observation ───────────────────────────────
    let _ = writeln!(r, "\n== Step 6 — Comparison with observation ==");
    if let Some(v) = delta_value {
        let _ = writeln!(r, "  Calculated (admitted): Δ[{rel_id}] = x[{src}] − x[{tgt}] = {v:?} {unit}");
    }
    let keys = ["comparison_separation", "comparison_uncertainty", "comparison_order",
                "comparison_method", "comparison_outcome"];
    for k in keys {
        let _ = writeln!(r, "  {k:24} = {}", decl.get(k).unwrap_or("<not declared>"));
    }
    let outcome = decl.get("comparison_outcome");
    let complete = keys.iter().all(|k| decl.get(k).is_some());
    st.observation = match (&origin_kind, outcome, complete) {
        (OriginKind::Measured, Some("agrees"), true) =>
            "VERIFIED — by the student's (Origin's) recorded comparison of the admitted result with an \
             independent measured observation. Automated correspondence evaluation in IRRS (M4) is \
             planned, not operational".into(),
        (OriginKind::Measured, Some("disagrees"), true) =>
            "DIVERGENT — mathematically conformant and observationally divergent (§16.3); preserved, \
             returned to Origin; not VERIFIED".into(),
        (OriginKind::Measured, _, _) =>
            "OPEN — measured readings, but the comparison is not fully declared".into(),
        _ =>
            "OPEN — DEMONSTRATION ONLY. The readings are illustrative, so agreement with them is not \
             an observation and cannot be VERIFIED".into(),
    };
    let _ = writeln!(r, "  The program computes no agreement. The comparison outcome is the student's own");
    let _ = writeln!(r, "  recorded judgement (EXPERIMENT.md), made against an independent observation.");
    st
}
