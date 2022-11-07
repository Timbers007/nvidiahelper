use std::io::{Result};
use std::process::Output;
use crate::{Environment, execute};

pub fn set_memory_offset(env: &Environment, gpu: &mut usize, memory_offset: i32) -> Result<Output> {
    execute(env, &format!("sudo DISPLAY={} XAUTHORITY={} nvidia-settings -a [gpu:{}]/GPUMemoryTransferRateOffsetAllPerformanceLevels={}", env.display, env.xauthority, gpu, memory_offset))
}

pub fn set_core_offset(env: &Environment, gpu: &mut usize, clock_offset: i32) -> Result<Output> {
    execute(env, &format!("sudo DISPLAY={} XAUTHORITY={} nvidia-settings -a [gpu:{}]/GPUGraphicsClockOffsetAllPerformanceLevels={}", env.display, env.xauthority, gpu, clock_offset))
}

pub fn lock_core(env: &Environment, gpu: &mut usize, clock_speed: usize) -> Result<Output> {
    execute(env, &format!("sudo nvidia-smi -i {} -lgc {}", gpu, clock_speed))
}

pub fn lock_memory(env: &Environment, gpu: &mut usize, memory_speed: usize) -> Result<Output> {
    execute(env, &format!("sudo nvidia-smi -i {} -lmc {}", gpu, memory_speed))
}

pub fn set_power_limit(env: &Environment, gpu: &mut usize, power: usize) -> Result<Output> {
    execute(env, &format!("sudo nvidia-smi -i {} -pl {}", gpu, power))
}

pub fn set_fan_speed(env: &Environment, gpu: &mut usize, fan_index: usize, fan_speed: usize) -> Result<Output> {
    execute(env, &format!("sudo DISPLAY={} XAUTHORITY={} nvidia-settings -a [gpu:{}]/GPUFanControlState=1 -a [fan:{}]/GPUTargetFanSpeed={}", env.display, env.xauthority, gpu, fan_index, fan_speed))
}

pub fn reset_fan_speed(env: &Environment, gpu: &mut usize) -> Result<Output> {
    execute(env, &format!("sudo DISPLAY={} XAUTHORITY={} nvidia-settings -a [gpu:{}]/GPUFanControlState=0", env.display, env.xauthority, gpu))
}

pub fn reset_core(env: &Environment, gpu: &mut usize) -> Result<Output> {
    execute(env, &format!("sudo nvidia-smi -i {} -rgc", gpu))
}

pub fn reset_memory(env: &Environment, gpu: &mut usize) -> Result<Output> {
    execute(env, &format!("sudo nvidia-smi -i {} -rmc", gpu))
}

pub fn query_gpu_field<'a>(env: &Environment, gpu: &'a usize, field: &'a str) -> String {
    let y = execute(env, &format!("nvidia-smi -i {} --query-gpu={} --format=csv,noheader", gpu, field));
    match y {
        Ok(x) => {
            match String::from_utf8(x.stdout) {
                Ok(output) => {
                    output
                },
                Err(_) => {
                    String::from("Unknown")
                }
            }
        },
        Err(_) => {
            String::from("Unknown")
        }
    }
}

pub fn print_query_info(env: &Environment, gpu: &usize) {
    let gpu_information_raw = query_gpu_field(env, gpu, "name,clocks.current.graphics,clocks.current.memory,temperature.gpu,power.draw,\
    fan.speed,memory.used,memory.total,enforced.power.limit,driver_version,pcie.link.gen.current,pcie.link.width.current,vbios_version");
    let gpu_information:Vec<&str> = gpu_information_raw.split(", ").collect();

    println!(" _                _________ ______  _________ _______ ");
    println!("( (    /||\\     /|\\__   __/(  __  \\ \\__   __/(  ___  )");
    println!("|  \\  ( || )   ( |   ) (   | (  \\  )   ) (   | (   ) |");
    println!("|   \\ | || |   | |   | |   | |   ) |   | |   | (___) |");
    println!("| (\\ \\) |( (   ) )   | |   | |   | |   | |   |  ___  |");
    println!("| | \\   | \\ \\_/ /    | |   | |   ) |   | |   | (   ) |");
    println!("| )  \\  |  \\   /  ___) (___| (__/  )___) (___| )   ( |");

    let footer = "|/    )_)   \\_/   \\_______/(______/ \\_______/|/     \\|";
    println!("{}", footer);
    println!();
    if gpu_information.len() != 13 {
        println!("{}", gpu_information[0]);
        return;
    }

    println!("Name: {}", gpu_information[0]);
    println!("Core Clock Speed: {}", gpu_information[1]);
    println!("Memory Clock Speed: {}", gpu_information[2]);
    println!("Temperature: {}", gpu_information[3]);
    println!("Power: {}", gpu_information[4]);
    println!("Fan Speed: {}", gpu_information[5]);
    println!();
    println!("Used Memory: {}", gpu_information[6]);
    println!("Total Memory: {}", gpu_information[7]);
    println!("Max Power: {}", gpu_information[8]);
    println!();
    println!("Driver: {}", gpu_information[9]);
    println!("GPU PCIe Generation: {}", gpu_information[10]);
    println!("GPU PCIe Link Width: {}", gpu_information[11]);
    println!("VBios: {}", gpu_information[12]);
}
