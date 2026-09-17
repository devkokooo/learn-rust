use std::collections::HashMap;
use restaurant::{customer, hosting};

use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    hosting::add_to_waitlist();
    customer::eat_at_restaurant();
}

fn function1() -> Result {
    todo!();
}

fn function2() -> IoResult<()> {
    todo!();
}
