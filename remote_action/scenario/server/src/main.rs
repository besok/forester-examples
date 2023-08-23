use forester_rs::runtime::action::keeper::ActionImpl;
use forester_rs::runtime::action::Action;
use forester_rs::runtime::args::RtArgs;
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::forester::Forester;
use forester_rs::runtime::rtree::builder::RtTreeBuilder;
use forester_rs::runtime::rtree::builder::*;
use forester_rs::runtime::rtree::rnode::FlowType;
use forester_rs::runtime::rtree::rnode::{RNode, RNodeName};
use forester_rs::runtime::trimmer::task::{RtTreeTrimTask, TrimTask};
use forester_rs::runtime::trimmer::{RequestBody, TreeSnapshot, TrimRequest};
use forester_rs::runtime::RtResult;
use forester_rs::simulator::actions::SimAction;
use forester_rs::tracer::{Event, Tracer, TracerConfig};
use forester_rs::visualizer::Visualizer;
use log::LevelFilter;
use std::path::PathBuf;

#[macro_use]
extern crate log;

fn main() {
    turn_on_logs();

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut root = root.parent().unwrap().to_path_buf();
    root.push("tree");

    let mut fb = forester_builder(&mut root);

    let mut forester = fb.build_with(|| success()).unwrap();

    vis(root.clone().join("main.svg"), &mut forester);

    // println!("{:?}", forester.run_until(Some(10)));
    //
    // vis(root.clone().join("after.svg"), &mut forester);
}

fn success() -> ActionImpl {
    ActionImpl::Present(Action::Sync(Box::new(SimAction::Success(100))))
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

fn turn_on_logs() {
    env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::max())
        .try_init()
        .unwrap();
}

fn tracer(mut root: &mut PathBuf) -> Tracer {
    let file = root.clone().join("trace.log");
    let config = TracerConfig::in_file(file, Some(TracerConfig::default_dt_fmt()));
    Tracer::create(config).unwrap()
}
