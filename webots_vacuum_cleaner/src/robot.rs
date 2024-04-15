use std::process::exit;
use std::sync::{Arc, Mutex};
use forester_rs::runtime::{RtResult, RuntimeError};

use forester_webots::*;
use forester_webots::bindings::WbDeviceTag;
use rand::random;

pub type RobotRef = Arc<Mutex<Robot>>;

pub fn new_robot_ref(robot: Robot) -> RobotRef {
    Arc::new(Mutex::new(robot))
}

#[derive(Default, Debug)]
pub struct Robot {
    basic_step: Option<f64>,
    receiver: WbDeviceTag,
    leds: Vec<WbDeviceTag>,
    left_bumper: WbDeviceTag,
    right_bumper: WbDeviceTag,
    cliff_left: WbDeviceTag,
    cliff_front_left: WbDeviceTag,
    cliff_front_right: WbDeviceTag,
    cliff_right: WbDeviceTag,
    left_motor: WbDeviceTag,
    right_motor: WbDeviceTag,
    left_position_sensor: WbDeviceTag,
    right_position_sensor: WbDeviceTag,
}


impl Robot {
    pub fn led_on(&mut self) {
        wb_led_set(self.leds.get(0).unwrap().clone(), 1);
    }

    pub fn get_time_step(&mut self) -> i32 {
        if let Some(b_step) = self.basic_step {
            b_step as i32
        } else {
            let step = wb_robot_get_basic_time_step();
            self.basic_step = Some(step);
            step as i32
        }
    }

    pub fn step(&mut self) {
        if wb_robot_step(self.get_time_step()) == -1 {
            wb_robot_cleanup();
            exit(0);
        }
    }
    pub fn init_devices(&mut self) -> RtResult<()> {
        let receiver = wb_robot_get_device("receiver");
        wb_receiver_enable(receiver, self.get_time_step());
        self.receiver = receiver;

        let leds_names = vec!["led_on", "led_play", "led_step"];

        self.leds = leds_names.iter().map(|name| wb_robot_get_device(name)).collect();

        self.left_bumper = wb_robot_get_device("bumper_left");
        self.right_bumper = wb_robot_get_device("bumper_right");

        wb_touch_sensor_enable(self.left_bumper, self.get_time_step());
        wb_touch_sensor_enable(self.right_bumper, self.get_time_step());


        self.cliff_left = wb_robot_get_device("cliff_left");
        wb_distance_sensor_enable(self.cliff_left, self.get_time_step());
        self.cliff_front_left = wb_robot_get_device("cliff_front_left");
        wb_distance_sensor_enable(self.cliff_front_left, self.get_time_step());
        self.cliff_front_right = wb_robot_get_device("cliff_front_right");
        wb_distance_sensor_enable(self.cliff_front_right, self.get_time_step());
        self.cliff_right = wb_robot_get_device("cliff_right");
        wb_distance_sensor_enable(self.cliff_right, self.get_time_step());

        self.left_motor = wb_robot_get_device("motor_left_wheel");
        self.right_motor = wb_robot_get_device("motor_right_wheel");
        wb_motor_set_position(self.left_motor, f64::INFINITY);
        wb_motor_set_position(self.right_motor, f64::INFINITY);

        wb_motor_set_velocity(self.left_motor, 0.0);
        wb_motor_set_velocity(self.right_motor, 0.0);

        self.left_position_sensor = wb_robot_get_device("left wheel sensor");
        self.right_position_sensor = wb_robot_get_device("right wheel sensor");

        wb_position_sensor_enable(self.left_position_sensor, self.get_time_step());
        wb_position_sensor_enable(self.right_position_sensor, self.get_time_step());
        Ok(())
    }

    pub fn is_there_a_collision_at_left(&self) -> bool {
        wb_touch_sensor_get_value(self.left_bumper) != 0.0
    }
    pub fn is_there_a_collision_at_right(&self) -> bool {
        wb_touch_sensor_get_value(self.right_bumper) != 0.0
    }

    pub fn flush_ir_receiver(&self) {
        while wb_receiver_get_queue_length(self.receiver) > 0 {
            wb_receiver_next_packet(self.receiver);
        }
    }
    pub fn is_there_a_virtual_wall(&self) -> bool {
        wb_receiver_get_queue_length(self.receiver) > 0
    }
    pub fn is_there_a_cliff_at_left(&self) -> bool {
        wb_distance_sensor_get_value(self.cliff_front_left) < 100.0 ||
            wb_distance_sensor_get_value(self.cliff_left) < 100.0
    }
    pub fn is_there_a_cliff_at_right(&self) -> bool {
        wb_distance_sensor_get_value(self.cliff_front_right) < 100.0 ||
            wb_distance_sensor_get_value(self.cliff_right) < 100.0
    }
    pub fn is_there_a_cliff_at_front(&self) -> bool {
        wb_distance_sensor_get_value(self.cliff_front_left) < 100.0 ||
            wb_distance_sensor_get_value(self.cliff_front_right) < 100.0
    }
    pub fn go_forward(&self) {
        wb_motor_set_velocity(self.left_motor, 16f64);
        wb_motor_set_velocity(self.right_motor, 16f64);
    }
    pub fn go_backward(&self) {
        wb_motor_set_velocity(self.left_motor, -8f64);
        wb_motor_set_velocity(self.right_motor, -8f64);
    }
    pub fn stop(&self) {
        wb_motor_set_velocity(self.left_motor, -0f64);
        wb_motor_set_velocity(self.right_motor, -0f64);
    }

    pub fn wait(&mut self, sec: f64) {
        let start_time = wb_robot_get_time();
        while start_time + sec > wb_robot_get_time() {
            self.step()
        }
    }
    pub fn turn(&mut self, angle: f64) {
        self.stop();
        let left_offset = wb_position_sensor_get_value(self.left_position_sensor);
        let right_offset = wb_position_sensor_get_value(self.right_position_sensor);
        self.step();
        let neg = if angle < 0.0 { -1.0 } else { 1.0 };
        println!("turning: neg={neg}, l_offset={left_offset}, r_offset={right_offset}");
        wb_motor_set_velocity(self.left_motor, neg * 8f64);
        wb_motor_set_velocity(self.right_motor, -neg * 8f64);

        let mut orientation = 0.0;

        while orientation < neg * angle {
            let l = wb_position_sensor_get_value(self.left_position_sensor) - left_offset;
            let r = wb_position_sensor_get_value(self.right_position_sensor) - right_offset;

            let dl = 0.031 * l;
            let dr = 0.031 * r;
            orientation = neg * (dl - dr) / 0.271756;
            self.step();
        }
        println!("turning: orientation={orientation}");

        self.stop();
        self.step();
    }
}


