# Simple tail viewer for log files

## Purpose
I work on different systems where `tail` like command can be not available. Another reason 
is
my log file entries contain a timestamp in milliseconds since the epoch. The program converts
the information in a human readable format.

There is also `simhead` counterpart of the `simtail`.

## Recommended project settings
If you use [RDS](https://github.com/vernisaz/rust_dev_studio) for operating to the project,
the following settings are recommended:
```properties
# property file on 03-18-2026 Wednesday, 20:08:45 -0700
src_dir=
project_home=projects/simlogtail
colapsed_dirs=
theme=One
proj_conf={"compile_debug":"rb -f bee","compile_release":"rb -Dmode=release -r -f bee","clippy":"rb clippy -f bee","debug_app":"rb -f bee-head","run_app":"rb clippy","test_app":"rb test","package_app":"rb package","format_src":"rustfmt --edition 2024"}
projectnp=no
user=Your name<your.name@your-organization.com>
persist_tabs=no
ai_server_url=
format_on_save=yes
autosave=yes
```

## How to build

1. Obtain [RustBee](https://github.com/vernisaz/rust_bee) 
2. Check out [Simple Time](https://github.com/vernisaz/simtime) and build (unless you  did that already)
3. Run *rb*
4. Check out [SimpleColor](https://github.com/vernisaz/simcolor) and build (unless you already did that)
5. Run *rb*

## What to improve
The current solution to deal with big files can impact the performance. So a prediction
to seek a file position to start reading, can be desirable.

Another improvement can be to keep running the program and read new lines as they get available.