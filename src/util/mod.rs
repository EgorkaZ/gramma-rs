use combine::{
    parser::{Parser},
    ParseResult,
    error::{StreamError, Tracked},
    StreamOnce, Stream, ParseError};
use kiam::when;

pub enum LoopFlow<T>
{
    Continue,
    Break(T),
}

pub struct SkipCode<Pos>
{
    pos: Pos,
}

pub fn code<Pos: Default>() -> SkipCode<Pos>
{ SkipCode{ pos: Pos::default() } }

impl<In> Parser<In> for SkipCode<<In as StreamOnce>::Position>
    where In: Stream<Token = char>
{
    type Output = ();

    type PartialState = ();

    fn parse_lazy(&mut self, input: &mut In) -> ParseResult<Self::Output, <In as combine::StreamOnce>::Error> {
        let mut balance: i32 = 0;
        let mut consumed: usize = 0;

        loop {
            match StreamOnce::uncons(input) {
                Ok(ch) => {
                    when! {
                        ch == '{' => balance += 1,
                        ch == '}' => balance -= 1,
                    }

                    if consumed > 0 && balance == 0 {
                        self.pos = input.position();
                        return ParseResult::CommitOk(())
                    }
                    consumed += ch.len_utf8();
                },
                Err(_) => {
                    return ParseResult::PeekErr(Tracked::from(In::Error::empty(input.position())))
                },
            }
        }
    }

    fn add_error(&mut self, error: &mut Tracked<In::Error>) {
        let err_eoi = <<In as StreamOnce>::Error as ParseError<_, _, _>>::StreamError::end_of_input();
        let res_err = In::Error::from_error(self.pos.clone(), err_eoi);
        *error = Tracked::from(res_err)
    }
}
