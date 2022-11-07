mod executor;
mod nvidiahelper;
mod nvidiagpu;
mod constants;

use std::env;
use std::collections::HashMap;
use std::process::Output;
use std::io::{Result};

use crate::executor::{execute, Environment};
use crate::nvidiahelper::{HelperCommand, new_command};

/*
 * The first section will generally work on systems like Ubuntu or Arch Linux
 * However, on some systems running CentOS, Fedora, and others it will not give
 * the .Xauthority file location. Therefore, we will have to query for it through
 * the ps command to find the Xorg running process and its arguments.
 *
 * If the .Xauthority file is located under the home directory of the user, then
 * nvidia-settings seems to ignore whatever we send to it through the command line and
 * uses that file instead.
 *
 * The default value of /run/user/1000/gdm/Xauthority will not be removed if neither
 * command can come up with something.
 */
fn get_xauthority(env: &mut Environment) {
    let sysctl_env_out = execute(env, &String::from("systemctl --user show-environment | grep XAUTHORITY=")).unwrap();
    let sysctl_env_out_str = String::from_utf8(sysctl_env_out.stdout).unwrap();

    if sysctl_env_out_str.contains("XAUTHORITY=") {
        env.xauthority = sysctl_env_out_str.chars().skip(11).take(sysctl_env_out_str.len() - 12).collect();
    } else {
        let sysctl_env_out = execute(env, &String::from("ps a | grep /Xauthority")).unwrap();
        let sysctl_env_out_str = String::from_utf8(sysctl_env_out.stdout).unwrap();

        match sysctl_env_out_str.find("/gdm/Xauthority") {
            Some(i) => {
                let mut index = i;

                while sysctl_env_out_str.chars().nth(index - 1).unwrap() != ' ' {
                    index -= 1;
                }

                env.xauthority = sysctl_env_out_str.chars().skip(index).take((i - index) + 15).collect();
            },
            None => {
                //Keep environment xauthority value as its default
            }
        }
    }
}

fn get_display(env: &mut Environment) {
    let sysctl_env_out = execute(env, &String::from("systemctl --user show-environment | grep DISPLAY=")).unwrap();
    let sysctl_env_out_str = String::from_utf8(sysctl_env_out.stdout).unwrap();

    if sysctl_env_out_str.contains("DISPLAY=") {
        let len = sysctl_env_out_str.len();
        env.display = sysctl_env_out_str.chars().skip(8).take(len - 9).collect();
    }

    //if not then we leave it as its default of :0 which is a pretty good guess
}

fn cmd_exists<'a>(arg: &'a String, commands: &'a HashMap<String, HelperCommand>) -> Option<&'a HelperCommand> {
    return match commands.get(arg) {
        Some(x) => {
            Some(x)
        },
        None => {
            match check_alias(arg, &commands) {
                Some(x) => {
                    Some(x)
                },
                None => {
                    None
                }
            }
        }
    }
}

fn check_alias<'a>(arg: &'a String, args: &'a HashMap<String, HelperCommand>) -> Option<&'a nvidiahelper::HelperCommand> {
    for x in args.values() {
        if x.aliases.contains(arg) {
            return Some(x);
        }
    }

    return None;
}

fn debug_message(env: &Environment, out: Result<Output>, operation: &'static str) {
    if env.debug {
        match out {
            Ok(o) => {
                println!("{}", String::from_utf8_lossy(&o.stdout));
                println!("{}", String::from_utf8_lossy(&o.stderr));
            }
            Err(err) => {
                println!("There was a problem setting this GPU's {}. Error: {}", operation, err);
            }
        }
    }
}

