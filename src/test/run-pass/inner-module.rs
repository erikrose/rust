


// -*- rust -*-
mod inner {
    mod inner2 {
        fn hello() { log "hello, modular world"; }
    }
    fn hello() { inner2::hello(); }
}

fn main() { inner::hello(); inner::inner2::hello(); }
