# Completely very fast RUST syysinfo project (faster than GO similar project 12 times)

Version for Mac OS X m1 only

## Run from source

```shell
cd /path/to/project
cargo run
```

Go to browser and you'll see completely json with system info

```shell
http://127.0.0.1:8989/system_info
```

## Run compiled release

```shell
cargo build
```

or

```shell
cargo build --release
```

Then, start the complied app

```shell
cargo run
```

or

```shell
cargo run --release
```

<details>
<summary>Example output</summary>

```json

{
  "memory": {
    "total": "32768.00 МБ",
    "used": "32623.77 МБ",
    "available": "144.23 МБ"
  },
  "disks": [
    {
      "name": "Macintosh HD",
      "total_space": "948584.16 МБ",
      "used_space": "743076.75 МБ"
    },
    {
      "name": "Macintosh HD",
      "total_space": "948584.16 МБ",
      "used_space": "743076.75 МБ"
    }
  ],
  "cpu": [
    {
      "id": 0,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 1,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 2,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 3,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 4,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 5,
      "name": "Apple M1 Max",
      "frequency": 0
    }
  ],
  "video": [
    {
      "details": {
        "Apple M1 Max": {
          "Metal Support": "Metal 3",
          "Online": "Yes",
          "Resolution": "3456 x 2234 Retina",
          "Total Number of Cores": "32",
          "Automatically Adjust Brightness": "Yes",
          "Type": "GPU",
          "Display Type": "Built-in Liquid Retina XDR Display",
          "Rotation": "Supported",
          "Chipset Model": "Apple M1 Max",
          "Displays": "",
          "UI Looks like": "1920 x 1080 @ 75.00Hz",
          "Connection Type": "Internal",
          "Bus": "Built-In",
          "Main Display": "Yes",
          "MP59G": "",
          "Mirror": "Off",
          "Color LCD": "",
          "Vendor": "Apple (0x106b)"
        }
      }
    }
  ],
  "displays": [
    {
      "details": {
        "Built-in Liquid Retina XDR Display": {
          "Connection Type": "Internal",
          "Automatically Adjust Brightness": "Yes",
          "Mirror": "Off",
          "Display Type": "Built-in Liquid Retina XDR Display",
          "Online": "Yes",
          "Resolution": "3456 x 2234 Retina"
        }
      }
    }
  ],
  "processes": [
    {
      "pid": 909,
      "name": "pkd",
      "status": "Run",
      "memory": "16.34 МБ",
      "cpu_usage": "0.00%",
      "uptime": "0 г. 0 мес. 4 д. 11 ч. 57 м. 16 с."
    },
    {
      "pid": 1043,
      "name": "spindump",
      "status": "Unknown(0)",
      "memory": "0.00 МБ",
      "cpu_usage": "0.00%",
      "uptime": "0 г. 0 мес. 0 д. 00 ч. 00 м. 00 с."
    },
    {
      "pid": 189,
      "name": "KernelEventAgent",
      "status": "Unknown(0)",
      "memory": "0.00 МБ",
      "cpu_usage": "0.00%",
      "uptime": "0 г. 0 мес. 0 д. 00 ч. 00 м. 00 с."
    },
    {
      "pid": 1118,
      "name": "TextInputSwitcher",
      "status": "Run",
      "memory": "24.30 МБ",
      "cpu_usage": "0.00%",
      "uptime": "0 г. 0 мес. 4 д. 11 ч. 57 м. 08 с."
    },
    {
      "pid": 95080,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run",
      "memory": "95.64 МБ",
      "cpu_usage": "0.00%",
      "uptime": "0 г. 0 мес. 0 д. 00 ч. 47 м. 22 с."
    }
  ],
  "kernel_version": "macos",
  "execution_time": "0.79 секунд"
}

```

</details>