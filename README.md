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

```javascript
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
| `catj earth_meteorite_landings.json` | 267.5 ± 10.7 | 250.7 | 283.4 | 11.3 |
| `meowj earth_meteorite_landings.json` | 23.7 ± 1.4 | 21.2 | 28.5 | 1.0 |

---

inspired by [catj](https://github.com/soheilpro/catj), but without dependencies on node/npm
