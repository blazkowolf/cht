# chtsh

`chtsh` is a Rust CLI for [cht.sh](https://cht.sh).

__I am unaffiliated with cht.sh. This is a hobby project.__

cht.sh's GitHub [repository](https://github.com/chubin/cheat.sh) advertises an
[existing CLI](https://github.com/chubin/cheat.sh#command-line-client-chtsh),
and it is stable and feature complete.

## Usage

```
chtsh <LANGUAGE> [QUERY_PARTS]
```

### Args

| Arg         | Required | Desc                                                                                                                                                                  |
| ----------- |:--------:| --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LANGUAGE    | ✔        | Must be a [valid programming language or topic](https://cht.sh/:list).                                                                                                |
| QUERY_PARTS | ❌       | Freeform parameter where you can ask a question regarding the provided `LANGUAGE`. Defaults to `:list` which will print the known topics for the provided `LANGUAGE`. |


### Options

| Option          | Desc                      |
| --------------- | ------------------------- |
| `-h, --help`    | Print help information    |
| `-V, --version` | Print version information |

_Valid usage examples:_

```sh
chtsh go
chtsh rust error handling
chtsh java stream filter
```

Any constructive feedback is appreciated 🙂
