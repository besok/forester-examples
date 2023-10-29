use std::path::PathBuf;
use std::sync::MutexGuard;
use forester_rs::runtime::args::RtArgs;
use forester_rs::runtime::blackboard::BlackBoard;
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::RuntimeError;
use forester_rs::tracer::{Tracer, TracerConfig};
use log::LevelFilter;

pub const R: &str = "result";

pub fn r() -> String {
    R.to_string()
}

pub fn builder(root: &PathBuf) -> ForesterBuilder {
    let mut fb = ForesterBuilder::from_fs();
    fb.main_file("main.tree".to_string());
    fb.root(root.clone());
    fb
}

pub fn root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tree");
    root
}

pub fn tracer(mut root: PathBuf) -> Tracer {
    let config = TracerConfig::in_file(root.join("traces.log"), None);
    Tracer::create(config).expect("fs is fine")
}


pub fn get_result(bb: &MutexGuard<BlackBoard>) -> Result<i64, RuntimeError> {
    bb.get(R.to_string())?
        .and_then(|v| v.clone().as_int())
        .ok_or(err("res"))
}

pub fn get_q(args: RtArgs) -> Result<i64, RuntimeError> {
    args.first()
        .and_then(|v| v.as_int())
        .ok_or(err("q"))
}

pub fn err(field: &str) -> RuntimeError {
    RuntimeError::uex(format!("the {field} is expected"))
}

pub fn turn_on_logs() {
    env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::max())
        .try_init()
        .unwrap();
}