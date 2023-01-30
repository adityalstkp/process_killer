# process_killer

Process killer based on config JSON file.


## Usage
```bash
Usage: process_killer [OPTIONS] --config-path <CONFIG_PATH>                              
                                                                                         
Options:                                                                                 
  -c, --config-path <CONFIG_PATH>                                                        
  -r, --refresh-time <REFRESH_TIME>  [default: 5000]                                     
  -d, --dry-run                                                                          
  -h, --help                         Print help                                          
  -V, --version                      Print version
```

## Example Run
```json
[
    {
        "name": "htop",
        "expired": "10s"
    }
]
```
----
```bash
name: htop | pid: 11220 | delta: 2s
name: htop | pid: 11603 | delta: 1s
name: htop | pid: 11220 | delta: 7s
name: htop | pid: 11603 | delta: 6s
name: htop | pid: 11220 | delta: 12s
procs with pid: 11220 is gonna be killed!
name: htop | pid: 11603 | delta: 11s
procs with pid: 11603 is gonna be killed!
```


## TODO:

- [x] Parse expired with `h`, `m`, and `s` unit.
- [x] Parse args when running binary.
- [x] Kill procs when expired
- [ ] Detect zombie ??
