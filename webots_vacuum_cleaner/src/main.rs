use std::f64::consts::PI;
use std::path::PathBuf;
use forester_rs::runtime::action::Impl;
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::RtResult;
use forester_rs::tracer::{Tracer, TracerConfig};

use crate::actions::{CollisionChecker, Init, Moving, Step, Turning, Waiting};
use crate::robot::{new_robot_ref, Robot};


mod robot;
mod actions;

fn main() -> RtResult<()> {
    let mut forester = init_fb().build()?;
    println!("The Forester is ready");
    let res = forester.run()?;
    println!("{:?}", res);
    Ok(())
}

fn init_fb() -> ForesterBuilder {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    root.push("webots_vacuum_cleaner");
    let mut fb = ForesterBuilder::from_fs();
    fb.main_file("irobot.tree".to_string());
    fb.root(root.clone());

    let robot = new_robot_ref(Robot::default());
    fb.register_sync_action("init_robot", Init(robot.clone()));
    fb.register_sync_action("wait", Waiting(robot.clone()));
    fb.register_sync_action("collision", CollisionChecker(robot.clone()));
    fb.register_sync_action("turn", Turning(robot.clone()));
    fb.register_sync_action("move", Moving(robot.clone()));
    fb.register_sync_action("step", Step(robot.clone()));

    fb.tracer(tracer());
    fb
}

fn tracer() -> Tracer {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("");
    root.push("tracer.log");
    Tracer::create(TracerConfig::in_file(root, None)).unwrap()
}