fn run(cmd: &HelperCommand, args: &Vec<&String>, env: &mut Environment, gpu: &mut usize) {
    if !cmd.args.contains(&args.len()) { println!("'{}' does not accept {} arguments. See 'help' for more information.", cmd.name, args.len()); return; }

    if cmd.name.eq("help") {
        println!("----- NVIDIA GPU Terminal Helper ----- v{} -----\n", constants::BUILD_VERSION);
        println!("Execute Command Format: ./nvidiahelper argument1 arg1value1 argument2 arg2value1 arg2value2");
        println!("Further, [argument] will represent an argument that is required. () is optional. Omit [] and/or () when you execute the command.\n");
        println!("GPU Control Arguments:");
        println!("  gpu [gpu_id]");
        println!("        Optional. Sets the GPU to adjust settings for. Defaults to GPU 0.\n");
        println!("  fan [fan_id] [fan_speed]");
        println!("        Sets the GPU fan at position fan_id to speed fan_speed.\n");
        println!("  clock [speed]");
        println!("        Sets the GPU core clock speed to speed.\n");
        println!("  memory [speed]");
        println!("        Sets the GPU memory clock speed to speed. \n");
        println!("  memoryoffset [speed] (power level)");
        println!("        Sets the GPU memory clock speed offset to speed. Overclocks or underclocks memory.\n");
        println!("  clockoffset [speed] (power level)");
        println!("        Sets the GPU core clock clock offset speed to speed. Overclocks or underclocks core.\n");
        println!("  power [watts]");
        println!("        Limits the GPU to only be able to pull at most the specified watts.\n");
        println!("  resetall");
        println!("        Resets all settings to their defaults. \n");
        println!("Advanced Options (Optional):\n");
        println!("  display [display_id]");
        println!("        Sets the Xorg display value to be passed into nvidia-settings. This is automatic if none is specified.\n");
        println!("  xauth [Xauthority path]");
        println!("        Sets the Xauthority file path to be passed into nvidia-settings. This is automatic if none is specified.\n");
        println!("  debug true");
        println!("        Shows output of all executions from this program. Will be detailed.\n\n");
        println!();
        println!("Example: ./nvidiahelper fan 0 75 fan 1 75 clockoffset 150 memoryoffset 500 power 400")
    } else if cmd.name.eq("version") {
        println!("NVIDIAHelper by Tim b{}", constants::BUILD_VERSION);
        println!("https://github.com/Timbers007/nvidiahelper")
    } else if cmd.name.eq("display") {
        env.display = args[0].clone();
    } else if cmd.name.eq("xauth") {
        env.xauthority = args[0].clone();
    } else if cmd.name.eq("debug") {
        match args[0].parse::<bool>() {
            Ok(n) => {
                env.debug = n;
                if env.debug { println!("Successfully set debug mode to {}", env.debug) }
            },
            Err(_) => {
                println!("{} is not true or false. {} must be set equal to true or false.", args[0], cmd.name);
            }
        }
    } else if cmd.name.eq("gpu") {
        match args[0].parse::<usize>() {
            Ok(n) => {
                *gpu = n;
                if env.debug { println!("Successfully set current GPU to {}.", gpu) }
            },
            Err(_) => {
                println!("{} is not an integer greater than or equal to 0.", args[0]);
            }
        }
    } else if cmd.name.eq("fan") {
        let mut fan_index = 0;
        let mut fan_speed = 0;

        if args.len() == 1 {
            match args[0].parse::<i32>() {
                Ok(n) => {
                    fan_speed = n;
                },
                Err(_) => {
                    println!("Failed to set fan speed. {} is not an integer greater than or equal to 0.", args[0]);
                    return;
                }
            }
        } else if args.len() == 2 {
            match args[0].parse::<usize>() {
                Ok(n) => {
                    fan_index = n;
                },
                Err(_) => {
                    println!("Failed to set fan speed. {} is not an integer greater than or equal to 0.", args[0]);
                    return;
                }
            }

            match args[1].parse::<i32>() {
                Ok(n) => {
                    fan_speed = n;
                },
                Err(_) => {
                    println!("Failed to set fan speed. {} is not an integer greater than or equal to 0.", args[0]);
                    return;
                }
            }
        }

        if (fan_speed < 0 || fan_speed > 100) && (fan_speed != -1) {
            println!("Failed to set fan speed. {} is not an integer between 0 and 100.", fan_speed);
        }

        if fan_speed == -1 {
            debug_message(&env, nvidiagpu::reset_fan_speed(&env, gpu), "Resetting Fan Speed");
        } else {
            debug_message(&env, nvidiagpu::set_fan_speed(&env, gpu, fan_index, fan_speed as usize), "Fan Speed");
        }
    } else if cmd.name.eq("memoryoffset") {
        let memory_offset;
        match args[0].parse::<i32>() {
            Ok(n) => {
                memory_offset = n;
            },
            Err(_) => {
                println!("Failed to set memory offset. {} is not a valid integer.", args[0]);
                return;
            }
        }

        debug_message(&env, nvidiagpu::set_memory_offset(&env, gpu, memory_offset), "Memory Speed Offset");
    } else if cmd.name.eq("clockoffset") {
        let clock_offset;
        match args[0].parse::<i32>() {
            Ok(n) => {
                clock_offset = n;
            },
            Err(_) => {
                println!("Failed to set core clock offset. {} is not a valid integer.", args[0]);
                return;
            }
        }

        debug_message(&env, nvidiagpu::set_core_offset(&env, gpu, clock_offset), "Clock Offset");
    } else if cmd.name.eq("clock") {
        let clock_speed;
        match args[0].parse::<i32>() {
            Ok(n) => {
                clock_speed = n;
            },
            Err(_) => {
                println!("Failed to lock core clock. {} is not an integer greater than or equal to zero. If you wish to remove the locked speed, please specify -1 as your argument.", args[0]);
                return;
            }
        }

        if clock_speed > 0 {
            debug_message(&env, nvidiagpu::lock_core(&env, gpu, clock_speed as usize), "Locked Core Clock");
        } else {
            debug_message(&env, nvidiagpu::reset_core(&env, gpu), "Resetting Core Clock");
        }
    } else if cmd.name.eq("memory") {
        let memory_speed;
        match args[0].parse::<i32>() {
            Ok(n) => {
                memory_speed = n;
            },
            Err(_) => {
                println!("Failed to lock memory clock. {} is not an integer. If you wish to remove the locked speed, please specify -1 as your argument.", args[0]);
                return;
            }
        }

        if memory_speed >= 0 {
            debug_message(&env, nvidiagpu::lock_memory(&env, gpu, memory_speed as usize), "Locked Memory Speed");
        } else {
            debug_message(&env, nvidiagpu::reset_memory(&env, gpu), "Resetting Memory Clock")
        }
    } else if cmd.name.eq("power") {
        let power;
        match args[0].parse::<usize>() {
            Ok(n) => {
                power = n;
            },
            Err(_) => {
                println!("Failed to set power limit. {} is not a valid integer greater than or equal to 0.", args[0]);
                return;
            }
        }

        debug_message(&env, nvidiagpu::set_power_limit(&env, gpu, power), "Power Limit");
    } else if cmd.name.eq("reset") {
        debug_message(&env, nvidiagpu::reset_core(&env, gpu), "Resetting Core Clock");
        debug_message(&env, nvidiagpu::reset_memory(&env, gpu), "Resetting Memory Clock");
        debug_message(&env, nvidiagpu::set_core_offset(&env, gpu, 0), "Clock Offset");
        debug_message(&env, nvidiagpu::set_memory_offset(&env, gpu, 0), "Memory Offset");
        debug_message(&env, nvidiagpu::reset_fan_speed(&env, gpu), "Fan Speed");
    }
}

