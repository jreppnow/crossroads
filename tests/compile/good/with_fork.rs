use crossroads::crossroads;

#[crossroads]
fn with_fork() {
    match fork!() {
        a => {}
        b => {}
    }
}

fn main() {}
