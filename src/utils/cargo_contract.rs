// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use regex::Regex;
use std::{
    path::PathBuf,
    process::Command,
};

/// Run cargo with the supplied args
///
/// If successful, returns the stdout bytes
pub(crate) fn build(manifest_path: &PathBuf) -> Result<String, String> {
    let mut dir = manifest_path.clone();
    dir.pop(); // pop `Cargo.toml` from the path

    let output = Command::new("cargo")
        .arg("contract")
        .arg("build")
        .arg(format!("--manifest-path={}", manifest_path.display()))
        .current_dir(dir)
        .output()
        .map_err(|err| format!("oh no - {:?}", err))
        .expect("failed to execute process");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("string conversion failed");
        // extract the path to the resulting `.contract` from the output
        let re = Regex::new(
            r"Your contract artifacts are ready. You can find them in:\n([A-Za-z0-9_\-/]+)\n"
        )
            .expect("invalid regex");
        let captures = re
            .captures(&stdout)
            .ok_or("regex does not match the command output")
            .map_err(|err| format!("{}: '{:?}'", err, stdout))?;
        let path = captures.get(1).expect("no capture group found").as_str();
        Ok(String::from(path))
    } else {
        let stderr = String::from_utf8(output.stderr).expect("string conversion failed");
        Err(format!(
            "Failed with exit code: {:?} and '{:?}'",
            output.status.code(),
            stderr
        ))
    }
}