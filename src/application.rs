use crate::colorscheme;
use crate::langs;

use colorscheme::Theme;
use langs::prepare_test;

use std::time::{Duration, Instant};
use tui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use std::borrow::Cow;

pub enum Screen {
    Test,
    Post,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub cursor_x: u16,
}

impl App {
    pub fn new() -> Self {
        App {
            screen: Screen::Test,
            should_quit: false,
            cursor_x: 1,
        }
    }
}

#[derive(Debug)]
pub struct WpmHoarder {
    pub wpms: Vec<f64>,
    pub capacity: usize,
    pub seconds: u64,
}

impl WpmHoarder {
    fn new(capacity: usize) -> Self {
        WpmHoarder {
            capacity,
            wpms: Vec::with_capacity(capacity),
            seconds: 1,
        }
    }

    fn reset(&mut self) {
        self.wpms.clear();
        self.seconds = 1;
    }

    fn is_due(&mut self, begining: Instant) -> bool {
        let elapsed = begining.elapsed().as_secs();
        let due_time = self.seconds * (self.wpms.len() as u64 + 1);
        elapsed >= due_time
    }

    fn push(&mut self, wpm: f64) {
        self.wpms.push(wpm);
        if self.wpms.len() == self.capacity {
            let new_len: usize = self.wpms.len() / 2;
            for i in 0..(self.wpms.len() / 2) {
                self.wpms[i] = (self.wpms[i + 1] + self.wpms[i]) / 2.;
            }
            self.wpms.resize(new_len, 0.);
            self.seconds *= 2;
        }
    }
}

#[allow(dead_code)]
pub struct TestState<'a> {
    // letter inputs
    pub done: usize,
    // blanks are unfortuante consequence of appending mistakes
    // at the end of the word
    pub blanks: usize,
    // corrects are 99% not needed
    pub correct: u32,
    pub mistakes: u32,
    pub extras: u32,

    pub current_char: char,
    pub word_amount: u32,

    // TODO time of the first input
    pub begining: Instant,
    // source for generating test
    pub source: String,

    pub text: Vec<Span<'a>>,
    pub test_length: usize,
    pub hoarder: WpmHoarder,
}

impl<'a> Default for TestState<'a> {
    fn default() -> Self {
        TestState {
            text: vec![],
            begining: Instant::now(),
            done: 0,
            blanks: 0,
            correct: 0,
            extras: 0,
            mistakes: 0,
            source: "./languages/english".to_string(),
            test_length: 0,
            current_char: ' ',
            word_amount: 20,
            hoarder: WpmHoarder::new(30),
        }
    }
}

impl<'a> TestState<'a> {
    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 12. * (self.done - self.blanks) as f64;
        let elapsed = Instant::now().duration_since(self.begining).as_secs() as f64;
        numerator / elapsed
    }

    pub fn reset(&mut self, app: &mut App, th: &'a Theme) {
        app.cursor_x = 1;
        self.blanks = 0;
        self.done = 0;
        self.text = prepare_test(&self.source, self.word_amount, th);
        // self.text = langs::mock(th);
        self.begining = Instant::now();
        self.mistakes = 0;
        self.current_char = self.text[self.done].content.chars().next().unwrap();
        self.test_length = self.text.len();
        debug!("{:?}", self.hoarder);
        self.hoarder.reset();
    }

    pub fn update_wpm_history(&mut self) {
        if self.hoarder.is_due(self.begining) {
            self.hoarder.push(self.calculate_wpm());
        }
    }

    pub fn set_next_char(&mut self) {
        self.current_char = self.text[self.done].content.chars().next().expect("oof");
    }

    pub fn get_next_char(&mut self) -> Option<char> {
        self.text[self.done].content.chars().next()
    }

    pub fn fetch(&self, index: usize) -> &str {
        self.text[index].content.as_ref()
    }

    pub fn change(&mut self, index: usize, item: String) {
        self.text[index].content = Cow::from(item);
    }
}
