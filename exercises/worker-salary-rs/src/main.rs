mod get_input;

fn main() {
   let (worked_hours, cost_per_hour) = get_input::get_user_input();
   let payment = get_input::calculate_total_payment(worked_hours, cost_per_hour);
   get_input::display(payment)
}
