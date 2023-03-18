use crate::{OpenRGBError, OpenRGBReadable, OpenRGBWritable, PacketId};
use alloc::format;
use genio::{Read, Write};

static MAGIC: [u8; 4] = *b"ORGB";

pub trait OpenRGBReadableSync: Read + Sized {
    fn read_value<T: OpenRGBReadable>(&mut self, protocol: u32) -> Result<T, OpenRGBError> {
        T::read(self, protocol)
    }

    fn read_u8(&mut self) -> Result<u8, OpenRGBError> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)
            .map_err(|_| OpenRGBError::CommunicationError())?;
        Ok(buf[0])
    }

    fn read_header(
        &mut self,
        protocol: u32,
        expected_device_id: u32,
        expected_packet_id: PacketId,
    ) -> Result<usize, OpenRGBError> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf[0..4])
            .map_err(|e| OpenRGBError::ProtocolError(format!("read error for magic")))?;
        for (i, c) in buf.iter().enumerate() {
            if *c != MAGIC[i] {
                return Err(OpenRGBError::BadMagic(*c));
            }
        }

        let device_id = self.read_value::<u32>(protocol)?;
        if device_id != expected_device_id {
            return Err(OpenRGBError::DeviceIDMismatch {
                expected: expected_device_id,
                got: device_id,
            });
        }

        let packet_id = self.read_value::<PacketId>(protocol)?;
        if packet_id != expected_packet_id {
            return Err(OpenRGBError::PacketIDMismatch {
                expected: expected_packet_id as u32,
                got: packet_id as u32,
            });
        }

        self.read_value::<u32>(protocol)?
            .try_into()
            .map_err(|_| OpenRGBError::CommunicationError())
    }

    fn read_packet<O: OpenRGBReadable>(
        &mut self,
        protocol: u32,
        expected_device_id: u32,
        expected_packet_id: PacketId,
    ) -> Result<O, OpenRGBError> {
        self.read_header(protocol, expected_device_id, expected_packet_id)?;
        // TODO check header length vs actual read length
        self.read_value(protocol)
    }
}

impl OpenRGBReadableSync for &[u8] {}

pub trait OpenRGBWritableSync: Write + Sized {
    fn write_value<T: OpenRGBWritable>(
        &mut self,
        value: T,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        T::write(value, self, protocol)
    }

    fn write_header(
        &mut self,
        protocol: u32,
        device_id: u32,
        packet_id: PacketId,
        data_len: usize,
    ) -> Result<(), OpenRGBError> {
        self.write_all(&MAGIC)
            .map_err(|_| OpenRGBError::CommunicationError())?;
        self.write_value(device_id, protocol)?;
        self.write_value(packet_id, protocol)?;
        self.write_value(data_len, protocol)?;
        Ok(())
    }

    fn write_packet<I: OpenRGBWritable>(
        &mut self,
        protocol: u32,
        device_id: u32,
        packet_id: PacketId,
        data: I,
    ) -> Result<(), OpenRGBError> {
        let size = data.size(protocol);
        {
            self.write_header(protocol, device_id, packet_id, size)?;
            self.write_value(data, protocol)?;
        }

        Ok(())
    }
}

pub trait OpenRGBSync: OpenRGBReadableSync + OpenRGBWritableSync {
    fn request<I: OpenRGBWritable, O: OpenRGBReadable>(
        &mut self,
        protocol: u32,
        device_id: u32,
        packet_id: PacketId,
        data: I,
    ) -> Result<O, OpenRGBError> {
        self.write_packet(protocol, device_id, packet_id, data)?;
        self.read_packet(protocol, device_id, packet_id)
    }
}

// impl OpenRGBReadableStream for TcpStream {}

// impl OpenRGBWritableStream for TcpStream {}

// impl OpenRGBStream for TcpStream {}

// #[cfg(debug_assertions)]
// impl OpenRGBWritableStream for Vec<u8> {}
