use forester_rs::runtime::action::builtin::remote::RemoteHttpAction;
use forester_rs::runtime::action::keeper::ActionImpl;
use forester_rs::runtime::action::{Action, Impl, Tick};
use forester_rs::runtime::args::{RtArgs, RtValue};
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::forester::Forester;
use forester_rs::runtime::rtree::builder::RtTreeBuilder;
use forester_rs::runtime::rtree::builder::*;
use forester_rs::runtime::rtree::rnode::FlowType;
use forester_rs::runtime::rtree::rnode::{RNode, RNodeName};
use forester_rs::runtime::trimmer::task::{RtTreeTrimTask, TrimTask};
use forester_rs::runtime::trimmer::{RequestBody, TreeSnapshot, TrimRequest};
use forester_rs::runtime::{RtResult, TickResult};
use forester_rs::simulator::actions::SimAction;
use forester_rs::tracer::{Event, Tracer, TracerConfig};
use forester_rs::visualizer::Visualizer;
use log::LevelFilter;
use std::path::PathBuf;

#[macro_use]
extern crate log;

fn main() {
    let mut root = root();

    let mut fb = forester_builder(&mut root);

    fb.register_remote_action(
        "calculate",
        RemoteHttpAction::new("http://localhost:10000/calculate".to_string()),
    );
    fb.register_remote_action(
        "move_to",
        RemoteHttpAction::new("http://localhost:10001/move_to".to_string()),
    );

    fb.http_serv(9000);

    let mut forester = fb.build().unwrap();

    println!("{:?}", forester.run_until(Some(100)));
    // vis(root.join("tree.svg"), &mut forester);
}

fn root() -> PathBuf {
    let mut r = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    r.push("tree");
    r
}

fn vis(path: PathBuf, forester: &mut Forester) {
    Visualizer::rt_tree_svg_to_file(&forester.tree, path).unwrap();
}

fn forester_builder(mut root: &mut PathBuf) -> ForesterBuilder {
    let mut fb = ForesterBuilder::from_fs();
    fb.main_file("main.tree".to_string());
    fb.root(root.clone());
    fb.tracer(tracer(&mut root));
    fb
}

fn tracer(mut root: &mut PathBuf) -> Tracer {
    let file = root.clone().join("trace.log");
    let config = TracerConfig::in_file(file, Some(TracerConfig::default_dt_fmt()));
    Tracer::create(config).unwrap()
}
