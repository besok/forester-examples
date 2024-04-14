use std::process::exit;
use std::sync::{Arc, Mutex};

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
    receiver: Option<WbDeviceTag>,
    leds: Vec<WbDeviceTag>,
    left_bumper: Option<WbDeviceTag>,
    right_bumper: Option<WbDeviceTag>,
    cliff_left: Option<WbDeviceTag>,
    cliff_front_left: Option<WbDeviceTag>,
    cliff_front_right: Option<WbDeviceTag>,
    cliff_right: Option<WbDeviceTag>,
    left_motor: Option<WbDeviceTag>,
    right_motor: Option<WbDeviceTag>,
    left_position_sensor: Option<WbDeviceTag>,
    right_position_sensor: Option<WbDeviceTag>,
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
    pub fn init_devices(&mut self) {
        let receiver = wb_robot_get_device("receiver");
        wb_receiver_enable(receiver, self.get_time_step());
        self.receiver = Some(receiver);

        let leds_names = vec!["led_on", "led_play", "led_step"];

        self.leds = leds_names.iter().map(|name| wb_robot_get_device(name)).collect();

        self.left_bumper = Some(wb_robot_get_device("bumper_left"));
        self.right_bumper = Some(wb_robot_get_device("bumper_right"));
        wb_touch_sensor_enable(self.left_bumper.unwrap(), self.get_time_step());
        wb_touch_sensor_enable(self.right_bumper.unwrap(), self.get_time_step());


        self.cliff_left = Some(wb_robot_get_device("cliff_left"));
        wb_distance_sensor_enable(self.cliff_left.unwrap(), self.get_time_step());
        self.cliff_front_left = Some(wb_robot_get_device("cliff_front_left"));
        wb_distance_sensor_enable(self.cliff_front_left.unwrap(), self.get_time_step());
        self.cliff_front_right = Some(wb_robot_get_device("cliff_front_right"));
        wb_distance_sensor_enable(self.cliff_front_right.unwrap(), self.get_time_step());
        self.cliff_right = Some(wb_robot_get_device("cliff_right"));
        wb_distance_sensor_enable(self.cliff_right.unwrap(), self.get_time_step());

        self.left_motor = Some(wb_robot_get_device("motor_left_wheel"));
        self.right_motor = Some(wb_robot_get_device("motor_right_wheel"));
        wb_motor_set_position(self.left_motor.unwrap(), f64::INFINITY);
        wb_motor_set_position(self.right_motor.unwrap(), f64::INFINITY);

        wb_motor_set_velocity(self.left_motor.unwrap(), 0.0);
        wb_motor_set_velocity(self.right_motor.unwrap(), 0.0);

        self.left_position_sensor = Some(wb_robot_get_device("left wheel sensor"));
        self.right_position_sensor = Some(wb_robot_get_device("right wheel sensor"));

        wb_position_sensor_enable(self.left_position_sensor.unwrap(), self.get_time_step());
        wb_position_sensor_enable(self.right_position_sensor.unwrap(), self.get_time_step());
    }

    pub fn is_there_a_collision_at_left(&self) -> bool {
        wb_touch_sensor_get_value(self.left_bumper.unwrap()) != 0.0
    }
    pub fn is_there_a_collision_at_right(&self) -> bool {
        wb_touch_sensor_get_value(self.right_bumper.unwrap()) != 0.0
    }

    pub fn flush_ir_receiver(&self) {
        while wb_receiver_get_queue_length(self.receiver.unwrap()) > 0 {
            wb_receiver_next_packet(self.receiver.unwrap());
        }
    }
    pub fn is_there_a_virtual_wall(&self) -> bool {
        wb_receiver_get_queue_length(self.receiver.unwrap()) > 0
    }
    pub fn is_there_a_cliff_at_left(&self) -> bool {
        wb_distance_sensor_get_value(self.cliff_front_left.unwrap()) < 100.0 ||
            wb_distance_sensor_get_value(self.cliff_left.unwrap()) < 100.0
    }
    pub fn is_there_a_cliff_at_right(&self) -> bool {
        wb_distance_sensor_get_value(self.cliff_front_right.unwrap()) < 100.0 ||
            wb_distance_sensor_get_value(self.cliff_right.unwrap()) < 100.0
    }
    pub fn is_there_a_cliff_at_front(&self) -> bool {
        wb_distance_sensor_get_value(self.cliff_front_left.unwrap()) < 100.0 ||
            wb_distance_sensor_get_value(self.cliff_front_right.unwrap()) < 100.0
    }
    pub fn go_forward(&self) {
        wb_motor_set_velocity(self.left_motor.unwrap(), 16f64);
        wb_motor_set_velocity(self.right_motor.unwrap(), 16f64);
    }
    pub fn go_backward(&self) {
        wb_motor_set_velocity(self.left_motor.unwrap(), -8f64);
        wb_motor_set_velocity(self.right_motor.unwrap(), -8f64);
    }
    pub fn stop(&self) {
        wb_motor_set_velocity(self.left_motor.unwrap(), -0f64);
        wb_motor_set_velocity(self.right_motor.unwrap(), -0f64);
    }

    pub fn wait(&mut self, sec: f64) {
        let start_time = wb_robot_get_time();
        while start_time + sec > wb_robot_get_time() {
            self.step()
        }
    }
    pub fn turn(&mut self, angle: f64) {
        println!("turning: angle={angle}");
        self.stop();
        let left_offset = wb_position_sensor_get_value(self.left_position_sensor.unwrap());
        let right_offset = wb_position_sensor_get_value(self.right_position_sensor.unwrap());
        self.step();
        let neg = if angle < 0.0 { -1.0 } else { 1.0 };
        println!("turning: neg={neg}, l_offset={left_offset}, r_offset={right_offset}");
        wb_motor_set_velocity(self.left_motor.unwrap(), neg * 8f64);
        wb_motor_set_velocity(self.right_motor.unwrap(), -neg * 8f64);

        let mut orientation = 0.0;

        while orientation < neg * angle {
            let l = wb_position_sensor_get_value(self.left_position_sensor.unwrap()) - left_offset;
            let r = wb_position_sensor_get_value(self.right_position_sensor.unwrap()) - right_offset;

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

pub fn randdouble() -> f64 {
    random::<f64>()
}


