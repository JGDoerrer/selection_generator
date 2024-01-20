# Selecting the `i'th largest of n` elements

## Table of Contents

## Introduction

The following introduction was copied from [1]. Our motivation was to confirm the values in the
table, find optimal algorithms for values where only a lower bound exists and add some values
for `n=16`.

This page gives algorithms for finding the `i'th largest of 'n` elements with the fewest possible
comparisons.

The problem has been widely studied in literature. **Knuth**[2] gives a brief history of the problem and
some known upper and lower bounds. Optimal algorithms are known for any `n' when i` is one or two,
but for example for finding the median (i = n/2), there is a significant performance gap from the
best known algorithm `(approximately 2.97 n + o(n) comparisons)` to the tightest known minimum
`(2 n - o(n))`.

Many other special cases are also known, such as the ones given in the table below. Many of these
were previously found by **Gasarch, Kelly and Pugh**[3] who introduced computer searching to find the
optimal selection algorithms. The additional results presented in the table above have been
found by **my independent implementation**[4], but it probably shares many characteristics with Gasarch's.

Numbers marked with one asterisk are new lower bounds. Numbers marked with two asterisks improve
previously known algorithms. Since I found no previous studies of `V(14,3)` and `V(15,3)` in the
literature, I have marked it with a question mark.

Click the number to get the optimal selection algorithm. If an optimal algorithm is not known,
I've given the tightest known lower bound and the best known algorithm.

| V(n,i) | i = 1        | i = 2        | i = 3          | i = 4            | i = 5            | i = 6            | i = 7                    | i = 8  |
| ------ | ------------ | ------------ | -------------- | ---------------- | ---------------- | ---------------- | ------------------------ | ------ |
| n = 2  | [1](V_2_1)   |              |                |                  |                  |                  |                          |        |
| n = 3  | [2](V_3_1)   | [3](V_3_2)   |                |                  |                  |                  |                          |        |
| n = 4  | [3](V_4_1)   | [4](V_4_2)   |                |                  |                  |                  |                          |        |
| n = 5  | [4](V_5_1)   | [6](V_5_2)   | [6](V_5_3)     |                  |                  |                  |                          |        |
| n = 6  | [5](V_6_1)   | [7](V_6_2)   | [8](V_6_3)     |                  |                  |                  |                          |        |
| n = 7  | [6](V_7_1)   | [8](V_7_2)   | [10](V_7_3)    | [10](V_7_4)      |                  |                  |                          |        |
| n = 8  | [7](V_8_1)   | [9](V_8_2)   | [11](V_8_3)    | [12](V_8_4)      |                  |                  |                          |        |
| n = 9  | [8](V_9_1)   | [11](V_9_2)  | [12](V_9_3)    | [14](V_9_4)      | [14](V_9_5)      |                  |                          |        |
| n = 10 | [9](V_10_1)  | [12](V_10_2) | [14](V_10_3)   | [15](V_10_4)     | [16](V_10_5)     |                  |                          |        |
| n = 11 | [10](V_11_1) | [13](V_11_2) | [15](V_11_3)   | [17](V_11_4)     | [18](V_11_5)\*   | [18](V_11_6)\*\* |                          |        |
| n = 12 | [11](V_12_1) | [14](V_12_2) | [17](V_12_3)   | [18](V_12_4)\*\* | [19](V_12_5)\*\* | [20](V_12_6)\*\* |                          |        |
| n = 13 | [12](V_13_1) | [15](V_13_2) | [18](V_13_3)\* | [20](V_13_4)\*   | [21](V_13_5)\*\* | [22](V_13_6)\*\* | [23](V_13_7)\*           |        |
| n = 14 | [13](V_14_1) | [16](V_14_2) | [19](V_14_3)?  | [21](V_14_4)\*   | [23](V_14_5)\*   | [24](V_14_6)\*\* | 24\*(..[25](V_14_7)\*\*) |        |
| n = 15 | [14](V_15_1) | [17](V_15_2) | [20](V_15_3)?  | [23](V_15_4)\*   | [25](V_15_5)\*   | 25\*(..26)       | 23..28                   | 24..28 |

## Improved known optimal compares

