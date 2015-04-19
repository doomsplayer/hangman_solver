use ::action::GameAction;
#[allow(non_snake_case)]  
#[derive(Serialize,Deserialize)]
pub struct StartGameJson {
    pub playerId: String,
    pub action: GameAction,
}
impl StartGameJson {
    pub fn new(player_id: String) -> StartGameJson {
        StartGameJson {
            playerId: player_id,
            action: GameAction::StartGame,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct StartGameResponse {
    pub message: String,
    pub sessionId: String,
    pub data: StartGameResponseData,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct StartGameResponseData {
    pub numberOfWordsToGuess: isize,
    pub numberOfGuessAllowedForEachWord: isize
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct NextWordJson {
    pub sessionId: String,
    pub action: GameAction,
}
impl NextWordJson {
    pub fn new(session_id: String) -> NextWordJson {
        NextWordJson {
            sessionId: session_id,
            action: GameAction::NextWord,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct NextWordResponse {
    pub sessionId: String,
    pub data: NextWordResponseData,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct NextWordResponseData {
    pub word: String,
    pub totalWordCount: usize,
    pub wrongGuessCountOfCurrentWord: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct GuessWordJson {
    pub sessionId: String,
    pub action: GameAction,
    pub guess: char,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct GuessWordResponse {
    pub sessionId: String,
    pub data: GuessWordResponseData,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct GuessWordResponseData {
    pub word: String,
    pub totalWordCount: usize,
    pub wrongGuessCountOfCurrentWord: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct GetResultJson {
    pub sessionId: String,
    pub action: GameAction,
}
impl GetResultJson {
    pub fn new(session_id: String) -> GetResultJson {
        GetResultJson {
            sessionId: session_id,
            action: GameAction::GetResult,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct GetResultResponse {
    pub sessionId: String,
    pub data: GetResultResponseData,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct GetResultResponseData {
    pub totalWordCount: isize,
    pub correctWordCount: isize,
    pub totalWrongGuessCount: isize,
    pub score: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct SubmitResultJson {
    pub sessionId: String,
    pub action: GameAction,
}
impl SubmitResultJson {
    pub fn new(session_id: String) -> SubmitResultJson {
        SubmitResultJson {
            sessionId: session_id,
            action: GameAction::SubmitResult,
        }
    }
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct SubmitResultResponse {
    pub message: String,
    pub sessionId: String,
    pub data: SubmitResultResponseData,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
pub struct SubmitResultResponseData {
    pub playerId: String,
    pub sessionId: String,
    pub totalWordCount: usize,
    pub correctWordCount: usize,
    pub totalWrongGuessCount: usize,
    pub score: usize,
    pub datetime: String,
}

#[derive(Serialize,Deserialize)]
pub struct ServerError {
    pub message: String
}
