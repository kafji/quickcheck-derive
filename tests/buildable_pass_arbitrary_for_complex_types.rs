use quickcheck::quickcheck;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct DateRange {
    pub begin: u64,
    pub end: u64,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct ReportMetadata {
    pub org_name: String,
    pub email: String,
    pub extra_contact_info: String,
    pub report_id: u64,
    pub date_range: DateRange,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub enum HandlingPolicy {
    None,
    Quarantine,
    Reject,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct PolicyPublished {
    pub domain: String,
    pub adkim: String,
    pub aspf: String,
    pub p: HandlingPolicy,
    pub sp: HandlingPolicy,
    pub pct: u8,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub enum PolicyResult {
    Fail,
    Pass,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct PolicyResultReason {
    type_: String,
    comment: String,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct PolicyEvaluated {
    pub disposition: String,
    pub dkim: PolicyResult,
    pub spf: PolicyResult,
    pub reason: Option<PolicyResultReason>,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct Row {
    pub source_ip: String,
    pub count: u32,
    pub policy_evaluated: PolicyEvaluated,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct Identifiers {
    pub header_from: String,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub enum AuthResult {
    Fail,
    Pass,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct DkimResult {
    pub domain: String,
    pub result: AuthResult,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct SpfResult {
    pub domain: String,
    pub result: AuthResult,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct AuthResults {
    pub dkim: Option<DkimResult>,
    pub spf: Option<SpfResult>,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct Record {
    pub row: Row,
    pub identifiers: Identifiers,
    pub auth_results: AuthResults,
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct DmarcReport {
    pub report_metadata: ReportMetadata,
    pub policy_published: PolicyPublished,
    pub record: Record,
}

fn main() {
    fn clone_equal_with_origin(report: DmarcReport) -> bool {
        report.clone() == report
    }
    quickcheck(clone_equal_with_origin as fn(DmarcReport) -> bool)
}
