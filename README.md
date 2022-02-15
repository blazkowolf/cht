# chtsh

`chtsh` is a Rust CLI for [cht.sh](https://cht.sh).

__I am unaffiliated with cht.sh. This is a hobby project.__

cht.sh's GitHub [repository](https://github.com/chubin/cheat.sh) advertises an
[existing CLI](https://github.com/chubin/cheat.sh#command-line-client-chtsh),
and it is stable and feature complete.

## Usage

```sh
chtsh <LANGUAGE> [QUERY_PARTS]
```

`LANGUAGE` is required and must be a [valid programming language or topic](https://cht.sh/:list).

`QUERY_PARTS` is an optional freeform parameter where you can ask a question
regarding the provided `LANGUAGE` value. If left empty, `QUERY_PARTS`'s value defaults to
`:list` which will print the known topics for the value provided in `LANGUAGE`.

_Valid usage examples:_

```sh
chtsh go
```

```sh
chtsh rust error handling
```

```sh
chtsh java stream filter
```

Any constructive feedback is appreciated 🙂
