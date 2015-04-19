use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct Dict {
    len_index: BTreeMap<usize, (usize, usize)>,
    pub inner: Vec<String>
}

impl Dict {
    pub fn new(path: &str) -> Result<Dict, Box<Error>> {
        let f = BufReader::new(try!(File::open(path)));
        let mut word_vec: Vec<String> = f.lines().map(|x| x.unwrap()).collect();
        word_vec.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut d = Dict {
            len_index: BTreeMap::new(),
            inner: word_vec,
        };

        let max_len = d.inner.last().unwrap().len();
        let (mut last_end, mut curr_idx) = (0, 0);
        for i in 0..max_len+1 {
            while curr_idx < d.inner.len() && d.inner[curr_idx].len() == i {
                curr_idx += 1;
            }
            d.len_index.insert(i, (last_end, curr_idx));
            last_end = curr_idx;
        }
        Ok(d)
    }
    pub fn guess(&self) -> Guesser {
        Guesser {
            history: vec![],
            last_input: None,
            dict: &self,
            slice: None
        }
    }
    
    pub fn get_len_index(&self, len: usize) -> (usize, usize) {
        self.len_index[&len]
    }
    
}

pub struct Guesser<'a> {
    history: Vec<char>,
    last_input: Option<String>,
    slice: Option<Vec<String>>,
    dict: &'a Dict
}

impl<'a> Guesser<'a> {
    pub fn set_history(&mut self, history: Vec<char>) {
        self.history = history;
    }
    pub fn set_last_input(&mut self, linput: String) {
        self.last_input = Some(linput);
    }
    pub fn guess(&mut self, input: &str) -> Option<char> {
        debug!("guesser: the input is: {:?}", input);
        if let None = self.slice {
            let (start, end) = self.dict.get_len_index(input.len());
            let slice = self.dict.inner[start..end].into();
            self.slice = Some(slice);
        }

        let valid_input_chars: Vec<(usize,char)> = input.chars().enumerate().filter_map(
            |(i,c)| {
                if c != '*' {
                    Some((i,c))
                } else {
                    None
                }
        }).filter(
            |&(idx, _)| {
                if let Some(linput) = self.last_input.as_ref() {
                    &linput[idx..idx+1] == "*"
                } else {
                    true
                }
            }).collect();

        trace!("history input: {:?}", self.history);
        let last_wrong_input = if let Some(last) = self.history.last() {
            if valid_input_chars.iter().count() == 0 {
                *last
            } else {
                '0'
            }
        } else {
            '0'
        };
        
        trace!("last wrong input: {:?}", last_wrong_input);
        debug!("guesser: valid_input_chars: {:?}", valid_input_chars);
        self.last_input = Some(input.to_string());
        
        
        let slice: Vec<String> = self.slice.take().unwrap();
        self.slice = Some(slice.into_iter().filter(|word| {
            if valid_input_chars.len() == 0 {
                word.chars().all(|c| c != last_wrong_input)
            } else {
                valid_input_chars.iter().all(|f| {
                    let v: Vec<u8> = word.clone().into();
                    v[f.0] == f.1 as u8
                })
            } 
        }).collect());
        
        debug!("guesser: candidate count: {:?}", self.slice.as_ref().unwrap().len());
        let mut statistic = BTreeMap::<char, usize>::new();
        for word in self.slice.as_ref().unwrap() {
            word.chars().all(
                |c| {
                    if self.history.iter().all(|&x| x != c) {
                        *statistic.entry(c).or_insert(0) += 1;
                    }
                    true
                });
        }
        trace!("btree is: {:?}", statistic);
        let (max_char, _) = statistic.into_iter().fold((None,0), |(ch, total_count), (sch, count)| if total_count < count { (Some(sch), count) } else { (ch, total_count) });
        if max_char.is_some() {
            self.history.push(max_char.unwrap());
        }
        debug!("guesser: guess: {:?}", max_char);
        max_char
    }
    
}

//#[test]
