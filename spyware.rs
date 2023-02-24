use sysinfo::{System, SystemExt};

// LEFT A LOT TO DO
pub fn spy_resources()
{
    let mut sys = System::new_all();

    sys.refresh_all();
    let NAME = sys.name();
    let KERNEL = sys.kernel_version();
    let OS_INFO = sys.os_version();
    let HOSTNAME = sys.host_name();
    let MEMORY = format!("{}/{}", sys.total_memory(), sys.used_memory());
}