| V(n,i) | i = 1        | i = 2        | i = 3          | i = 4            | i = 5            | i = 6            | i = 7                    | i = 8  |
| ------ | ------------ | ------------ | -------------- | ---------------- | ---------------- | ---------------- | ------------------------ | ------ |
| n = 2  | [1](V_2_1)   |              |                |                  |                  |                  |                          |        |
| n = 3  | [2](V_3_1)   | [3](V_3_2)   |                |                  |                  |                  |                          |        |
| n = 4  | [3](V_4_1)   | [4](V_4_2)   |                |                  |                  |                  |                          |        |
| n = 5  | [4](V_5_1)   | [6](V_5_2)   | [6](V_5_3)     |                  |                  |                  |                          |        |
| n = 6  | [5](V_6_1)   | [7](V_6_2)   | [8](V_6_3)     |                  |                  |                  |                          |        |
| n = 7  | [6](V_7_1)   | [8](V_7_2)   | [10](V_7_3)    | [10](V_7_4)      |                  |                  |                          |        |
| n = 8  | [7](V_8_1)   | [9](V_8_2)   | [11](V_8_3)    | [12](V_8_4)      |                  |                  |                          |        |
| n = 9  | [8](V_9_1)   | [11](V_9_2)  | [12](V_9_3)    | [14](V_9_4)      | [14](V_9_5)      |                  |                          |        |
| n = 10 | [9](V_10_1)  | [12](V_10_2) | [14](V_10_3)   | [15](V_10_4)     | [16](V_10_5)     |                  |                          |        |
| n = 11 | [10](V_11_1) | [13](V_11_2) | [15](V_11_3)   | [17](V_11_4)     | [18](V_11_5)\*   | [18](V_11_6)\*\* |                          |        |
| n = 12 | [11](V_12_1) | [14](V_12_2) | [17](V_12_3)   | [18](V_12_4)\*\* | [19](V_12_5)\*\* | [20](V_12_6)\*\* |                          |        |
| n = 13 | [12](V_13_1) | [15](V_13_2) | [18](V_13_3)\* | [20](V_13_4)\*   | [21](V_13_5)\*\* | [22](V_13_6)\*\* | [23](V_13_7)\*           |        |
| n = 14 | [13](V_14_1) | [16](V_14_2) | [19](V_14_3)?  | [21](V_14_4)\*   | [23](V_14_5)\*   | [24](V_14_6)\*\* | 24\*(..[25](V_14_7)\*\*) |        |
| n = 15 | [14](V_15_1) | [17](V_15_2) | [20](V_15_3)?  | [23](V_15_4)\*   | [25](V_15_5)\*   | 25\*(..26)       | 23..28                   | 24..28 |
| n = 16 | [14](V_16_1) | [17](V_16_2) | [20](V_16_3)   | [23](V_16_4)     | [25](V_16_5)     | 25\*(..26)       | 23..28                   | 24..28 |

## Requirements

A Linux based operating system is needed to compile and run the program. In addition you need to install `Rust`[5], `nauty`[6] and `clang`[7]. The more hardware resources e.g. CPU and RAM you have the better.

Since the operating system of choice is `Archlinux` you can install the packages with:

```shell
# pacman -S clang nauty
```

to install `Rust` please follow the guide on the **website**[5].

The package version used to create the data is

```shell
$ uname -a
Linux froschteich 6.6.1-arch1-1 #1 SMP PREEMPT_DYNAMIC Wed, 08 Nov 2023 16:05:38 +0000 x86_64 GNU/Linux
$ rustc -V
rustc 1.77.0-nightly (6ae4cfbbb 2024-01-17)
$ pacman -Q nauty
nauty 1:2.8.8-2
$ pacman -Q clang
clang 16.0.6-1
```

For other operating systems please follow the appropriate instructions to get the required packages.

## Quick start

Having all required packages installed, starting a calculation on your own is straight forward.

```shell
cargo run --release -- -n 10 -i 3
```

This will start calculating compares starting at the **i=4th** (index starts at 0) value for a list of **n=10** and will continue calculating until you stop.

For more details please read

```shell
./target/release/selection_generator --help
Usage: selection_generator [OPTIONS]

Options:
  -n <N>                                 The n to start at
  -i <I>                                 The i to start at. Requires n to be set
  -s, --single                           Do only a single calculation
      --cache-file <CACHE_FILE>          The name of the cache file to use [default: cache.dat]
      --no-cache-file                    Do not use a cache file
  -e, --explore                          Explore the cache interactively
      --max-cache-size <MAX_CACHE_SIZE>  The max amount of bytes of the cache [default: 8589934592]
  -h, --help                             Print help
  -V, --version                          Print version
```

