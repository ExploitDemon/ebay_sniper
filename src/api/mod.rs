mod end_date;

use rocket::{launch, routes};
use end_date::get_end_date;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![get_end_date])
}
