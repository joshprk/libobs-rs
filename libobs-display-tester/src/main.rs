use libobs_wrapper::{
    context::ObsContext,
    utils::StartupInfo
};

fn main() -> anyhow::Result<()> {
    let info = StartupInfo::new();
    let context = ObsContext::new(info)?;

    Ok(())
}