## Calculation times

One can find the calculation times in the `times*.txt` files. The times were generated using the hardware listed in the []

## Hardware used

The hardware used to calculate these results was

```shell
H/W path         Device    Class          Description
=====================================================
                           system         All Series (All)
/0                         bus            STRIX X99 GAMING
/0/0                       memory         64KiB BIOS
/0/62                      memory         64GiB System Memory
/0/62/0                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/62/1                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/62/2                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/62/3                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/62/4                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/62/5                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/62/6                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/62/7                    memory         8GiB DIMM DDR4 Synchronous 2133 MHz (0.5 ns)
/0/74                      memory         384KiB L1 cache
/0/75                      memory         1536KiB L2 cache
/0/76                      memory         15MiB L3 cache
/0/77                      processor      Intel(R) Core(TM) i7-6850K CPU @ 3.60GHz
/0/100                     bridge         Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DMI2
/0/100/1                   bridge         Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D PCI Express Root Port 1
/0/100/1.1                 bridge         Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D PCI Express Root Port 1
/0/100/3                   bridge         Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D PCI Express Root Port 3
/0/100/3/0                 display        GP104 [GeForce GTX 1080]
/0/100/3/0.1     card1     multimedia     GP104 High Definition Audio Controller
/0/100/3/0.1/0   input6    input          HDA NVidia HDMI/DP,pcm=3
/0/100/3/0.1/1   input7    input          HDA NVidia HDMI/DP,pcm=7
/0/100/3/0.1/2   input8    input          HDA NVidia HDMI/DP,pcm=8
/0/100/3/0.1/3   input9    input          HDA NVidia HDMI/DP,pcm=9
/0/100/5                   generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Map/VTd_Misc/System Management
/0/100/5.1                 generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D IIO Hot Plug
/0/100/5.2                 generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D IIO RAS/Control Status/Global Errors
/0/100/5.4                 generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D I/O APIC
/0/100/11                  generic        C610/X99 series chipset SPSR
/0/100/11.4                storage        C610/X99 series chipset sSATA Controller [AHCI mode]
/0/100/14                  bus            C610/X99 series chipset USB xHCI Host Controller
/0/100/14/0      usb3      bus            xHCI Host Controller
/0/100/14/0/a              bus            ASM107x
/0/100/14/0/a/1            communication  Qualcomm Bluetooth 4.1
/0/100/14/1      usb4      bus            xHCI Host Controller
/0/100/14/1/6              bus            ASM107x
/0/100/16                  communication  C610/X99 series chipset MEI Controller #1
/0/100/19        eno1      network        Ethernet Connection (2) I218-V
/0/100/1a                  bus            C610/X99 series chipset USB Enhanced Host Controller #2
/0/100/1a/1      usb1      bus            EHCI Host Controller
/0/100/1a/1/1              bus            Hub
/0/100/1b        card0     multimedia     C610/X99 series chipset HD Audio Controller
/0/100/1b/0      input10   input          HDA Intel PCH Front Mic
/0/100/1b/1      input11   input          HDA Intel PCH Rear Mic
/0/100/1b/2      input12   input          HDA Intel PCH Line
/0/100/1b/3      input13   input          HDA Intel PCH Line Out Front
/0/100/1b/4      input14   input          HDA Intel PCH Line Out Surround
/0/100/1b/5      input15   input          HDA Intel PCH Line Out CLFE
/0/100/1b/6      input16   input          HDA Intel PCH Front Headphone
/0/100/1c                  bridge         C610/X99 series chipset PCI Express Root Port #1
/0/100/1c.3                bridge         C610/X99 series chipset PCI Express Root Port #4
/0/100/1c.3/0    wlan0     network        QCA6174 802.11ac Wireless Network Adapter
/0/100/1c.4                bridge         C610/X99 series chipset PCI Express Root Port #5
/0/100/1c.4/0              bus            ASM1142 USB 3.1 Host Controller
/0/100/1c.4/0/0  usb5      bus            xHCI Host Controller
/0/100/1c.4/0/1  usb6      bus            xHCI Host Controller
/0/100/1d                  bus            C610/X99 series chipset USB Enhanced Host Controller #1
/0/100/1d/1      usb2      bus            EHCI Host Controller
/0/100/1d/1/1              bus            8 channel internal hub
/0/100/1f                  bridge         C610/X99 series chipset LPC Controller
/0/100/1f/0                system         AT Real-Time Clock
/0/100/1f/1                system         Motherboard registers
/0/100/1f/2                system         Motherboard registers
/0/100/1f/3                communication  16550A-compatible COM port
/0/100/1f.2                storage        C610/X99 series chipset 6-Port SATA Controller [AHCI mode]
/0/100/1f.3                bus            C610/X99 series chipset SMBus Controller
/0/b                       generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D R3 QPI Link 0/1
/0/b.1                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D R3 QPI Link 0/1
/0/b.2                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D R3 QPI Link 0/1
/0/b.3                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D R3 QPI Link Debug
/0/c                       generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/c.1                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/c.2                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/c.3                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/c.4                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/c.5                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/f                       generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/f.1                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/f.4                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/f.5                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/f.6                     generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Caching Agent
/0/10                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D R2PCIe Agent
/0/10.1                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D R2PCIe Agent
/0/10.5                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Ubox
/0/10.6                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Ubox
/0/10.7                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Ubox
/0/12                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Home Agent 0
/0/12.1                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Home Agent 0
/0/13                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Target Address/Thermal/RAS
/0/13.1                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Target Address/Thermal/RAS
/0/13.2                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel Target Address Decoder
/0/13.3                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel Target Address Decoder
/0/13.4                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel Target Address Decoder
/0/13.5                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel Target Address Decoder
/0/13.6                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 0/1 Broadcast
/0/13.7                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Global Broadcast
/0/14                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 0 Thermal Control
/0/14.1                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 1 Thermal Control
/0/14.2                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 0 Error
/0/14.3                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 1 Error
/0/14.4                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 0/1 Interface
/0/14.5                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 0/1 Interface
/0/14.6                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 0/1 Interface
/0/14.7                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 0/1 Interface
/0/15                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 2 Thermal Control
/0/15.1                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 3 Thermal Control
/0/15.2                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 2 Error
/0/15.3                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 0 - Channel 3 Error
/0/16                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Target Address/Thermal/RAS
/0/16.6                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 2/3 Broadcast
/0/16.7                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Global Broadcast
/0/17                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Memory Controller 1 - Channel 0 Thermal Control
/0/17.4                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 2/3 Interface
/0/17.5                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 2/3 Interface
/0/17.6                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 2/3 Interface
/0/17.7                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D DDRIO Channel 2/3 Interface
/0/1e                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Power Control Unit
/0/1e.1                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Power Control Unit
/0/1e.2                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Power Control Unit
/0/1e.3                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Power Control Unit
/0/1e.4                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Power Control Unit
/0/1f                      generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Power Control Unit
/0/1f.2                    generic        Xeon E7 v4/Xeon E5 v4/Xeon E3 v4/Xeon D Power Control Unit
/1                         power          To Be Filled By O.E.M.
/2               /dev/fb0  display        simpledrmdrmfb
/3               input0    input          Power Button
/4               input1    input          Power Button
/5               input2    input          PC Speaker
/6               input3    input          Eee PC WMI hotkeys

```

## References

[1]: Kenneth Oksanen. _Research_. [https://www.cs.hut.fi/~cessu/selection/](https://www.cs.hut.fi/~cessu/selection/)

[2]: Knuth. _Research_. [https://www-cs-faculty.stanford.edu/~knuth/taocp.html#vol3](https://www-cs-faculty.stanford.edu/~knuth/taocp.html#vol3)

[3]: Gasarch, Kelly and Pugh. _Research_. [https://www.cs.umd.edu/~gasarch/papers/select.ps](https://www.cs.umd.edu/~gasarch/papers/select.ps)

[4]: Kenneth Oksanen. _Research_. [https://www.cs.hut.fi/~cessu/selection/selgen.c.html](https://www.cs.hut.fi/~cessu/selection/selgen.c.html)

[5]: Rust [https://www.rust-lang.org/](https://www.rust-lang.org/)

[6]: Nauty [https://pallini.di.uniroma1.it/](https://pallini.di.uniroma1.it/)

[7]: Clang [https://clang.llvm.org/](https://clang.llvm.org/)