fn main() {
    let mut env = Environment::default();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        nvidiagpu::print_query_info(&env, &0);
    } else if args.len() == 2 {
        match args[1].parse::<usize>() {
            Ok(n) => {
                nvidiagpu::print_query_info(&env, &n);
                return;
            },
            Err(_) => {}
        }
    }

    let mut commands = HashMap::new();
    commands.insert(String::from("help"), new_command(String::from("help"), vec![String::from("--help"), String::from("-h")], vec![0]));
    commands.insert(String::from("version"), new_command(String::from("version"), vec![String::from("--version"), String::from("-v"), String::from("v")], vec![0]));
    commands.insert(String::from("xauth"), new_command(String::from("xauth"), vec![String::from("xauthority"), String::from("xa"), String::from("--xauth")], vec![1]));
    commands.insert(String::from("debug"), new_command(String::from("debug"), vec![String::from("debug"), String::from("--debug")], vec![1]));
    commands.insert(String::from("display"), new_command(String::from("display"), vec![String::from("dp"), String::from("--display")], vec![1]));
    commands.insert(String::from("gpu"), new_command(String::from("gpu"), vec![], vec![1]));
    commands.insert(String::from("fan"), new_command(String::from("fan"), vec![], vec![1, 2]));
    commands.insert(String::from("memoryoffset"), new_command(String::from("memoryoffset"), vec![String::from("moc"), String::from("--memoc"), String::from("--memory-offset")], vec![1]));
    commands.insert(String::from("clockoffset"), new_command(String::from("clockoffset"), vec![String::from("--clockoc"), String::from("--clock-offset")], vec![1]));
    commands.insert(String::from("clock"), new_command(String::from("clock"), vec![String::from("lgc"), String::from("--clock")], vec![1]));
    commands.insert(String::from("memory"), new_command(String::from("memory"), vec![String::from("lmc"), String::from("--memory")], vec![1]));
    commands.insert(String::from("power"), new_command(String::from("power"), vec![String::from("pl"), String::from("--power")], vec![1]));
    commands.insert(String::from("reset"), new_command(String::from("reset"), vec![String::from("r"), String::from("--reset")], vec![0]));

    get_xauthority(&mut env);
    get_display(&mut env);

    let mut gpu_index = 0;

    let mut index: usize = 1;
    let mut finding_argument = true;
    let mut cmd = commands.get("help").unwrap();
    let mut arguments: Vec<&String> = Vec::new();

    let mut args_max: &usize = &usize::default();
    let mut args_count: usize = 0;

    while index < args.len() {
        if finding_argument {
            match cmd_exists(&args[index], &commands) {
                Some(x) => {
                    cmd = x;
                    finding_argument = false;
                    args_max = cmd.args.iter().max().unwrap();
                },
                None => {
                    println!("'{}' was not recognized as a valid argument. Try using 'help' for more information.", &args[index]);
                }
            }
        } else {
            if args_count >= *args_max {
                run(cmd, &arguments, &mut env, &mut gpu_index);

                finding_argument = true;
                index -= 1;
                args_count = 0;
                arguments.clear();
            } else {
                match cmd_exists(&args[index], &commands) {
                    Some(_) => {
                        run(cmd, &arguments, &mut env, &mut gpu_index);

                        finding_argument = true;
                        index -= 1;
                        args_count = 0;
                        arguments.clear();
                    },
                    None => {
                        arguments.push(&args[index]);
                    }
                }
            }
        }

        index += 1;
    }

    if !finding_argument {
        run(cmd, &arguments, &mut env, &mut gpu_index);
    }
}
