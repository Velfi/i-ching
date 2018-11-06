use clap::{App, Arg, ArgMatches};
use iching::{
    hexagram::{
        Hexagram,
        HexagramOrdering,
    },
    hexagram_repository::HexagramRepository,
    trigram::TrigramName,
};
use self::hexagram_json::HexagramJson;

mod hexagram_json;

static APP_TITLE: &str = r#"
8888888        .d8888b.  888      d8b
  888         d88P  Y88b 888      Y8P
  888         888    888 888
  888         888        88888b.  888 88888b.   .d88b.
  888         888        888 "88b 888 888 '88b d88P'88b
  888  888888 888    888 888  888 888 888  888 888  888
  888         Y88b  d88P 888  888 888 888  888 Y88b 888
8888888        'Y8888P'  888  888 888 888  888  'Y88888
                                                    888
                                               Y8b d88P
                                                'Y88P'

Version"#;
static APP_DESCRIPTION: &str = r#"
Ever wish you could know the future? Well now you can!
Divine the answers to life's greatest mysteries using
the ancient Chinese method of the I-Ching.

Learn more on Wikipedia:
https://en.wikipedia.org/wiki/I_Ching
"#;

fn main() {
    let matches = start_app_and_get_matches();

    let mut hexagrams = HexagramJson::new();
    hexagrams.initialize().expect("Initialization of hexagrams has failed");

    if matches.is_present("hexagram") {
        print_hexagram_by_number(&matches, &hexagrams);
    } else if matches.is_present("trigram") {
        print_trigram_by_number(&matches);
    } else {
        let user_question = matches.value_of("question");
        print_fortune(user_question, &hexagrams);
    }
}

fn start_app_and_get_matches() -> ArgMatches<'static> {
    App::new(APP_TITLE)
        .version("0.1")
        .author("Zelda H. <zeldah@pm.me>")
        .about(APP_DESCRIPTION)
        .arg(Arg::with_name("question")
            .short("q")
            .long("question")
            .value_name("QUESTION")
            .help("The question that you want answered")
            .takes_value(true))
        .arg(Arg::with_name("hexagram")
            .short("h")
            .long("hexagram")
            .value_name("HEXAGRAM NUMBER")
            .help("Look up a hexagram by number (King Wen sequence)")
            .takes_value(true))
        .arg(Arg::with_name("trigram")
            .short("t")
            .long("trigram")
            .value_name("TRIGRAM NUMBER")
            .help("Look up a trigram by number")
            .takes_value(true))
        .get_matches()
}

fn print_hexagram_by_number(matches: &ArgMatches, hexagrams: &impl HexagramRepository) {
    let hexagram_number_string = matches.value_of("hexagram").unwrap();
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
    let trigram_number_string = matches.value_of("trigram").unwrap();
    let trigram_number_result = trigram_number_string.parse::<usize>();

    match trigram_number_result {
        Ok(trigram_number) => match TrigramName::from_usize(trigram_number) {
            Ok(trigram) => println!("{}", trigram),
            Err(_) => println!("Trigrams number 1 to 8 but you asked for trigram No. {}", trigram_number),
        },
        Err(error) => println!("No trigram for you: {}", error),
    }
}

fn print_fortune(question: Option<&str>, hexagrams: &impl HexagramRepository) {
    let new_hexagram = Hexagram::new();
    let hexagram_number_pre_changes = new_hexagram.as_number(false, HexagramOrdering::KingWen);
    let hexagram_number_post_changes = new_hexagram.as_number(true, HexagramOrdering::KingWen);

    let hexagram_info_pre_changes = hexagrams.get_by_number(
        hexagram_number_pre_changes
    ).expect("Failed to get hexagram info by number (pre)");
    let hexagram_info_post_changes = if hexagram_number_pre_changes != hexagram_number_post_changes {
        hexagrams.get_by_number(hexagram_number_post_changes)
    } else { None };

    if let Some(question_text) = question {
        println!("Q: {}", question_text)
    }

    print!("{}", hexagram_info_pre_changes);

    if let Some(hexagram_info) = hexagram_info_post_changes {
        print!("Changes into:\n\n{}", hexagram_info);
    }
}
