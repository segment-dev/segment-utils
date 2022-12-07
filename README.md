# Segment Utils

This repository contains a bunch of utilities that can be used with the [Segment server](https://github.com/segment-dev/segment). Currently there are following utilities:

- `segment-cli` - A CLI to interact with the segment server, can be used for server maintenance or just to play with Segment.
- `segment-benchmark` (under development) - A benchmarking tool that can be used to benchmark the Segment server to see if it fits your requirements.

### Installation

To install these utilities you can follow the following steps:

1. Clone this repository
2. `cd /path/to/cloned/repo`
3. `cargo build --release`
4. The final binary can be found in `./target/release`

### Segment CLI

Segment CLI is an interactive CLI that can be used to connect to the segment server directly.

#### Basic Usage

##### Connect to the server

```shell
segment-cli --host=<SEGMENT SERVER HOST> --port=<SEGMENT SERVER PORT>
```

If you don't pass any values for the `--host` and `--port` options, default values be used.

You can use the `--help` option to get more information about the usage.

##### Issuing commands

Once the connection has been successfully established with the server you will see the following prompt.

```shell
127.0.0.1:1698>
```

In this case we have connected to the default host and port which are `127.0.0.1` and `1698` respectively. If you pass some other value for host and port that will be visible here.

After connecting you can start issuing commands just by typing and pressing enter.

```shell
127.0.0.1:1698> create foo
(boolean) true
127.0.0.1:1698>
```

The above example creates keyspace called foo and the server responds with a boolean value `true` which is printed to the console.

You can issue any valid segment commands via the CLI. For a full list of commands, [visit this page](https://github.com/segment-dev/segment#list-of-commands).

##### Dealing with whitespace in the command arguments

Sometimes you may come across a scenario where there are spaces in a command argument, for example you are trying to drop a keyspace called `"this keyspace has spaces in its name`", since using the CLI white space is considered as a breaking point, you can wrap the argument containing spaces in double quotes, so to drop the keyspace you would do something like this.

```shell
127.0.0.1:1698> drop "this keyspace has spaces in its name"
(boolean) true
127.0.0.1:1698>
```

This concept applies to all other commands as well.
