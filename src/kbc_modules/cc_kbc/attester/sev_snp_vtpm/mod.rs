// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use super::Attester;
use anyhow::*;
use serde::{Deserialize, Serialize};
use vtpm_snp::vtpm;

pub fn detect_platform() -> bool {
    vtpm::has_tpm_device()
}

#[derive(Debug, Default)]
pub struct VtpmAttester;

#[derive(Serialize, Deserialize)]
struct VtpmSnpEvidence {
    quote: vtpm::Quote,
    report: Vec<u8>,
}

impl Attester for VtpmAttester {
    fn get_evidence(&self, report_data: String) -> Result<String> {
        let report = vtpm::get_report()?;
        let report_data_bin = base64::decode(&report_data)?;
        let quote = vtpm::get_quote(&report_data_bin)?;

        let evidence = VtpmSnpEvidence { quote, report };

        Ok(serde_json::to_string(&evidence)?)
    }
}
