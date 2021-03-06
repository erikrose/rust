// -*- rust -*-

use std;
import std::task;
import std::comm::*;

fn main() {
    let p = port();
    let y: int;

    task::spawn(bind child(chan(p)));
    y = recv(p);
    log "received 1";
    log y;
    assert (y == 10);

    task::spawn(bind child(chan(p)));
    y = recv(p);
    log "received 2";
    log y;
    assert (y == 10);
}

fn child(c: chan<int>) { send(c, 10); }
