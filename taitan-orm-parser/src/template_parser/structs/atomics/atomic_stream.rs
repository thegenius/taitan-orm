use std::fmt::Debug;
use crate::template_parser::structs::atomics::atomic_trait::AtomicTrait;
use crate::Atomic;
use nom::character::complete::multispace0;
use nom::sequence::preceded;
use tracing::debug;

#[derive(Debug, Clone, PartialEq)]
pub struct AtomicStream<T: AtomicTrait + Clone + PartialEq + Debug> {
    pub atomics: Vec<T>,
}

impl<T> AtomicStream<T>
where
    T: AtomicTrait + Clone + PartialEq + Debug,
{
    pub fn parse(input: &str) -> Result<Self, String> {
        debug!("SqlTemplate::parse({})", input);
        let mut atomics = Vec::new();
        let mut remainder = input;
        loop {
            let parse_result = preceded(multispace0, T::parse)(remainder);
            match parse_result {
                Ok((remaining, parsed)) => {
                    debug!("SqlTemplate::parse({})->{:?}", remaining, parsed);
                    atomics.push(parsed);
                    remainder = remaining;
                }
                Err(err_msg) => {
                    debug!("SqlTemplate::parse error: {}", err_msg);
                    let err_msg = format!("failed to parse atomic: {}", input);
                    return Err(err_msg);
                }
            }

            if remainder.is_empty() {
                break;
            }
        }
        Ok(AtomicStream { atomics })
    }
}
