#[derive(Deserialize,Debug,PartialEq)]
pub enum GameAction {
    StartGame,
    NextWord,
    GuessWord,
    GetResult,
    SubmitResult,
}
impl ::serde::Serialize for GameAction {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ::serde::Serializer,
    {
        use self::GameAction::*;
        let output = match self {
            &StartGame => "startGame",
            &NextWord => "nextWord",
            &GuessWord => "guessWord",
            &GetResult => "getResult",
            &SubmitResult => "submitResult",
        };
        serializer.visit_str(output)
    }
}
