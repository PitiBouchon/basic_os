# HartId and Device Tree

The two arguments hart id and the adress of the devicetree are RiscV platform specific

## Hart Id

"Hart" is the abbreviation of "Hardware Thread"

Basically one hart equal one cpu [^note]

[^note] If you have ever heard of "Hyper Threading it means that one CPU can have multiple hardware thread (often two)"

## DeviceTree

The device tree is a data structure placed in memory by the BIOS in order for the OS to know the architecture of the machine

You can see the "readable" version with the file `qemu.dts`

Often you'll see FDT and DTB which refer to Flatten Device Tree and the Device Tree Blob which corresponds to the data 
structure and the binary of the device tree