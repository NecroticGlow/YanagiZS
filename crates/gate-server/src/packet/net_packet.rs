use byteorder::{ByteOrder, BE};
use evelyn_proto::{PacketHead, Protobuf, ProtobufDecodeError};

pub struct NetPacket<Proto> {
    pub head: PacketHead,
    pub body: Proto,
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("head magic mismatch")]
    HeadMagicMismatch,
    #[error("tail magic mismatch")]
    TailMagicMismatch,
    #[error("input buffer is less than overhead, len: {0}, overhead: {1}")]
    InputLessThanOverhead(usize, usize),
    #[error("out of bounds ({0}/{1})")]
    OutOfBounds(usize, usize),
    #[error("failed to decode PacketHead: {0}")]
    HeadDecode(ProtobufDecodeError),
    #[error("failed to decode body: {0}")]
    BodyDecode(ProtobufDecodeError),
}

type Result<T> = std::result::Result<T, DecodeError>;

const OVERHEAD: usize = 16;
const HEAD_MAGIC: [u8; 4] = 0x01234567_u32.to_be_bytes();
const TAIL_MAGIC: [u8; 4] = 0x89ABCDEF_u32.to_be_bytes();

impl<Proto> NetPacket<Proto>
where
    Proto: evelyn_proto::NapMessage,
{
    pub fn decode(buf: &[u8]) -> Result<Self> {
        if &buf[0..4] != HEAD_MAGIC {
            return Err(DecodeError::HeadMagicMismatch);
        }

        let head_len = buf.get_head_len()?;
        let body_len = buf.get_body_len()?;

        if OVERHEAD + head_len + body_len > buf.len() {
            return Err(DecodeError::OutOfBounds(
                OVERHEAD + head_len + body_len,
                buf.len(),
            ));
        }

        if &buf[(12 + head_len + body_len)..(16 + head_len + body_len)] != TAIL_MAGIC {
            return Err(DecodeError::TailMagicMismatch);
        }

        let head = PacketHead::decode(&buf[12..12 + head_len]).map_err(DecodeError::HeadDecode)?;
        let mut body = Proto::decode(&buf[12 + head_len..12 + head_len + body_len])
            .map_err(DecodeError::BodyDecode)?;

        body.xor_fields();

        Ok(NetPacket { head, body })
    }

    pub fn encode(&self) -> Box<[u8]> {
        let head_len = self.head.encoded_len();
        let body_len = self.body.encoded_len();
        let encoded_len = OVERHEAD + head_len + body_len;

        let mut buf = vec![0u8; encoded_len];
        (&mut buf[0..4]).copy_from_slice(&HEAD_MAGIC);
        BE::write_u16(&mut buf[4..6], self.body.get_cmd_id());
        BE::write_u16(&mut buf[6..8], head_len as u16);
        BE::write_u32(&mut buf[8..12], body_len as u32);

        self.head
            .encode(&mut buf[12..12 + head_len].as_mut())
            .unwrap();

        self.body
            .encode(&mut buf[12 + head_len..12 + head_len + body_len].as_mut())
            .unwrap();

        (&mut buf[12 + head_len + body_len..16 + head_len + body_len]).copy_from_slice(&TAIL_MAGIC);
        buf.into_boxed_slice()
    }
}

pub trait PacketData {
    fn get_cmd_id(&self) -> Result<u16>;
    fn get_head_len(&self) -> Result<usize>;
    fn get_body_len(&self) -> Result<usize>;
}

impl<T: AsRef<[u8]>> PacketData for T {
    fn get_cmd_id(&self) -> Result<u16> {
        sanity_check(self)?;
        Ok(BE::read_u16(&self.as_ref()[4..6]))
    }

    fn get_head_len(&self) -> Result<usize> {
        sanity_check(self)?;
        Ok(BE::read_u16(&self.as_ref()[6..8]) as usize)
    }

    fn get_body_len(&self) -> Result<usize> {
        sanity_check(self)?;
        Ok(BE::read_u32(&self.as_ref()[8..12]) as usize)
    }
}

#[inline]
fn sanity_check<T: AsRef<[u8]>>(buf: T) -> Result<()> {
    let buf = buf.as_ref();
    if buf.len() < OVERHEAD {
        return Err(DecodeError::InputLessThanOverhead(buf.len(), OVERHEAD));
    }

    Ok(())
}
