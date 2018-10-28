use clap::{App, Arg, ArgMatches};
use iching::{
    line::Line,
    Hexagrams,
};

fn main() {
    let matches = App::new("I-Ching")
        .version("1.0")
        .author("Zelda H. <zeldah@pm.me>")
        .about("Ever wish you could know the future? Well now you can!")
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
        .get_matches();

    let mut hexagrams = Hexagrams::new();
    hexagrams.initialize("C:\\projects\\iching\\target\\release\\hexagrams.json")
        .expect("Initialization of hexagrams has failed");

    if matches.is_present("hexagram") {
        print_hexagram_by_number(matches, hexagrams);
    } else {
        println!("Consulting the oracle...");
        for _ in 0..=6 {
            let line: Line = rand::random();
            println!("{}", line);
        }
    }
}

fn print_hexagram_by_number(matches: ArgMatches, hexagrams: Hexagrams) {
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