// Uses foldl to exhibit the unchecked block syntax.
use std;

import std::list::*;

// Can't easily be written as a "pure fn" because there's
// no syntax for specifying that f is pure.
fn pure_foldl<@T, @U>(ls: list<T>, u: U, f: block(T, U) -> U) -> U {
    alt ls { nil. { u } cons(hd, tl) { f(hd, pure_foldl(*tl, f(hd, u), f)) } }
}

// Shows how to use an "unchecked" block to call a general
// fn from a pure fn
pure fn pure_length<@T>(ls: list<T>) -> uint {
    fn count<T>(_t: T, u: uint) -> uint { u + 1u }
    unchecked{ pure_foldl(ls, 0u, count) }
}

pure fn nonempty_list<@T>(ls: list<T>) -> bool { pure_length(ls) > 0u }

// Of course, the compiler can't take advantage of the
// knowledge that ls is a cons node. Future work.
// Also, this is pretty contrived since nonempty_list
// could be a "tag refinement", if we implement those.
fn safe_head<@T>(ls: list<T>) : nonempty_list(ls) -> T { car(ls) }

fn main() {
    let mylist = cons(@1u, @nil);
    // Again, a way to eliminate such "obvious" checks seems
    // desirable. (Tags could have postconditions.)
    check (nonempty_list(mylist));
    assert (*safe_head(mylist) == 1u);
}
