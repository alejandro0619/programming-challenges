// Calculate how many months have passed since a given date
 fn calculate_month_until_date(month: &str){
  
   enum MonthsCount<'a> {
     Month(&'a str),
   }
  // The dirtiest way to do this :D
   match MonthsCount::Month(&month.to_lowercase()) {
     MonthsCount::Month("january")   => println!("Have passed 0 months since the beginning of the year"),
     MonthsCount::Month("february")  =>  println!("Have passed 1 months since the beginning of the year"),
     MonthsCount::Month("march")     =>  println!("Have passed 2 months since the beginning of the year"),
     MonthsCount::Month("april")     =>  println!("Have passed 3 months since the beginning of the year"),
     MonthsCount::Month("may")       =>  println!("Have passed 4 months since the beginning of the year"),
     MonthsCount::Month("june")      =>  println!("Have passed 5 months since the beginning of the year"),
     MonthsCount::Month("july")      =>  println!("Have passed 6 months since the beginning of the year"),
     MonthsCount::Month("august")    =>  println!("Have passed 7 months since the beginning of the year"),
     MonthsCount::Month("september") =>  println!("Have passed 8 months since the beginning of the year"),
     MonthsCount::Month("october")   =>  println!("Have passed 9 months since the beginning of the year"),
     MonthsCount::Month("november")  =>  println!("Have passed 10 months since the beginning of the year"),
     MonthsCount::Month("december")  =>  println!("Have passed 11 months since the beginning of the year"),
     MonthsCount::Month(&_)          =>  println!("Enter a valid month")
   }
 }

fn main() {
    calculate_month_until_date("January");
}
