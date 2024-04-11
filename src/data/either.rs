use super::Parser;

pub enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}

pub type EitherParser<Left, Right> = Parser<Either<Left, Right>>;


