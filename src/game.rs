use hyper::Client;
use hyper::header;
use hyper::client::Response;
use serde::json;
use std::io::Read;
use serde::{Serialize, Deserialize};
use error::*;
use protocol::*;
use action::*;
use std::ascii::AsciiExt;

#[derive(Debug,PartialEq)]
enum Status {
    Created,
    Guessing,
    Started,
    Failed,
    Complete,
    Finished,
}

pub struct Game {
    client: Client,
    player_id: String,
    req_url: String,
    status: Status,
    session_id: Option<String>,
    current_wrong_count: usize,
    total_word_guessed: usize,
    pub current_word: Option<String>
}

impl Game {
    pub fn new(req_url: String, player_id: String) -> Game {
        let c = ::HttpConnector(None, 20);
        Game {
            client: Client::with_connector(c),
            req_url: req_url,
            status: Status::Created,
            session_id: None,
            current_word: None,
            current_wrong_count: 0,
            total_word_guessed: 0,
            player_id: player_id,
        }
    }
    pub fn force_start(&mut self, session_id: String) {
        self.status = Status::Guessing;
        self.session_id = Some(session_id);
    }
    pub fn send_req<S>(&mut self, body: S) -> Result<Response, GameError> where S: Serialize {
        let js = try!(json::to_string(&body));
        debug!("send the req: {}", js);
        Ok(try!(self.client.post(&self.req_url[..]).
                header(header::ContentType(try!("application/json".parse()))).
                body(&js[..]).
                send()))
    }
    
    pub fn parse_resp<T>(&mut self, mut resp: Response) -> Result<T, GameError> where T: Deserialize {
        let mut ret_body = String::new();
        let _ = resp.read_to_string(&mut ret_body);
        let r: Result<T, _> = json::from_str(&ret_body);
        debug!("get the response: {}", ret_body);
        match r {
            Ok(resp) => Ok(resp),
            Err(err) => {
                let e: Result<ServerError,_> = json::from_str(&ret_body);
                match e {
                    Err(_) => {
                        println!("{}", ret_body);
                        try!(Err(err))
                    }
                    Ok(e) => try!(Err(e)),
                }
            }
        }
    }
    
    pub fn start_game(&mut self) -> Result<(), GameError> {
        let start = StartGameJson::new(self.player_id.clone());
        let resp = try!(self.send_req(start));
        let resp: StartGameResponse = try!(self.parse_resp(resp));
        
        self.session_id = Some(resp.sessionId);
        self.status = Status::Started;
        
        Ok(())
    }

    pub fn next_word(&mut self) -> Result<(), GameError> {
        match self.status {
            Status::Created => return Err(GameError::GameNotStarted),
            Status::Finished => return Err(GameError::GameFinished),
            _ => ()
        }

        if self.total_word_guessed == 80 {
            self.status = Status::Finished;
            return Err(GameError::GameFinished);
        }
        
        let next = NextWordJson::new(self.session_id.clone().unwrap());
        let resp = try!(self.send_req(next));
        let resp: NextWordResponse = try!(self.parse_resp(resp));
        
        self.session_id = Some(resp.sessionId);
        self.current_word = Some(resp.data.word);
        self.status = Status::Guessing;
        self.current_wrong_count = resp.data.wrongGuessCountOfCurrentWord;
        self.total_word_guessed = resp.data.totalWordCount;
        Ok(())
    }

    pub fn guess(&mut self, character: char) -> Result<bool, GameError> {
        match self.status {
            Status::Failed => return Err(GameError::GuessFailed),
            Status::Created => return Err(GameError::GameNotStarted),
            Status::Started => return Err(GameError::CurrentNoWord),
            Status::Finished => return Err(GameError::GameFinished),
            Status::Complete => return Err(GameError::GuessComplete),
            _ => ()
        }
        if self.current_wrong_count == 10 {
            self.status = Status::Failed;
            return Err(GameError::GuessFailed);
        }

        if !self.current_word.as_ref().unwrap().contains('*') {
            self.status = Status::Complete;
            return Err(GameError::GuessComplete);
        }
        
        let character = character.to_ascii_uppercase();
        
        let gs = GuessWordJson {
            sessionId: self.session_id.clone().unwrap(),
            action: GameAction::GuessWord,
            guess: character
        };
        let resp = try!(self.send_req(gs));
        let resp: GuessWordResponse = try!(self.parse_resp(resp));
        
        self.session_id = Some(resp.sessionId);
        self.current_wrong_count = resp.data.wrongGuessCountOfCurrentWord;
        
        if self.current_word.as_ref() == Some(&resp.data.word) {
            Ok(false)
        } else {
            self.current_word = Some(resp.data.word);
            Ok(true)
        }
    }

    pub fn get_result(&mut self) -> Result<GetResultResponse,GameError> {
        if self.status == Status::Created {
            return Err(GameError::GameNotStarted);
        }
        let gr = GetResultJson::new(self.session_id.clone().unwrap());
        let resp = try!(self.send_req(gr));
        let resp: GetResultResponse = try!(self.parse_resp(resp));
        Ok(resp)
    }
    
    pub fn submit_result(&mut self) -> Result<usize, GameError> {
        let sr = SubmitResultJson::new(self.session_id.clone().unwrap());
        let resp = try!(self.send_req(sr));
        let resp: SubmitResultResponse = try!(self.parse_resp(resp));
        Ok(resp.data.score)
    }
}
