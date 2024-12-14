use super::{
    buffer::Buffer,
    terminal::{Size, Terminal},
};

#[derive(Default)]
pub struct View {
    buffer: Buffer,

}
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::clear_line()?;

        for current_row in 0..height{
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row){
                Terminal::print(&line);
            } else{
                Self::draw_empty_row();
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }
    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
