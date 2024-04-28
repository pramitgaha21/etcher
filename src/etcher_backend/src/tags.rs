use bitcoin::blockdata::script;

use std::{convert::TryInto, mem};

/// The maximum allowed script size.
pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;

#[derive(Copy, Clone)]
#[allow(unused)]
pub(crate) enum Tag {
    Pointer = 2,
    Unbound = 66,

    ContentType = 1,
    Parent = 3,
    Metadata = 5,
    Metaprotocol = 7,
    ContentEncoding = 9,
    Delegate = 11,
    Rune = 13,
    Note = 15,
    Nop = 255,
}

impl Tag {
    fn is_chunked(self) -> bool {
        matches!(self, Self::Metadata)
    }

    pub(crate) fn bytes(self) -> &'static [u8] {
        match self {
            Self::Pointer => &[2],
            Self::Unbound => &[66],

            Self::ContentType => &[1],
            Self::Parent => &[3],
            Self::Metadata => &[5],
            Self::Metaprotocol => &[7],
            Self::ContentEncoding => &[9],
            Self::Rune => &[13],
            Self::Note => &[15],
            Self::Delegate => &[11],
            Self::Nop => &[255],
        }
    }

    pub(crate) fn encode(self, builder: &mut script::Builder, value: &Option<Vec<u8>>) {
        if let Some(value) = value {
            let mut tmp = script::Builder::new();
            mem::swap(&mut tmp, builder);

            if self.is_chunked() {
                for chunk in value.chunks(MAX_SCRIPT_ELEMENT_SIZE) {
                    tmp = tmp
                        .push_slice::<&script::PushBytes>(self.bytes().try_into().unwrap())
                        .push_slice::<&script::PushBytes>(chunk.try_into().unwrap());
                }
            } else {
                tmp = tmp
                    .push_slice::<&script::PushBytes>(self.bytes().try_into().unwrap())
                    .push_slice::<&script::PushBytes>(value.as_slice().try_into().unwrap());
            }

            mem::swap(&mut tmp, builder);
        }
    }
}
