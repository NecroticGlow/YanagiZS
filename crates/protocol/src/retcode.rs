use num_enum::FromPrimitive;
use qwer::OctData;

#[repr(i32)]
#[derive(Debug, Default, FromPrimitive, OctData, PartialEq, Eq)]
pub enum Retcode {
    #[default]
    Succ = 0,
    Fail = -1,
}

impl From<Retcode> for i32 {
    fn from(value: Retcode) -> Self {
        value as i32
    }
}
