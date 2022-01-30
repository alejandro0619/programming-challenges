// TODO: return two u32 vectors in a tuple
fn parse_string(good_string: &str, evil_string: &str)-> [Vec<u32>, Vec<u32>] {
    let mut good_races_number_parsed: Vec<u32> = Vec::new();
    let mut evil_races_number_parsed: Vec<u32> = Vec::new();
    for race in good_string.split_whitespace() {
        good_races_number_parsed.push(race.parse().unwrap());
    }
    for race in evil_string.split_whitespace( ){
        evil_races_number_parsed.push(race.parse().unwrap());
    }
    [good_races_number_parsed, evil_races_number_parsed]
}

// TODO: create both Good and Evil implementation 
fn good_vs_evil(good: &str, evil: &str) -> String {
    parse_string(good, evil);
    #[derive(Debug)]
    enum GoodNumber {
        HobbitsNumber(u32),
        MenNumber(u32),
        ElvesNUmber(u32),
        DwarvesNumber(u32),
        EaglesNumber(u32),
        WizardsNumber(u32)
    }
    enum EvilNumber {
        OrcsNumber(u32),
        MenNumber(u32),
        WargsNumber(u32),
        GoblinsNumber(u32),
        Uruk_haiNumber(u32),
        TrollsNumber(u32),
        WizardsNumber(u32)
    }
    #[derive(Debug)]
    struct Good {
        hobbits: GoodNumber,
        men: GoodNumber,
        elves: GoodNumber,
        dwarves: GoodNumber,
        eagles: GoodNumber,
        wizards: GoodNumber
    }
    struct Evil {
        orcs: EvilNumber,
        men: EvilNumber,
        wargs: EvilNumber,
        goblins: EvilNumber,
        uruk_hai: EvilNumber,
        trolls: EvilNumber,
        wizards: EvilNumber
    }
    let a = Good {
        hobbits: GoodNumber::HobbitsNumber(5),
        men: GoodNumber::MenNumber(10),
        elves: GoodNumber::ElvesNUmber(4),
        dwarves: GoodNumber::DwarvesNumber(9),
        eagles: GoodNumber::EaglesNumber(0),
        wizards: GoodNumber::WizardsNumber(1),
    };
    println!("{:?}", a);
    "".to_string()
  }

fn main() {
    good_vs_evil("0 0 0 3 6 9", "0 5 9 0");
}
