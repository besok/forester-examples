use forester_rs::runtime::action::{Impl, Tick};
use forester_rs::runtime::args::RtArgs;
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::{RuntimeError, TickResult};
use forester_webots::wb_robot_init;
use rand::Rng;
use crate::robot::RobotRef;

pub struct Init(pub RobotRef);

impl Impl for Init {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        ctx.trace("Rust controller of the iRobot Create robot started".to_string())?;
        ctx.trace(format!("The robot is {}", if wb_robot_init() > 0 { "ready" } else { "not ready" }))?;
        let mut robot = &mut self.0.lock()?;
        robot.init_devices();
        robot.led_on();
        ctx.trace("The devices are initialized!".to_string())?;
        Ok(TickResult::success())
    }
}


pub struct CollisionChecker(pub RobotRef);

impl Impl for CollisionChecker {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let mut robot = &self.0.lock()?;
        let target = args
            .first()
            .and_then(|v| v.cast(ctx.clone()).str().ok())
            .flatten()
            .ok_or(RuntimeError::fail("target is absent".to_owned()))?;

        match target.as_str() {
            "wall" => {
                if robot.is_there_a_virtual_wall() {
                    ctx.trace("Virtual wall detected!".to_string())?;
                    println!("Virtual wall detected!");
                    Ok(TickResult::success())
                } else {
                    Ok(TickResult::failure("no walls here".to_string()))
                }
            }
            "left" => {
                if robot.is_there_a_collision_at_left() || robot.is_there_a_cliff_at_left() {
                    println!("The obstacle at left is detected!");
                    ctx.trace("The obstacle at left is detected!".to_string())?;
                    Ok(TickResult::success())
                } else {
                    Ok(TickResult::failure("no collisions here".to_string()))
                }
            }
            "right" => {
                if robot.is_there_a_cliff_at_right() || robot.is_there_a_collision_at_right() {
                    println!("The obstacle at right is detected!");
                    ctx.trace("The obstacle at right is detected!".to_string())?;
                    Ok(TickResult::success())
                } else {
                    Ok(TickResult::failure("no collisions here".to_string()))
                }
            }
            "front" => {
                if robot.is_there_a_cliff_at_front() {
                    println!("The obstacle at front is detected!");
                    ctx.trace("The obstacle at front is detected!".to_string())?;
                    Ok(TickResult::success())
                } else {
                    Ok(TickResult::failure("no collisions here".to_string()))
                }
            }
            e => Err(RuntimeError::fail(format!("target is not expected {e}")))
        }
    }
}

pub struct Moving(pub RobotRef);

impl Impl for Moving {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let mut robot = &self.0.lock()?;
        let target = args
            .first()
            .and_then(|v| v.cast(ctx.clone()).str().ok())
            .flatten()
            .ok_or(RuntimeError::fail("target is absent".to_owned()))?;

        match target.as_str() {
            "forward" => {
                robot.go_forward();
                Ok(TickResult::success())
            }
            "backward" => {
                robot.go_backward();
                Ok(TickResult::success())
            }
            "stop" => {
                robot.stop();
                Ok(TickResult::success())
            }

            e => Err(RuntimeError::fail(format!("target is not expected {e}")))
        }
    }
}

pub struct Turning(pub RobotRef);

impl Impl for Turning {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let mut robot = &mut self.0.lock()?;
        let angle = args
            .first()
            .and_then(|v| v.cast(ctx.clone()).float().ok())
            .flatten()
            .ok_or(RuntimeError::fail("angle is absent".to_owned()))?;
        let with_rand =
            args.find_or_ith("with_random".to_string(), 1)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
        let angle = if with_rand {
            let mut rng = rand::thread_rng();
            let multi: f64 = rng.gen();
            angle * multi
        } else {
            angle
        };
        println!("turning on {angle}");
        robot.turn(angle);
        Ok(TickResult::success())
    }
}

pub struct Waiting(pub RobotRef);

impl Impl for Waiting {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let mut robot = &mut self.0.lock()?;
        let wait = args
            .first()
            .and_then(|v| v.cast(ctx.clone()).float().ok())
            .flatten()
            .ok_or(RuntimeError::fail("wait is absent".to_owned()))?;

        robot.wait(wait);
        Ok(TickResult::success())
    }
}

pub struct Step(pub RobotRef);

impl Impl for Step {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let mut robot = &mut self.0.lock()?;
        robot.flush_ir_receiver();
        robot.step();
        Ok(TickResult::success())
    }
}