mod utils;
mod actions;
mod daemons;

use forester_rs::runtime::action::{Impl, Tick};
use forester_rs::runtime::args::RtValue;
use forester_rs::runtime::env::daemon::Daemon;
use crate::actions::*;
use crate::daemons::HttpListener;

use crate::utils::{builder, root, tracer, turn_on_logs};


fn main() {
    // turn_on_logs();

    let root = root();
    let mut fb = builder(&root);

    fb.bb_load("bb_load.json".to_string());
    fb.tracer(tracer(root.clone()));

    fb.register_sync_action("add", Add);
    fb.register_sync_action("sub", Sub);
    fb.register_sync_action("mul", Mul);

    fb.register_named_daemon("http_watcher".to_string(),
                             Daemon::a_sync(HttpListener(root.join("data.txt"))),
    );

    let mut forester = fb.build().unwrap();

    let result = forester.run();

    println!("{:?}", result);
}


