# nvidiahelper

A simple & lightweight Rust program that allows you to tweak NVIDIA GPUs from the linux command line without having to remember lengthy commands.

## Usage

For viewing your GPUs current settings, simply run `./nvidiahelper`. If you wish to view another GPU in your system other than the one in the first slot, you can specify it like this: `./nvidiahelper 1`.
Other than just viewing current metrics, there are a lot of settings available for tweaking. You can access the help menu with `./nvidiahelper help` where a full list of 
commands are listed. You can also find that full help menu below:

```
----- NVIDIA GPU Terminal Helper ----- v1.0 -----

Execute Command Format: ./nvidiahelper argument1 arg1value1 argument2 arg2value1 arg2value2
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



Example: ./nvidiahelper fan 0 75 fan 1 75 clockoffset 150 memoryoffset 500 power 400
```

## Examples
### Example One
Set the first and second fans of the first GPU to 75% speed:
```
nvidiahelper fan 0 75 fan 1 75
```

### Example Two
GPU 0: Set a locked core clock of 1500 MHz, a memory offset of +1500 (+750 MHz), a power limit of 250 W, and the first two fans to 75% speed.

GPU 1: Set a locked core clock of 900 MHz, a memory offset of -1000 (-500 MHz), a power limit of 100 W, and the first fan to 75% and second fan to 77% speed.
```
nvidiahelper gpu 0 clock 1500 memoryoffset 1000 power 250 fan 0 70 fan 1 70 gpu 1 clock 900 memoryoffset -1000 power 100 fan 2 75 fan 3 77
```
**NOTE**: Fan IDs are cumulative for each GPU in your system. For example, if you have five GPUs with two fans each, the second fan of the third GPU
would be fan 5.

## Build

Simply clone the repository and build with cargo:

```
cargo build --release
```

## Advanced Options
On newer drivers, setting `xauth` or `display` should not be necessary. However, older drivers may require these fields be provided. Nvidiahelper will attempt
to search for the in use .Xauthroity file and current display ID. Should this fail, nvidiahelper may not be able to interact propery with nvidia-settings. To fix this, you
can simply pass through the location of the xauth file and display id. 

Example: `./nvidiahelper display :0 xauth /dir/example/xorg/.Xauthority gpu 1 fan 0 70`.

## License

This software is licensed under the Apache License 2.0. The NVIDIA name is trademarked and has no affiliation with this repository. 

