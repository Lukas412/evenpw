use clap::{value_parser, Arg, ArgAction, Command};
use german_querz::GermanQuertzLayout;
use rand::{seq::SliceRandom, Rng};

struct Settings {
    length: usize,
    hand_switch: usize,
    shift_hand_switch: usize,
    shifting_ratio: f64,
    special_ratio: f64,
}

trait PwGenerator {
    fn keys(hand: bool, shift: bool, special: bool) -> &'static [char];

    fn generate(buffer: &mut String, settings: &Settings) {
        let mut random = rand::thread_rng();

        let mut shifting = random.gen_bool(settings.shifting_ratio);
        let mut hand = random.gen_bool(0.5);
        let mut chars_with_hand = 0;

        for _ in 0..settings.length {
            let special = random.gen_bool(settings.special_ratio);
            let key = Self::keys(hand, shifting, special)
                .choose(&mut random)
                .expect("expect possible keys to be not empty");

            buffer.push(*key);

            chars_with_hand += 1;

            let should_switch_hands = !shifting && chars_with_hand == settings.hand_switch
                || shifting && chars_with_hand == settings.shift_hand_switch;
            if should_switch_hands {
                hand = !hand;
                chars_with_hand = 0;
                shifting = random.gen_bool(settings.shifting_ratio);
            }
        }
    }
}

mod german_querz;

fn main() {
    let cli = Command::new("evenpw")
        .about("generate easy to type passwords")
        .args([
            Arg::new("length")
                .short('l')
                .long("length")
                .value_parser(value_parser!(usize))
                .help("the length of the password"),
            Arg::new("count")
                .short('n')
                .long("count")
                .value_parser(value_parser!(usize))
                .help("the number of passwords to generate"),
            Arg::new("copy")
                .short('c')
                .long("copy")
                .help("copy the last password to the clipboard"),
            Arg::new("hand-switch")
                .short('s')
                .long("hand-switch")
                .help("switch hand after keypresses (default is 1)"),
            Arg::new("shift-hand-switch")
                .short('w')
                .long("shift-hand-switch")
                .help("switch hand after keypresses when shift is pressed (default is 2)"),
            Arg::new("german-qwertz")
                .long("german-qwertz")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("keyboard layout to generate the password for"),
        ])
        .get_matches();

    let length: usize = cli.get_one("length").copied().unwrap_or(16);
    let count: usize = cli.get_one("count").copied().unwrap_or(1);

    let mut german_querz = cli.get_flag("german-qwertz");
    if !german_querz {
        german_querz = true;
    }

    let settings = Settings {
        length,
        hand_switch: 1,
        shift_hand_switch: 2,
        shifting_ratio: 1. / 3.,
        special_ratio: 1. / 4.,
    };

    let mut buffer = String::with_capacity(length);
    for _ in 0..count {
        buffer.clear();

        if german_querz {
            GermanQuertzLayout::generate(&mut buffer, &settings)
        }

        println!("{}", buffer);
    }
}
