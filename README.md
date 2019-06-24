meowj
=====

displays json in a flat format.

*note: still a very new project, no `--help` or anything yet, also recusrive instead of iterative.*

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
| `meowj earth_meteorite_landings.json` | 21.5 ± 1.4 | 20.4 | 30.9 | 1.0 |
| `gron earth_meteorite_landings.json` | 60.3 ± 2.7 | 54.1 | 65.2 | 2.8 |
| `catj earth_meteorite_landings.json` | 218.5 ± 7.9 | 207.6 | 231.6 | 10.2 |

Alternatives
------------

- catj - pretty much the same thing - https://github.com/soheilpro/catj/blob/master/catj.js - Node/NPM
- gron - much more features, such as going _backwards_ given the output of itself, which is quite neat - https://github.com/tomnomnom/gron - Go

---

initially inspired by [catj](https://github.com/soheilpro/catj), but without dependencies on node/npm
