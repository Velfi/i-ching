mod hexagram_json;

use clap::{Parser, Subcommand, ValueEnum};
use hexagram_json::{HexagramJson, HexagramJsonInfo};
use iching::{
    divination_method::DivinationMethod, hexagram::Hexagram,
    hexagram_repository::HexagramRepository, trigram::Trigram,
};
use std::io::Write;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

// If the ABOUT is defined like this instead of using a doc comment, then the `--help` output will
// respect the line breaks.
const ABOUT: &str = r#"
8888888        .d8888b.  888      d8b
  888         d88P  Y88b 888      Y8P
  888         888    888 888
  888         888        88888b.  888 88888b.   .d88b.
  888         888        888 "88b 888 888 '88b d88P'88b
  888         888    888 888  888 888 888  888 888  888
  888         Y88b  d88P 888  888 888 888  888 Y88b 888
8888888        'Y8888P'  888  888 888 888  888  'Y88888
                                                    888
                                               Y8b d88P
                                                'Y88P'

Ever wish you could know the future? Well now you can!
Divine the answers to life's greatest mysteries using
the ancient Chinese method of the I-Ching.

Learn more on Wikipedia:
https://en.wikipedia.org/wiki/I_Ching
"#;

#[derive(Parser)]
#[command(author, version, arg_required_else_help(true), about = ABOUT)]
struct Args {
    /// Set whether output should be colorful or not
    #[arg(long, value_enum, default_value_t = ColorPreference::Auto)]
    color: ColorPreference,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum ColorPreference {
    /// Output will be colorful when a terminal is detected
    Auto,
    /// Output will always be colorful
    Always,
    /// Output will never be colorful
    Never,
}

impl From<ColorPreference> for termcolor::ColorChoice {
    fn from(value: ColorPreference) -> Self {
        match value {
            ColorPreference::Auto => termcolor::ColorChoice::Auto,
            ColorPreference::Always => termcolor::ColorChoice::Always,
            ColorPreference::Never => termcolor::ColorChoice::Never,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Receive the answers that you seek
    Divine {
        /// A question that you wish to answer
        ///
        /// This should be a question that you want to try to answer with divination. General
        /// questions are better than specific questions. Divination is an indirect method of
        /// introspection, and can help you become conscious of how you feel about something.
        /// With that in mind, take time to think about what you hope to accomplish by consulting
        /// the I-Ching
        #[arg(short, long, value_name = "QUESTION")]
        question: Option<String>,
        /// The method of divination to use.
        #[arg(short, long, value_name = "DIVINATION METHOD", value_enum, default_value_t = DivinationMethod::AncientYarrowStalk)]
        method: DivinationMethod,
    },
    /// Look up a hexagram by its King Wen sequence number
    Hexagram {
        /// The hexagram's number
        #[arg(long, value_name = "HEXAGRAM NUMBER", value_parser = clap::value_parser!(u8).range(1..=64))]
        number: u8,
    },
    /// Look up a trigram by its King Wen sequence number
    Trigram {
        /// The trigram's number
        #[arg(short, long, value_name = "TRIGRAM NUMBER", value_parser = clap::value_parser!(u8).range(1..=8))]
        number: u8,
    },
}

fn main() {
    let cli = Args::parse();

    let color = if cli.color == ColorPreference::Auto && !atty::is(atty::Stream::Stdout) {
        termcolor::ColorChoice::Never
    } else {
        cli.color.into()
    };
    let mut output = termcolor::StandardStream::stdout(color);

    let mut hexagrams = HexagramJson::new();
    hexagrams
        .initialize()
        .expect("hexagrams loaded successfully");

    if let Some(command) = cli.command {
        match command {
            Commands::Divine { question, method } => {
                let hexagram = Hexagram::new_random(method);

                print_fortune(&mut output, question.as_deref(), hexagram, &hexagrams);
            }
            Commands::Hexagram {
                number: hexagram_number,
            } => {
                let hexagram = hexagrams
                    .get_by_number(hexagram_number)
                    .expect("clap has validated this number already");

                hexagram
                    .write_to(&mut output)
                    .expect("hexagram written successfully");
            }
            Commands::Trigram {
                number: trigram_number,
            } => {
                let trigram: Trigram = trigram_number
                    .try_into()
                    .expect("clap has validated this number already");
                trigram
                    .write_to(&mut output)
                    .expect("trigram written successfully");
            }
        }
    }
}

fn print_fortune(
    output: &mut StandardStream,
    question: Option<&str>,
    hexagram: Hexagram,
    hexagrams: &impl HexagramRepository<HexagramInfo = HexagramJsonInfo>,
) {
    // Get the primary hexagram. panic if the repository doesn't contain the hexagram.
    let hexagram_info_pre_changes = hexagrams.get_info_for_hexagram(&hexagram);

    // Get the relating hexagram, if any lines in the primary hexagram are changing.
    let hexagram_info_post_changes = hexagram
        .relating_hexagram()
        .map(|h| hexagrams.get_info_for_hexagram(&h));

    // If the user provided a question, then print it out
    if let Some(question_text) = question {
        println!("Q: {question_text}\n");
    }

    // Print info for the primary hexagram
    hexagram_info_pre_changes
        .write_to(output)
        .expect("hexagram info written successfully");

    // Print info for any changing lines
    print_changing_lines_info(output, &hexagram, hexagram_info_pre_changes)
        .expect("changing lines info written successfully");

    // Print info for the relating hexagram (if applicable)
    if let Some(hexagram_info) = hexagram_info_post_changes {
        println!("Changes into:\n");
        hexagram_info
            .write_to(output)
            .expect("hexagram info written successfully");
    }
}

fn print_changing_lines_info<T>(
    output: &mut T,
    hexagram: &Hexagram,
    hexagram_info: &HexagramJsonInfo,
) -> Result<(), std::io::Error>
where
    T: WriteColor + Write,
{
    // Get a list of changing lines by their positions.
    let changing_line_positions = hexagram.get_changing_line_positions();

    // If there are no changing lines, return.
    if !changing_line_positions.is_empty() {
        writeln!(output)?;
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(130))))
            .expect("output stream color can be set");
        writeln!(output, "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~")?;
        output.reset().expect("output stream color can be reset");
        writeln!(output, "Lines are changing! Consider:")?;
        // Else, print out the changing line meanings.
        for line_meaning in hexagram_info.line_meanings(&changing_line_positions) {
            writeln!(output)?;
            output
                .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(220))))
                .expect("output stream color can be set");
            writeln!(output, "Line {} changes:", line_meaning.position)?;
            output.reset().expect("output stream color can be reset");
            writeln!(output, "{}", line_meaning.meaning)?;
        }
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(130))))
            .expect("output stream color can be set");
        writeln!(output, "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n")?;
        output.reset().expect("output stream color can be reset");
    }

    Ok(())
}
