use crate::colorscheme::{ToForeground, THEME};
use tui::text::Span;

pub trait SpanIntake {
    fn push_styled_char(&mut self, c: char);
}

impl<'a> SpanIntake for Vec<Span<'a>> {
    fn push_styled_char(&mut self, c: char) {
        self.push(Span::styled(c.to_string(), THEME.todo.fg()));
    }
}

/// Remembers that the next word needs to be capitalized
pub struct Capitalize {
    sync: [u8; 2],
}

impl Default for Capitalize {
    fn default() -> Self {
        Self { sync: [0, 0] }
    }
}

impl Capitalize {
    /// signals that the next word should be capitalized
    pub fn signal(&mut self) {
        if self.sync[0] == 0 {
            self.sync[0] = 2;
            return;
        }
        self.sync[1] = 2;
    }

    /// checks whether word should be capitalized
    /// should be queried only once per word
    pub fn capitalize(&mut self) -> bool {
        if self.sync[0] == 1 {
            if self.sync[1] == 0 {
                self.sync[0] = 0;
            } else {
                self.sync = [1, 0];
            }
            return true;
        }
        self.sync[0] = self.sync[0].saturating_sub(1);
        false
    }
}
