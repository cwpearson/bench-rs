extern crate bench;

use bench::Bencher;


fn routine(b: &mut Bencher) {
    // Setup (construct data, allocate memory, etc)

    b.iter(|| {
        let s = String::new();
    })

    // Teardown (free resources)
}

pub fn main() {
    let mut b = Bencher::default("ex1");

    routine(&mut b);


}