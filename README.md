# Team Green Helper

A simple & lightweight Rust program that allows you to tweak NVIDIA GPUs from the linux command line without having to remember lengthy commands. It can also display some essential stats such as frequencies, temperatures, and more.

## Usage

For viewing your GPUs current settings, simply run `./teamgreenhelper`. If you wish to view another GPU in your system other than the one in the first slot, you can specify it like this: `./teamgreenhelper 1`.
Other than just viewing current metrics, there are a lot of settings available for tweaking. You can access the help menu with `./teamgreenhelper help` where a full list of 
commands are listed. You can also find that full help menu below:

```
----- NVIDIA GPU Terminal Helper ----- b2 -----

Execute Command Format: ./teamgreenhelper argument1 arg1value1 argument2 arg2value1 arg2value2
Further, [argument] will represent an argument that is required. () is optional. Omit [] and/or () when you execute the command.

GPU Control Arguments:
  gpu [gpu_id]
        Optional. Sets the GPU to adjust settings for. Defaults to GPU 0.

  fan [fan_id] [fan_speed]
        Sets the GPU fan at position fan_id to speed fan_speed.

  clock [speed]
        Sets the GPU core clock speed to speed.

  memory [speed]
        Sets the GPU memory clock speed to speed. 

  memoryoffset [speed] (power level)
        Sets the GPU memory clock speed offset to speed. Overclocks or underclocks memory.

  clockoffset [speed] (power level)
        Sets the GPU core clock clock offset speed to speed. Overclocks or underclocks core.

  power [watts]
        Limits the GPU to only be able to pull at most the specified watts.

  resetall
        Resets all settings to their defaults. 

Advanced Options (Optional):

  display [display_id]
        Sets the Xorg display value to be passed into nvidia-settings. This is automatic if none is specified.

  xauth [Xauthority path]
        Sets the Xauthority file path to be passed into nvidia-settings. This is automatic if none is specified.

  debug true
        Shows output of all executions from this program. Will be detailed.



Example: ./teamgreenhelper fan 0 75 fan 1 75 clockoffset 150 memoryoffset 500 power 400
```

### Summary Example Output
```
./teamgreenhelper
 _______                    _____                     
|__   __|                  / ____|                    
   | | ___  __ _ _ __ ___ | |  __ _ __ ___  ___ _ __  
   | |/ _ \/ _` | '_ ` _ \| | |_ | '__/ _ \/ _ \ '_ \ 
   | |  __/ (_| | | | | | | |__| | | |  __/  __/ | | |
   |_|\___|\__,_|_| |_| |_|\_____|_|  \___|\___|_| |_|

Name: NVIDIA GeForce RTX 3080
Core Clock Speed: 1635 MHz
Memory Clock Speed: 10902 MHz
Temperature: 49.5 C
Power: 179.16 W
Fan Speed: 57 %

Used Memory: 3255 MiB
Total Memory: 10240 MiB
Max Power: 180.00 W

Driver: 525.60.11
GPU PCIe Generation: 3
GPU PCIe Link Width: 16
VBios: 94.02.71.40.A9
```

## Examples
### Example One
Set the first and second fans of the first GPU to 75% speed:
```
./teamgreenhelper fan 0 75 fan 1 75
```

### Example Two
GPU 0: Set a locked core clock of 1500 MHz, a memory offset of +1500 (+750 MHz), a power limit of 250 W, and the first two fans to 75% speed.

GPU 1: Set a locked core clock of 900 MHz, a memory offset of -1000 (-500 MHz), a power limit of 100 W, and the first fan to 75% and second fan to 77% speed.
```
./teamgreenhelper gpu 0 clock 1500 memoryoffset 1000 power 250 fan 0 70 fan 1 70 gpu 1 clock 900 memoryoffset -1000 power 100 fan 2 75 fan 3 77
```
**NOTE**: Fan IDs are cumulative for each GPU in your system. For example, if you have five GPUs with two fans each, the second fan of the third GPU
would be fan 5.

## Build

Simply clone the repository and build with cargo:

```
cargo build --release
```

## FAQ

### Why can't I set fan speeds, core offsets, or memory offsets?
Ensure that you have an X server running. You can check this by running `nvidia-smi` and looking for an Xorg process. If you are, try generating an X.org config file by running `nvidia-xconfig --enable-all-gpus --cool-bits=28`. Restart your X server, and try again. To help debug your issue, you should also run your commands with `debug` enabled to see any success or error message that might be provided internally.
```
# Enable debugging output & increase memory frequency 
./teamgreenhelper debug true memoryoffset 1000 
Successfully set debug mode to true

ERROR: Error assigning value 1 to attribute 'GPUMemoryTransferRateOffsetAllPerformanceLevels' (archlinux:0[gpu:0]) as specified in assignment '[gpu:0]/GPUMemoryTransferRateOffsetAllPerformanceLevels=1' (Unknown Error).

```

### Why is root required to set a locked core or memory clock?
Team Green Helper uses NVIDIA's built-in `nvidia-smi` utility which requires root privilege when locking these frequencies. You can still set clock offsets and fan speeds as a normal user that is authenticated with the X server.

## Advanced Options
On newer drivers, setting `xauth` or `display` should not be necessary. However, older drivers may require these fields be provided. Nvidiahelper will attempt
to search for the in use .Xauthroity file and current display ID. Should this fail, nvidiahelper may not be able to interact propery with nvidia-settings. To fix this, you
can simply pass through the location of the xauth file and display id. 

Example: `./teamgreenhelper display :0 xauth /dir/example/xorg/.Xauthority gpu 1 fan 0 70`.

## License

This software is licensed under the Apache License 2.0. The NVIDIA name is trademarked and has no affiliation with this repository. 

