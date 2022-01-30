// Parses the strin the user input, returns it as a tuple of unsigned 32 bits vector
fn parse_string(good_string: &str, evil_string: &str)-> (Vec<u32>,Vec<u32>) {
    let mut good_races_number_parsed: Vec<u32> = Vec::new();
    let mut evil_races_number_parsed: Vec<u32> = Vec::new();
    for race in good_string.split_whitespace() {
        good_races_number_parsed.push(race.parse().unwrap());
    }
    for race in evil_string.split_whitespace( ){
        evil_races_number_parsed.push(race.parse().unwrap());
    }
    (good_races_number_parsed, evil_races_number_parsed)
}

// Return a string depending who wins or draws
fn display(good_worth: u32, evil_worth: u32) -> String {
    if good_worth > evil_worth {
        format!("Battle Result: Good triumphs over Evil")
    } else if good_worth == evil_worth {
        format!("Battle Result: No victor on this battle field")
    } else {
        format!("Battle Result: Evil eradicates all trace of Good")
    }
}

// TODO: create and Evil implementation 
fn good_vs_evil(good: &str, evil: &str) -> String {
    // turn both good and evil str into a vector of u32 valeues.
    let (good_races_parsed, evil_races_parsed) = parse_string(good, evil);
    //---- Enums
    enum GoodWorth {
        HobbitsWorth,
        MenWorth,
        ElvesWorth,
        DwarvesWorth,
        EaglesWorth,
        WizardsWorth
    }
    enum EvilWorth {
        OrcsWorth,
        MenWorth,
        WargsWorth,
        GoblinsWorth,
        UrukHaiWorth,
        TrollsWorth,
        WizardsWorth
    }

    // both GoodNumber and EvilNumber enum will provide a type to store the
    // number of races of each side
    #[derive(Debug)]
    enum GoodNumber {
        HobbitsNumber(u32),
        MenNumber(u32),
        ElvesNUmber(u32),
        DwarvesNumber(u32),
        EaglesNumber(u32),
        WizardsNumber(u32)
    }
    #[derive(Debug)]
    enum EvilNumber {
        OrcsNumber(u32),
        MenNumber(u32),
        WargsNumber(u32),
        GoblinsNumber(u32),
        Uruk_haiNumber(u32),
        TrollsNumber(u32),
        WizardsNumber(u32)
    }
    //---- Structs
    // Both Good and Evil trais will store each race to keep it all data
    // joined inside a single type.
    #[derive(Debug)]
    struct Good {
        hobbits: GoodNumber,
        men: GoodNumber,
        elves: GoodNumber,
        dwarves: GoodNumber,
        eagles: GoodNumber,
        wizards: GoodNumber
    }
    #[derive(Debug)]
    struct Evil {
        orcs: EvilNumber,
        men: EvilNumber,
        wargs: EvilNumber,
        goblins: EvilNumber,
        uruk_hai: EvilNumber,
        trolls: EvilNumber,
        wizards: EvilNumber
    }
    //---- Imlemenations:

    //---- impl GoodWorth enum 
    impl GoodWorth {
        fn get_worth(&self) -> u32 {
            match self {
                 GoodWorth::HobbitsWorth => 1,
                 GoodWorth::MenWorth => 2,
                 GoodWorth::ElvesWorth => 3,
                 GoodWorth::DwarvesWorth => 3,
                 GoodWorth::EaglesWorth => 4,
                 GoodWorth::WizardsWorth => 10
            }
        }
    }

    //----impl EvilWorth enum
    impl EvilWorth {
        fn get_worth(&self) -> u32 {
            match self {
                EvilWorth::OrcsWorth => 1,
                EvilWorth::MenWorth => 2,
                EvilWorth::WargsWorth => 2,
                EvilWorth::GoblinsWorth => 2,
                EvilWorth::UrukHaiWorth => 3,
                EvilWorth::TrollsWorth => 5,
                EvilWorth::WizardsWorth => 10
            }
        }
    }
    //---- impl GoodNumbers
    impl GoodNumber {
        fn get_number(&self) -> u32 {
            match self {
                GoodNumber::HobbitsNumber(val) => *val,
                GoodNumber::MenNumber(val) => *val,
                GoodNumber::ElvesNUmber(val) => *val,
                GoodNumber::DwarvesNumber(val) => *val,
                GoodNumber::EaglesNumber(val) => *val,
                GoodNumber::WizardsNumber(val) => *val
            }
        }
    }
    //---- impl Good struct
    impl Good {
        fn from(races: Vec<u32>) -> Self {
            let hobbits = GoodNumber::HobbitsNumber(races[0]);
            let men = GoodNumber::MenNumber(races[1]);
            let elves = GoodNumber::ElvesNUmber(races[2]);
            let dwarves = GoodNumber::DwarvesNumber(races[3]);
            let eagles = GoodNumber::EaglesNumber(races[4]);
            let wizards = GoodNumber::WizardsNumber(races[5]);
            Self {
                hobbits,
                men,
                elves,
                dwarves,
                eagles,
                wizards
            }
        }
        fn calculate_worth(&self) -> u32 {
            // Gets every race's number and mutiply by the worth value of each race to get total worth of each one.
            let hobbits_worth = self.hobbits.get_number() * GoodWorth::get_worth(&GoodWorth::HobbitsWorth);
            let men_worth = self.men.get_number() *  GoodWorth::get_worth(&GoodWorth::MenWorth);
            let elves_worth = self.elves.get_number() * GoodWorth::get_worth(&GoodWorth::ElvesWorth);
            let dwarves_worth = self.dwarves.get_number() * GoodWorth::get_worth(&GoodWorth::DwarvesWorth);
            let eagles_worth =self.eagles.get_number() * GoodWorth::get_worth(&GoodWorth::EaglesWorth);
            let wizards_worth = self.wizards.get_number() * GoodWorth::get_worth(&GoodWorth::WizardsNumber);

            // Return the total worth of the Good side
            hobbits_worth + men_worth + elves_worth + dwarves_worth + eagles_worth + wizards_worth
        } 
    }

    //---- impl Evil struct
    impl Evil {

    }

    println!("{:#?}", Good::from(good_races_parsed).calculate_worth());
    "".to_string()
  }

fn main() {
    good_vs_evil("999 0 0 3 6 9", "0 5 9 0");
}
