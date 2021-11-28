use crate::{
    processors::items::{ItemProcessor, Processors},
    syntax::items::definers,
};
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollector,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub public: bool,
    pub defining: bool,
    pub parameters: Vec<FunctionParameter>,
    pub parameters_pos: defs::Cursor,
    pub return_type: definers::DefinerCollector,
    pub return_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub body: Vec<Processors>,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCollector {
    pub data: Function,
    pub name_collected: bool,
    pub parameters_collected: bool,
    pub key_collected: bool,
    pub return_collected: bool,
    pub return_keyword_collected: bool,
    pub iterator: Box<crate::iterator::Iterator>,
    pub brace_count: usize,
    pub complete: bool,
}

impl Converter<FunctionCollector, ellie_core::definite::items::function::Function>
    for FunctionCollector
{
    fn to_definite(self) -> ellie_core::definite::items::function::Function {
        ellie_core::definite::items::function::Function {
            name: self.data.name,
            parameters: self
                .data
                .parameters
                .into_iter()
                .map(
                    |x| ellie_core::definite::items::function::FunctionParameter {
                        name: x.name,
                        rtype: x.rtype.definer_type.to_definite(),
                        pos: x.pos,
                        multi_capture: x.multi_capture,
                    },
                )
                .collect(),
            return_type: self.data.return_type.definer_type.to_definite(),
            public: self.data.public,
            body: self
                .data
                .body
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            name_pos: self.data.name_pos,
            parameters_pos: self.data.parameters_pos,
            return_pos: self.data.return_pos,
            pos: self.data.pos,
            body_pos: self.data.body_pos,
        }
    }

    fn from_definite(
        self,
        from: ellie_core::definite::items::function::Function,
    ) -> FunctionCollector {
        FunctionCollector {
            data: Function {
                name: from.name,
                name_pos: from.name_pos,
                public: from.public,
                parameters: from
                    .parameters
                    .into_iter()
                    .map(|x| FunctionParameter {
                        name: x.name,
                        rtype: definers::DefinerCollector {
                            definer_type: definers::DefinerTypes::default().from_definite(x.rtype),
                            complete: true,
                        },
                        pos: x.pos,
                        multi_capture: x.multi_capture,
                    })
                    .collect(),
                parameters_pos: from.parameters_pos,
                return_type: definers::DefinerCollector {
                    definer_type: definers::DefinerTypes::default().from_definite(from.return_type),
                    complete: true,
                },
                return_pos: from.return_pos,
                body_pos: from.body_pos,
                body: from
                    .body
                    .into_iter()
                    .map(|x| Processors::default().from_definite(x))
                    .collect(),
                pos: from.pos,
                defining: false,
            },
            ..Default::default()
        }
    }
}
