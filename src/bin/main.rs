use clap::{
    App,
    ArgMatches,
};

use iching::{
    hexagram::{
        Hexagram,
        HexagramOrdering,
    },
    hexagram_repository::{
        HexagramInfo,
        HexagramRepository,
    },
    trigram::TrigramName,
};

use self::{
    args::{
        cast,
        hexagram,
        question,
        trigram,
    },
    hexagram_json::HexagramJson,
};

mod hexagram_json;
mod args;

const APP_TITLE: &str = r#"
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
Version"#;

const APP_DESCRIPTION: &str = r#"
Ever wish you could know the future? Well now you can!
Divine the answers to life's greatest mysteries using
the ancient Chinese method of the I-Ching.

Learn more on Wikipedia:
https://en.wikipedia.org/wiki/I_Ching

Simply run the app to get a reading. Passing a question with '-q' is optional
"#;

fn main() {
    let matches = start_app_and_get_matches();

    let mut hexagrams = HexagramJson::new();
    hexagrams.initialize().expect("Initialization of hexagrams has failed");

    if matches.is_present(args::hexagram::NAME) {
        print_hexagram_by_number(&matches, &hexagrams);
    } else if matches.is_present(args::trigram::NAME) {
        print_trigram_by_number(&matches);
    } else if matches.is_present(args::cast::NAME) {
        print_fortune_from_self_cast(&matches, &hexagrams);
    } else {
        print_fortune(
            matches.value_of(args::question::NAME),
            Hexagram::from_coin_tosses(),
            &hexagrams,
        );
    }
}

fn start_app_and_get_matches() -> ArgMatches<'static> {
    App::new(APP_TITLE)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(APP_DESCRIPTION)
        .arg(question::declare_arg())
        .arg(hexagram::declare_arg())
        .arg(trigram::declare_arg())
        .arg(cast::declare_arg())
        .get_matches()
}

fn print_hexagram_by_number(matches: &ArgMatches, hexagrams: &impl HexagramRepository) {
    let hexagram_number_string = matches.value_of(args::hexagram::NAME).unwrap();
    let hexagram_number_result = hexagram_number_string.parse::<usize>();

    match hexagram_number_result {
        Ok(hexagram_number) => match hexagrams.get_by_number(hexagram_number) {
            Some(hexagram) => println!("{}", hexagram),
            None => println!("Hexagrams number 1 to 64 but you asked for hexagram No. {}", hexagram_number),
        },
        Err(error) => println!("No hexagram for you: {}", error),
    }
}

fn print_trigram_by_number(matches: &ArgMatches) {
    let trigram_number_string = matches.value_of(args::trigram::NAME).unwrap();
    let trigram_number_result = trigram_number_string.parse::<usize>();

    match trigram_number_result {
        Ok(trigram_number) => match TrigramName::from_usize(trigram_number) {
            Ok(trigram) => println!("{}", trigram),
            Err(_) => println!("Trigrams number 1 to 8 but you asked for trigram No. {}", trigram_number),
        },
        Err(error) => println!("No trigram for you: {}", error),
    }
}

fn print_fortune_from_self_cast(matches: &ArgMatches, hexagrams: &impl HexagramRepository) {
    // unwrap here is ok because this should only be called after the existence of the value
    // has already been verified.
    let digits = matches.value_of(args::cast::NAME).unwrap();
    let user_question = matches.value_of(args::question::NAME);

    let hexagram = Hexagram::from_digits_str(digits)
        .expect(&format!("Failed to create a Hexagram from digits string: {}", digits));
    print_fortune(user_question, hexagram, hexagrams);
}

fn print_fortune(question: Option<&str>, hexagram: Hexagram, hexagrams: &impl HexagramRepository) {
    // Get the primary hexagram
    let hexagram_number_pre_changes = hexagram.as_number(false, HexagramOrdering::KingWen);
    let hexagram_info_pre_changes = hexagrams.get_by_number(
        hexagram_number_pre_changes
    ).expect(
        "Failed to get the primary hexagram info by number.\
        Perhaps there is an issue with the repository?"
    );

    // Get the relating hexagram (if applicable)
    let hexagram_number_post_changes = hexagram.as_number(true, HexagramOrdering::KingWen);
    let hexagram_info_post_changes = if hexagram_number_pre_changes != hexagram_number_post_changes {
        hexagrams.get_by_number(hexagram_number_post_changes)
    } else { None };

    // If the user provided a question, then print it out
    if let Some(question_text) = question {
        println!("Q: {}\n", question_text)
    }

    // Print info for the primary hexagram
    print!("{}", hexagram_info_pre_changes);

    // Print info for any changing lines
    print_changing_lines_info(&hexagram, hexagram_info_pre_changes);

    // Print info for the relating hexagram (if applicable)
    if let Some(hexagram_info) = hexagram_info_post_changes {
        print!("Changes into:\n\n{}", hexagram_info);
    }
}

fn print_changing_lines_info(hexagram: &Hexagram, hexagram_info: &dyn HexagramInfo) {
    let changing_line_positions = hexagram.get_changing_line_positions();

    if !changing_line_positions.is_empty() {
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Lines are changing! Consider:");
        for line_meaning in hexagram_info.get_line_meanings(&changing_line_positions) {
            print!("\nLine {} changes:\n{}", line_meaning.position, line_meaning.meaning);
        }
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
    }
}
