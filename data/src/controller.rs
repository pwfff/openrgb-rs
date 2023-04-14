use alloc::string::String;
use alloc::vec::Vec;

use crate::protocol::OpenRGBReadableSync;
use crate::{Color, DeviceType, Mode, OpenRGBReadable, Zone, LED};
use crate::{OpenRGBError, OpenRGBWritable};

/// RGB controller.
///
/// See [Open SDK documentation](https://gitlab.com/CalcProgrammer1/OpenRGB/-/wikis/OpenRGB-SDK-Documentation#net_packet_id_request_controller_data) for more information.
#[derive(Debug, Eq, PartialEq)]
pub struct Controller {
    /// Controller type.
    pub r#type: DeviceType,

    /// Controller name.
    pub name: String,

    /// Controller vendor.
    pub vendor: String,

    /// Controller description.
    pub description: String,

    /// Controller version.
    pub version: String,

    /// Controller serial.
    pub serial: String,

    /// Controller location.
    pub location: String,

    /// Controller active mode index.
    pub active_mode: i32,

    /// Controller modes.
    pub modes: Vec<Mode>,

    /// Controller zones.
    pub zones: Vec<Zone>,

    /// Controller LEDs.
    pub leds: Vec<LED>,

    /// Controller colors.
    pub colors: Vec<Color>,
}

impl OpenRGBReadable for Controller {
    fn read(stream: &mut impl OpenRGBReadableSync, protocol: u32) -> Result<Self, OpenRGBError> {
        let _data_size = stream.read_value::<u32>(protocol)?;
        let r#type = stream.read_value(protocol)?;
        let name = stream.read_value(protocol)?;
        let vendor = stream.read_value(protocol)?;
        let description = stream.read_value(protocol)?;
        let version = stream.read_value(protocol)?;
        let serial = stream.read_value(protocol)?;
        let location = stream.read_value(protocol)?;
        let _num_modes = stream.read_value::<u16>(protocol)?;
        let active_mode = stream.read_value(protocol)?;
        let mut modes = Vec::with_capacity(_num_modes as usize);
        for _ in 0.._num_modes {
            modes.push(stream.read_value(protocol)?);
        }
        let zones = stream.read_value(protocol)?;
        let leds = stream.read_value(protocol)?;
        let colors = stream.read_value(protocol)?;

        Ok(Controller {
            r#type,
            name,
            vendor,
            description,
            version,
            serial,
            location,
            active_mode,
            modes,
            zones,
            leds,
            colors,
        })
    }
}

impl OpenRGBWritable for Controller {
    fn size(&self, protocol: u32) -> usize {
        let mut size = 0;
        size += 0u32.size(protocol);
        size += self.r#type.size(protocol);
        size += self.name.size(protocol);
        size += self.vendor.size(protocol);
        size += self.description.size(protocol);
        size += self.version.size(protocol);
        size += self.serial.size(protocol);
        size += self.location.size(protocol);
        size += (self.modes.len() as u16).size(protocol);
        size += self.active_mode.size(protocol);
        for mode in self.modes.iter() {
            size += mode.size(protocol);
        }
        size += self.zones.size(protocol);
        size += self.leds.size(protocol);
        size += self.colors.size(protocol);

        size
    }

