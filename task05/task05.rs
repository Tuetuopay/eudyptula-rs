//! USB Device driver registration
//!
//! Left undone for the moment as the infrastructure for USB device is not there yet.

use kernel::{define_usb_id_table, module_usb_driver, prelude::*, usb};

module_usb_driver! {
    type: Task05,
    name: "task05",
    license: "GPL",
}

define_usb_id_table! {(), [
    (usb::DeviceId::interface_info(usb::IfClass::Hid, usb::IfSubClass::HidBoot, usb::IfProto::HidKeyboard), None),
]}

struct Task05;

impl usb::Driver for Task05 {
    fn probe(_iface: &mut usb::Interface, _device_id: &usb::DeviceId) -> Result<()> {
        pr_info!("Loaded USB driver.");
        Ok(())
    }
    fn disconnect(_data: &Self::Data) {
        pr_info!("Unloaded USB driver.");
    }
}
