use std::io::{self, Read};
use canvas::Canvas;
use painter::Painter;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut canvas = Canvas::new(500, 500);

    let mut painter = Painter::new();

    painter.draw(&mut canvas, &buffer);

    canvas.reset_boundary();

    canvas.print();

    Ok(())
}
