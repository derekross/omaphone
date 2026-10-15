// SPDX-License-Identifier: GPL-3.0-or-later
//! Write omatel's fictional demo data (contacts, calls, messages) into the
//! directory given, for the screenshots: `demo-data DATA_HOME`. Refuses a
//! directory that already has omatel data.

fn main() -> anyhow::Result<()> {
    let dir = std::path::PathBuf::from(
        std::env::args_os()
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("usage: demo-data DATA_HOME"))?,
    );
    if dir.join("omatel").exists() || dir.join("contacts").exists() {
        anyhow::bail!("{} already has omatel data or contacts", dir.display());
    }
    omatel_mock::demo::seed(&dir, omatel_store::now_ms())?;
    Ok(())
}
