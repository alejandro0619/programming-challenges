// TODO: return two u32 vectors in a tuple
fn parse_string(good_string: &str, evil_string: &str) {
    let good_vec: Vec<&str> = good_string.split_whitespace().collect();
    let evil_vec: Vec<&str> = evil_string.split_whitespace().collect();
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
        Hobbits: GoodNumber,
        Men: GoodNumber,
        Elves: GoodNumber,
        Dwarves: GoodNumber,
        Eagles: GoodNumber,
        Wizards: GoodNumber
    }
    struct Evil {
        Orcs: EvilNumber,
        Men: EvilNumber,
        Wargs: EvilNumber,
        Goblins: EvilNumber,
        UrukHai: EvilNumber,
        Trolls: EvilNumber,
        Wizards: EvilNumber
    }
    let a = Good {
        Hobbits: GoodNumber::HobbitsNumber(5),
        Men: GoodNumber::MenNumber(10),
        Elves: GoodNumber::ElvesNUmber(4),
        Dwarves: GoodNumber::DwarvesNumber(9),
        Eagles: GoodNumber::EaglesNumber(0),
        Wizards: GoodNumber::WizardsNumber(1),
    };
    println!("{:?}", a);
    "".to_string()
  }

fn main() {
    good_vs_evil("0 0 0 3 6 9", "0 5 9 0");
}
