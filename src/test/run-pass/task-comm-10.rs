// xfail-stage0
// xfail-stage1
// xfail-stage2

use std;
import std::task;

fn start(chan[chan[str]] c) {
    let port[str] p;

    p = port();
    c <| chan(p);
    p |> a;
    log_err a;
    p |> b;
    log_err b;

    auto a;
    auto b;
}

fn main() {
    let port[chan[str]] p;
    auto child;

    p = port();
    child = spawn start(chan(p));

    p |> c;
    c <| "A";
    c <| "B";
    task::yield();

    auto c;
}
