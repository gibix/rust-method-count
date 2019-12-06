
// 1 function
// 1 custom method
// 2 sub module function (pub/private)
//

struct Tstruct {}

impl Tstruct {
    fn tfn() { }
}

trait Ttrait {
    fn trfn();
}

impl Ttrait for Tstruct {
    pub fn trfn() { }
}

mod submod {
    impl Tstruct {
        fn tfn() { }
    }
}

mod extsubmod;

fn main() {
    println!("Hello, world!");
}
