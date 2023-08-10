mod pick;
mod queue;

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
use forester_rs::*;
use log::LevelFilter;
use std::collections::HashMap;
use std::path::PathBuf;

#[macro_use]
extern crate log;

fn main() {}

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
