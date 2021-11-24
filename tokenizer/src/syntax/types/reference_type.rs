use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub pos: defs::Cursor,
    pub value: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceType {
    pub reference: Box<types::Processors>,
    pub reference_pos: defs::Cursor,
    pub chain: Vec<Chain>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceTypeCollector {
    pub data: ReferenceType,
    pub on_dot: bool,
    pub complete: bool,
}

impl ReferenceTypeCollector {
    pub fn to_definite(self) -> definite::types::reference::ReferenceType {
        definite::types::reference::ReferenceType {
            reference: Box::new(self.data.reference.to_definite()),
            reference_pos: self.data.reference_pos,
            chain: self
                .data
                .chain
                .into_iter()
                .map(|x| definite::types::reference::Chain {
                    pos: x.pos,
                    value: x.value,
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn from_definite(self, from: definite::types::reference::ReferenceType) -> Self {
        ReferenceTypeCollector {
            data: ReferenceType {
                reference: Box::new(types::Processors::default().from_definite(*from.reference)),
                reference_pos: from.reference_pos,
                chain: from
                    .chain
                    .into_iter()
                    .map(|x| Chain {
                        pos: x.pos,
                        value: x.value,
                    })
                    .collect::<Vec<_>>(),
            },
            complete: true,
            ..Default::default()
        }
    }
}
