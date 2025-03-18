use nom::IResult;
pub trait AtomicTrait: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;
}