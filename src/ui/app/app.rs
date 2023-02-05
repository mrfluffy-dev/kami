use std::io;
use tui::{backend::Backend, Frame, Terminal};

pub trait KamiApp {
    fn new() -> Self;
    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>;
    fn ui<B: Backend>(&mut self, f: &mut Frame<B>);
}
