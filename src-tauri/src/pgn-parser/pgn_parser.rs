use nom::IResult;

#[derive(Debug)]
enum TagName {
    Event,
    Site,
    Date,
    Round,
    White,
    Black,
    Result,
}

#[derive(Debug)]
enum SupplementalTagName {
    WhiteTitle,
    BlackTitle,
    WhiteElo,
    BlackElo,
    WhiteUSCF,
    BlackUSCF,
    WhiteNA,
    BlackNA,
    WhiteType,
    BlackType,
    EventDate,
    EventSponsor,
    Section,
    Stage,
    Board,
    Opening,
    Variation,
    SubVariation,
    ECO,
    NIC,
    Time,
    UTCTime,
    UTCDate,
    TimeControl,
    SetUp,
    FEN,
    Termination,
    Annotator,
    Mode,
    PlyCount,
}

#[derive(Debug)]
struct TagPair {
    name: TagName,
    value: String,
}

#[derive(Debug)]
struct SupplementalTagPair {
    name: SupplementalTagName,
    value: String,
}

#[derive(Debug)]
struct Move {
    turn_number: u32,
    white_move: String,
    white_comment: Option<String>,
    black_move: Option<String>,
    black_comment: Option<String>,
}

#[derive(Debug)]
struct Game {
    tags: Vec<TagPair>,
    supplemental_tags: Option<Vec<SupplementalTagPair>>,
    moves: Vec<Move>,
    fens: Vec<String>
}

fn parse_pgn(input: &str) -> IResult<&str, Game> {
    todo!()
}

fn parse_tag_pair(input: &str) -> IResult<&str, TagPair> {
    todo!()
}

fn parse_supplemental_tag_pair(input: &str) -> IResult<&str, SupplementalTagPair> {
    todo!()
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    todo!()
}