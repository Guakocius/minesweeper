use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

<<<<<<< HEAD:rust/src/main/app.rs
<<<<<<<< HEAD:rust/src/main/app.rs
use crate::Tui; 
========
use crate::Tui;
>>>>>>>> 845b92b (added observer, model with board and field, and view):src/main/view/tui/app.rs
=======
use crate::Tui;
use crate::model::board::Board;
>>>>>>> 166f2dcd293967d29e5536fbfe26fe61898c38e9:rust/src/main/view/tui/app.rs

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn board_to_text(&self) -> Text<'static> {
        let lines: Vec<Line> = self.board.board.iter().enumerate().map(|(y, row)| {
            let spans: Vec<Span> = row.iter().enumerate().map(|(x, field)| {
                match field {
                    f if f.is_flag => "⚑".red(),
                    f if !f.is_opened => "■".gray(),
                    f if f.is_bomb => "*".red(),
                    _ => {
                        let n = self.board.get_bomb_neighbor(x as isize, y as isize);
                        if n == 0 {
                            " ".into()
                        } else {
                            n.to_string().cyan()
                        }
                    }
                }
            }).collect();

            Line::from(spans)
        }).collect();
        Text::from(lines)
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(())
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('h') => self.show_help_message(),
            KeyCode::Char('g') => self.generate_board(),
            KeyCode::Char('r') => self.redo(),
            KeyCode::Char('u') => self.undo(),
            KeyCode::Char('s') => self.save_game(),
            KeyCode::Char('l') => self.load_game(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn show_help_message(&mut self) {

    }
    /*fn generate_board(&mut self) -> Board {
        Board::default()
    }*/
    fn generate_board(&mut self) {

    }
    fn redo(&mut self) {

    }
    fn undo(&mut self) {

    }
    fn save_game(&mut self) {

    }
    fn load_game(&mut self) {

    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        let board = Board::new(
            10, 10,
            5, 5,
            10
        ).expect("failed to create board");

        Self {
            board,
            exit: false,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Winesmeeper - A Minesweeper Saga ".bold());

        let sys_commands = Line::from(vec![
            " Help ".into(),
            "<H> ".blue().bold(),
            " Generate ".into(),
            "<G> ".blue().bold(),
            " Redo ".into(),
            "<R> ".blue().bold(),
            " Undo ".into(),
            "<U> ".blue().bold(),
            " Save ".into(),
            "<S> ".blue().bold(),
            " Load ".into(),
            "<L> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let turn_commands = Line::from(vec![
            " Flag ".into(),
            "<F> ".blue().bold(),
            " Open Field ".into(),
            "<O> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_top(sys_commands.centered())
            .title_bottom(turn_commands.centered())
            .border_set(border::THICK);

        let board = self.board_to_text();

        Paragraph::new(board)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        //assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into()).unwrap();
        //assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert!(app.exit);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn handle_key_event_panic() {
        let mut app = App::default();
        let _ = app.handle_key_event(KeyCode::Left.into());
    }

    #[test]
    fn handle_key_event_overflow() {
        let mut app = App::default();
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert_eq!(
            app.handle_key_event(KeyCode::Right.into())
                .unwrap_err()
                .to_string(),
            "counter overflow"
        );
    }
}