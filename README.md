meowj
=====

[![CircleCI](https://circleci.com/gh/jackharrhy/meowj.svg?style=svg)](https://circleci.com/gh/jackharrhy/meowj)

displays json in a flat format.

*note: still a very new project, no `--help` or anything yet, also recusrive instead of iterative.*

Install
-------

Via Cargo

```sh
cargo install meowj
```

Usage
-----

```sh
cat example.json | meowj
```

or

```sh
meowj example.json
```

example.json

```json
{
  "array": [
    1,
    2,
    3
  ],
  "boolean": true,
  "color": "#82b92c",
  "null": null,
  "number": 123,
  "object": {
    "a": "b",
    "c": "d",
    "e": "f"
  },
  "string": "Hello World"
}
```

meowj output

```txt
.array[0] = 1
.array[1] = 2
.array[2] = 3
.boolean = true
.color = "#82b92c"
.null = null
.number = 123
.object.a = "b"
.object.c = "d"
.object.e = "f"
.string = "Hello World"
```

Benchmarks
----------

Using this test data:

```sh
curl "https://data.nasa.gov/resource/y77d-th95.json" > earth_meteorite_landings.json
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `meowj earth_meteorite_landings.json` | 30.1 ± 1.4 | 28.7 | 34.9 | 1.0 |
| `gron earth_meteorite_landings.json` | 60.8 ± 1.9 | 56.1 | 64.9 | 2.0 |
| `tabbyj --file earth_meteorite_landings.json` | 179.2 ± 4.8 | 172.3 | 189.4 | 5.9 |
| `catj earth_meteorite_landings.json` | 218.6 ± 7.1 | 207.4 | 231.4 | 7.3 |

Alternatives
------------

- catj - pretty much the same thing - https://github.com/soheilpro/catj/blob/master/catj.js - Node/NPM
- tabbyj - also similar - https://github.com/nint8835/tabbyj - Python
- gron - much more features, such as going _backwards_ given the output of itself, which is quite neat - https://github.com/tomnomnom/gron - Go

---

initially inspired by [catj](https://github.com/soheilpro/catj), but without dependencies on node/npm
