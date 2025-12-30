use crate::protocol;
use anyhow::{Result, anyhow};
use hidapi::{DeviceInfo, HidApi};

pub fn run(api: &HidApi, device: &DeviceInfo) -> Result<()> {
    let device_path = device.path();
    let dev = api.open_path(device_path)?;
    let capabilities = protocol::scan_capabilities(&dev)?;
    if capabilities.vial_version != 0 {
        let status = protocol::get_locked_status(&dev)?;
        if status.locked {
            return Err(anyhow!(
                "Keyboard is locked you can unlock it by running subcommand 'lock -u' to unlock it"
            ));
        }
    }
    protocol::bootloader_jump(&dev)?;
    Ok(())
}
