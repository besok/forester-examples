use std::f64::consts::PI;
use std::path::PathBuf;
use forester_rs::runtime::action::Impl;
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::tracer::{Tracer, TracerConfig};

use crate::actions::{CollisionChecker, Init, Moving, Step, Turning, Waiting};
use crate::robot::{new_robot_ref, randdouble, Robot};


mod robot;
mod actions;

fn main() {
    let mut forester = init_fb().build().unwrap();
    println!("The Forester is ready");
    let res = forester.run();
    println!("{:?}", res);
    // let mut controller = Robot::default();
    // controller.init_devices();
    // controller.led_on();
    // controller.wait(0.5f64);
    //
    // loop {
    //     if controller.is_there_a_virtual_wall() {
    //         println!("Virtual wall detected! Turning...");
    //         controller.turn(PI)
    //     } else if controller.is_there_a_collision_at_left()
    //         || controller.is_there_a_cliff_at_left() {
    //         println!("The obstacle at left is detected! Turning...");
    //         controller.go_backward();
    //         controller.wait(0.5f64);
    //         controller.turn(randdouble() * PI);
    //     }
    //     else if controller.is_there_a_cliff_at_right()
    //         || controller.is_there_a_cliff_at_front()
    //         || controller.is_there_a_collision_at_right(){
    //         println!("The obstacle at right is detected! Turning...");
    //         controller.go_backward();
    //         controller.wait(0.5f64);
    //         controller.turn(-randdouble() * PI);
    //     }
    //     else {
    //         controller.go_forward();
    //     }
    //     controller.flush_ir_receiver();
    //     controller.step();
    // }
}

fn init_fb() -> ForesterBuilder {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    root.push("bt");
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