mod utils;
mod actions;
mod daemons;

use forester_rs::runtime::action::{Impl, Tick};
use forester_rs::runtime::env::daemon::Daemon;
use crate::actions::*;
use crate::daemons::HttpListener;

use crate::utils::{builder, r, root, tracer, turn_on_logs};


fn main() {
    let root = root();
    let mut fb = builder(&root);
    fb.tracer(tracer(root.clone()));
    fb.bb_load("bb_load.json".to_string());

    fb.register_sync_action("add", Add);
    fb.register_sync_action("sub", Sub);
    fb.register_sync_action("mul", Mul);

    fb.register_named_daemon(
        "http_watcher".to_string(),
        Daemon::a_sync(HttpListener),
    );

    let result = fb.build().unwrap().run();
    println!("{:?}", result);

}


