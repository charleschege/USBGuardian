use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

mod properties;
use udev::{Device, Devices};

use crate::properties::*;
use anyhow::Result;

fn main() -> Result<()> {
    //USBDeviceSimple::print_devices(USBDeviceSimple::enumerate_devices());
    let monitor = udev::MonitorBuilder::new()?;
    let mut socket = monitor.match_subsystem("usb")?.listen()?;
    loop {
        match socket.next() {
            Some(event) => {
                USBDeviceSimple::print(USBDeviceSimple::enumerate(event.device()));
            }
            None => {
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }
        }
    }
}

struct USBDevice {
    devnum: String,
    devpath: String,
    devtype: String,
    kernel_drive: String,
    id_bus: String,
    id_model: String,
    id_model_enc: String,
    id_model_from_db: String,
    id_model_id: String,
    id_revision: String,
    id_serial: String,
    id_serial_short: String,
    id_usb_interfaces: String,
    id_vendor: String,
    id_vendor_enc: String,
    id_vendor_from_database: String,
    major: String,
    minor: String,
    product: String,
    seqnum: String,
    r#type: String,
    usec_initialized: String,
    init: bool,
}
/*
fn usb_device(device: &Device) -> USBDevice {
    USBDevice {
        devnum: device.property_value(DEVNUM),
        devpath: device.property_value(DEVPATH),
        devtype: device.property_value(DEVTYPE),
        kernel_drive: device.property_value(KERNEL_DRIVER),
        id_bus: device.property_value(ID_BUS),
        id_model: device.property_value(ID_MODEL),
        id_model_enc: device.property_value(ID_MODEL_ENC),
        id_model_from_db: device.property_value(ID_MODEL_FROM_DATABASE),
        id_model_id: device.property_value(ID_MODEL_ID),
        id_revision: device.property_value(ID_REVISION),
        id_serial: device.property_value(ID_SERIAL),
        id_serial_short: device.property_value(ID_SERIAL_SHORT),
        id_usb_interfaces: device.property_value(ID_USB_INTERFACES),
        id_vendor: device.property_value(ID_VENDOR),
        id_vendor_enc: device.property_value(ID_VENDOR_ENC),
        id_vendor_from_database: device.property_value(ID_VENDOR_FROM_DATABASE),
        major: device.property_value(MAJOR),
        minor: device.property_value(MINOR),
        product: device.property_value(PRODUCT),
        seqnum: device.property_value(SEQNUM),
        r#type: device.property_value(TYPE),
        usec_initialized: device.property_value(USEC_INITALIZED),
        init: device.is_initialized(),
    }
}*/
#[derive(Debug)]
enum USBUdevEvent {
    Add(PathBuf),
    Bind(PathBuf),
    Unbind(PathBuf),
    Remove(PathBuf),
}
struct USBDeviceSimple {
    device_name: String,
    device_id: String,
    vendor: String,
    vendor_id: String,
    model: String,
    syspath: String,
    devnum: String,
    product: String,
    subsystem: String,
    init: bool,
}

fn transform(value: Option<&OsStr>) -> String {
    match value {
        Some(value) => match value.to_str() {
            Some(value_str) => value_str.into(),
            None => "USBG::<CONVERT_TO_DISPLAY>::(ERROR)".into(), //FIXME `trasnform` to an `Result<_>`
        },
        None => "NOT DETECTED".into(),
    }
}

impl USBDeviceSimple {
    fn enumerate(device: Device) -> USBDeviceSimple {
        USBDeviceSimple {
            device_name: transform(device.property_value(ID_MODEL_FROM_DATABASE)),
            device_id: transform(device.property_value(ID_MODEL_ID)),
            vendor: transform(device.property_value(ID_VENDOR_FROM_DATABASE)),
            vendor_id: transform(device.property_value(ID_VENDOR_ID)),
            model: transform(device.property_value(ID_MODEL)),
            syspath: transform(device.property_value(DEVPATH)),
            devnum: transform(device.property_value(DEVNUM)),
            product: transform(device.property_value(PRODUCT)),
            subsystem: transform(device.property_value(SUBSYSTEM)),
            init: device.is_initialized(),
        }
    }

    fn enumerate_devices() -> Result<Vec<USBDeviceSimple>> {
        //FIXME handle Result
        let enumerator = udev::Enumerator::new();
        let devices = enumerator?.scan_devices()?;

        let mut usbdevices: Vec<USBDeviceSimple> = Vec::new();
        devices
            .into_iter()
            .filter(|device| device.devtype() == Some(OsStr::new("usb_device")))
            .for_each(|device| {
                let usbdevices_detected = USBDeviceSimple {
                    device_name: transform(device.property_value(ID_MODEL_FROM_DATABASE)),
                    device_id: transform(device.property_value(ID_MODEL_ID)),
                    vendor: transform(device.property_value(ID_VENDOR_FROM_DATABASE)),
                    vendor_id: transform(device.property_value(ID_VENDOR_ID)),
                    model: transform(device.property_value(ID_MODEL)),
                    syspath: transform(device.property_value(DEVNAME)),
                    devnum: transform(device.property_value(DEVNUM)),
                    product: transform(device.property_value(PRODUCT)),
                    subsystem: transform(device.property_value(SUBSYSTEM)),
                    init: device.is_initialized(),
                };
                usbdevices.push(usbdevices_detected);
            });

        Ok(usbdevices)
    }

    fn print(usb_device: Self) {
        println!(
            "[USB-{}]\nNAME: {}\nID: {}\nVENDOR: {}\nVENDOR ID: {}\nMODEL: {}\nPATH: {}\nDEV NUM: {}\nPRODUCT: {}\nSUBSYSTEM: {}\nONLINE: {}\n",
            usb_device.model,
            usb_device.device_name,
            usb_device.device_id,
            usb_device.vendor,
            usb_device.vendor_id,
            usb_device.model,
            usb_device.syspath,
            usb_device.devnum,
            usb_device.product,
            usb_device.subsystem,
            usb_device.init,
        )
    }

    fn print_devices(usb_devices: Vec<Self>) {
        usb_devices
            .into_iter()
            .map(|usb_device| {
                usb_device
            })
            .for_each(|usb_device| {
                println!(
                    "[USB-{}]\nNAME: {}\nID: {}\nVENDOR: {}\nVENDOR ID: {}\nMODEL: {}\nPATH: {}\nDEV NUM: {}\nPRODUCT: {}\nSUBSYSTEM: {}\nONLINE: {}\n",
                    usb_device.model,
                    usb_device.device_name,
                    usb_device.device_id,
                    usb_device.vendor,
                    usb_device.vendor_id,
                    usb_device.model,
                    usb_device.syspath,
                    usb_device.devnum,
                    usb_device.product,
                    usb_device.subsystem,
                    usb_device.init,
                )
            });
    }
}
//TODO check why multiple entries are being registered
//Maybe store in a map and check if the devname exists as a hash etc
//offer encryption to prevent hijacking by other OS level apps
