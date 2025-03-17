use std::fmt::Display;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::{Number, Operator, Placeholder, Variable, VariableChain};
use crate::template_parser::structs::bool_value::Bool;
use crate::template_parser::structs::text::Text;
use crate::template_parser::{TemplatePart};





