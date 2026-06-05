# Simple tail viewer for log files

## Purpose
I work on different platforms where `tail` like command can be not available. Another reason 
is that
my log file entries contain a timestamp in milliseconds since the epoch. The program converts
the information in a human readable form.

There is also `simhead` a counterpart of the `simtail`.

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

## Usage

`Usage: simtail [opts] <file path>[...<file path>]
Where opts are:
-c	Do not show and count empty lines in the out
-f	Real time tail monitoring (only the last file when more than one specified)
-h	This help screen
-n	Number of shown lines
-v	Version of the product`

`Usage: simhead [opts] <file path>[ ...<file path>]
Where opts are:
-c	Do not show and count empty lines in the out
-h	This help screen
-n	Number of shown lines
-v	Version of the product`

## How to build

1. Obtain [RustBee](https://github.com/vernisaz/rust_bee)
2. Checkout [common script](https://github.com/vernisaz/simscript), unless it's done in 1st step
3. Check out [Simple Time](https://github.com/vernisaz/simtime) and build (unless you  did that already)
4. Run *rb*
5. Check out [SimpleColor](https://github.com/vernisaz/simcolor) and build (unless you already did that)
6. Run *rb*
7. Check out [SimpleCLI](https://github.com/vernisaz/simcli) and build it, unless you already did it
8. Run *rb*
9. Finally run *rb -f bee* here to build _simtail_ and *rb -f bee-head* for _simhead_

### Use tips
When a real time tail monitoring is used, type 'q' to exit it.

## What to improve
The current solution to deal with big files can impact the performance. So a prediction
to seek a file position to start reading, can be desirable.
