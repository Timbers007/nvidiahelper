use std::io::{Result};
use std::process::{Command, Output};

pub struct Environment {
    pub(crate) xauthority: String,
    pub(crate) display: String,
    pub(crate) debug:bool,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {xauthority:String::from("/run/user/1000/gdm/Xauthority"), display:String::from(":0"), debug: false}
    }
}

pub fn execute(env: &Environment, cmd: &String) -> Result<Output> {
    let cmd_output_opt = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();

    match cmd_output_opt {
        Ok(output) => {
            if env.debug && output.status.code().map_or(false, |code| code != 0) {
                let code = output.status.code().unwrap();
                println!("Failed to execute command: {}", cmd);
                println!("Status Code: {} - {}", code, get_smi_ret_message(code))
            }

            Ok(output)
        },
        Err(e) => {
            if env.debug {
                println!("Failed to execute command: {}", cmd);
                println!("    Error: {}", e.to_string());
            }

            Err(e)
        }
    }
}

//Work in progress for nvidia-smi return codes
pub fn get_smi_ret_message(x: i32) -> &'static str {
    match x {
        0 => { "Successfully executed" },
        2 => { "Argument was invalid" },
        3 => { "Operation is not available on device" }
        4 => { "Insufficient permission" }
        6 => { "Unable to query" }
        8 => { "VGA power cable error" }
        9 => { "Driver error" }
        10 => { "GPU interrupt error" }
        12 => { "NVML library unavailable" }
        13 => { "NVML library does not support operation" }
        14 => { "infoROM is corrupted" }
        15 => { "GPU is inaccessible due to an error" }
        255 => { "Driver or other error related to GPU" }
        _ => { "There was an unknown error" }
    }
}