    fn write(
        self,
        stream: &mut impl crate::OpenRGBWritableSync,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        stream.write_value(self.size(protocol) as u32, protocol)?;
        stream.write_value(self.r#type, protocol)?;
        stream.write_value(self.name, protocol)?;
        stream.write_value(self.vendor, protocol)?;
        stream.write_value(self.description, protocol)?;
        stream.write_value(self.version, protocol)?;
        stream.write_value(self.serial, protocol)?;
        stream.write_value(self.location, protocol)?;
        stream.write_value(self.modes.len() as u16, protocol)?;
        stream.write_value(self.active_mode, protocol)?;
        for mode in self.modes {
            stream.write_value(mode, protocol)?;
        }
        stream.write_value(self.zones, protocol)?;
        stream.write_value(self.leds, protocol)?;
        stream.write_value(self.colors, protocol)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::vec::Vec;
    use alloc::{format, vec};
    use num_traits::ToPrimitive;

    use crate::protocol::OpenRGBReadableSync;
    use crate::{Color, ColorMode, Controller, DeviceType, Mode, ModeFlag::*, Zone, ZoneType};

    static DEFAULT_PROTOCOL: u32 = 3;

    #[test]
    fn test_read_001() {
        let mut input: Vec<u8> = vec![
            3, 0, 0, 0, 18, 0, 84, 104, 101, 114, 109, 97, 108, 116, 97, 107, 101, 32, 82, 105,
            105, 110, 103, 0, 12, 0, 84, 104, 101, 114, 109, 97, 108, 116, 97, 107, 101, 0, 25, 0,
            84, 104, 101, 114, 109, 97, 108, 116, 97, 107, 101, 32, 82, 105, 105, 110, 103, 32, 68,
            101, 118, 105, 99, 101, 0, 1, 0, 0, 1, 0, 0, 19, 0, 72, 73, 68, 58, 32, 47, 100, 101,
            118, 47, 104, 105, 100, 114, 97, 119, 49, 48, 0, 8, 0, 0, 0, 0, 0, 7, 0, 68, 105, 114,
            101, 99, 116, 0, 24, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 7, 0,
            83, 116, 97, 116, 105, 99, 0, 25, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0,
            0, 1, 0, 0, 0, 0, 0, 5, 0, 70, 108, 111, 119, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 83, 112, 101, 99, 116, 114, 117, 109, 0, 4, 0, 0, 0, 1,
            0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 82, 105, 112, 112, 108, 101, 0,
            8, 0, 0, 0, 33, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 6, 0, 66, 108, 105, 110,
            107, 0, 12, 0, 0, 0, 33, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 6, 0, 80, 117,
            108, 115, 101, 0, 16, 0, 0, 0, 33, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 5,
            0, 87, 97, 118, 101, 0, 20, 0, 0, 0, 33, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 5, 0, 16, 0, 82, 105, 105, 110, 103, 32, 67, 104, 97, 110, 110, 101, 108, 32, 49, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 82, 105, 105, 110, 103,
            32, 67, 104, 97, 110, 110, 101, 108, 32, 50, 0, 1, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 16, 0, 82, 105, 105, 110, 103, 32, 67, 104, 97, 110, 110, 101, 108, 32,
            51, 0, 1, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 82, 105, 105, 110,
            103, 32, 67, 104, 97, 110, 110, 101, 108, 32, 52, 0, 1, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 16, 0, 82, 105, 105, 110, 103, 32, 67, 104, 97, 110, 110, 101,
            108, 32, 53, 0, 1, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        for (i, b) in input
            .len()
            .to_u32()
            .unwrap()
            .to_le_bytes()
            .iter()
            .enumerate()
        {
            input.insert(i, *b);
        }

        assert_eq!(
            input
                .as_slice()
                .read_value::<Controller>(DEFAULT_PROTOCOL)
                .expect("couldnt do it bro"),
            Controller {
                r#type: DeviceType::Cooler,
                name: format!("Thermaltake Riing"),
                vendor: format!("Thermaltake"),
                description: format!("Thermaltake Riing Device"),
                version: format!(""),
                serial: format!(""),
                location: format!("HID: /dev/hidraw10"),
                active_mode: 0,
                modes: vec![
                    Mode {
                        name: format!("Direct"),
                        value: 24,
                        flags: HasPerLEDColor.into(),
                        speed_min: None,
                        speed_max: None,
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: None,
                        colors_max: None,
                        speed: None,
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::PerLED),
                        colors: vec![],
                    },
                    Mode {
                        name: format!("Static"),
                        value: 25,
                        flags: HasModeSpecificColor.into(),
                        speed_min: None,
                        speed_max: None,
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: Some(1),
                        colors_max: Some(1),
                        speed: None,
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::ModeSpecific),
                        colors: vec![Color { r: 0, g: 0, b: 0 }],
                    },
                    Mode {
                        name: format!("Flow"),
                        value: 0,
                        flags: HasSpeed.into(),
                        speed_min: Some(3),
                        speed_max: Some(0),
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: None,
                        colors_max: None,
                        speed: Some(2),
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::None),
                        colors: vec![],
                    },
                    Mode {
                        name: format!("Spectrum"),
                        value: 4,
                        flags: HasSpeed.into(),
                        speed_min: Some(3),
                        speed_max: Some(0),
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: None,
                        colors_max: None,
                        speed: Some(2),
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::None),
                        colors: vec![],
                    },
                    Mode {
                        name: format!("Ripple"),
                        value: 8,
                        flags: HasSpeed | HasPerLEDColor,
                        speed_min: Some(3),
                        speed_max: Some(0),
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: None,
                        colors_max: None,
                        speed: Some(2),
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::PerLED),
                        colors: vec![],
                    },
                    Mode {
                        name: format!("Blink"),
                        value: 12,
                        flags: HasSpeed | HasPerLEDColor,
                        speed_min: Some(3),
                        speed_max: Some(0),
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: None,
                        colors_max: None,
                        speed: Some(2),
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::PerLED),
                        colors: vec![],
                    },
                    Mode {
                        name: format!("Pulse"),
                        value: 16,
                        flags: HasSpeed | HasPerLEDColor,
                        speed_min: Some(3),
                        speed_max: Some(0),
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: None,
                        colors_max: None,
                        speed: Some(2),
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::PerLED),
                        colors: vec![],
                    },
                    Mode {
                        name: format!("Wave"),
                        value: 20,
                        flags: HasSpeed | HasPerLEDColor,
                        speed_min: Some(3),
                        speed_max: Some(0),
                        brightness_min: None,
                        brightness_max: None,
                        colors_min: None,
                        colors_max: None,
                        speed: Some(2),
                        brightness: None,
                        direction: None,
                        color_mode: Some(ColorMode::PerLED),
                        colors: vec![],
                    },
                ],
                zones: vec![
                    Zone {
                        name: format!("Riing Channel 1"),
                        r#type: ZoneType::Linear,
                        leds_min: 0,
                        leds_max: 20,
                        leds_count: 0,
                        matrix: None,
                    },
                    Zone {
                        name: format!("Riing Channel 2"),
                        r#type: ZoneType::Linear,
                        leds_min: 0,
                        leds_max: 20,
                        leds_count: 0,
                        matrix: None,
                    },
                    Zone {
                        name: format!("Riing Channel 3"),
                        r#type: ZoneType::Linear,
                        leds_min: 0,
                        leds_max: 20,
                        leds_count: 0,
                        matrix: None,
                    },
                    Zone {
                        name: format!("Riing Channel 4"),
                        r#type: ZoneType::Linear,
                        leds_min: 0,
                        leds_max: 20,
                        leds_count: 0,
                        matrix: None,
                    },
                    Zone {
                        name: format!("Riing Channel 5"),
                        r#type: ZoneType::Linear,
                        leds_min: 0,
                        leds_max: 20,
                        leds_count: 0,
                        matrix: None,
                    },
                ],
                leds: vec![],
                colors: vec![],
            }
        );
    }
}
