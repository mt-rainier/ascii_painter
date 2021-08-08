//! This crate is the main entry of ascii_painter. It processes command parameters and generates
//! callgraph diagram using [`Painter`] and [`Canvas`].
use canvas::Canvas;
use painter::Painter;
use std::io::{self, Read};
use std::{fs::File, io::Write, path::PathBuf};
use structopt::StructOpt;

/// Returns true if and only if stdin is believed to be connectted to a tty
/// or a console.
pub fn is_tty_stdin() -> bool {
    atty::is(atty::Stream::Stdin)
}

/// Command-line parameter structure for ascii_painter.
#[derive(StructOpt, Debug)]
#[structopt(
    name = "ascii_painter",
    about = "Converts callgraph text into UML sequence graph."
)]
struct Opt {
    /// Input file where the callgraph text is read from.
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output file where the result graph is written to.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

/// Main entry of ascii_painter program.
fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let mut buffer = String::new();

    if opt.input.is_none() {
        if is_tty_stdin() {
            println!("Need input text from either piped stdin or file. Use --help (-h) for help message.");
            return Ok(());
        }
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer)?;
    } else {
        let mut f = File::open(opt.input.unwrap().as_path())?;
        f.read_to_string(&mut buffer)?;
    }

    if buffer.is_empty() {
        println!("Input text is empty. Use --help (-h) for help message.");
        return Ok(());
    }

    let mut canvas = Canvas::new(10000, 10000);

    let mut painter = Painter::new();

    painter.draw(&mut canvas, &buffer);

    canvas.reset_boundary();

    if opt.output.is_some() {
        let mut output_f = File::create(opt.output.unwrap().as_path())?;
        output_f.write(canvas.to_string().as_bytes())?;
    } else {
        canvas.print();
    }

    Ok(())
}
