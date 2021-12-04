# [YAML, fun]

Just an experimental project implementing embedded functional scripting language based on YAML syntax.

## Concept

Code:

```yaml
let:
  Maybe:
    rec:
      map:
        lambda: [callback, val]
        do:
          if:
            ==: [val, { :: null }]
          then: val
          else: [callback, val]

      withDefault:
        lambda: [default, val]
        do:
          if:
            ==: [val, { :: null }]
          then: default
          else: val

  (+):
    lambda: [x, y]
    do:
      +: [x, y]

  Pair:
    rec:
      cons:
        lambda: [a, b]
        do:
          lambda: [op]
          do: [op, a, b]

      car:
        lambda: [cons]
        do:
          - cons
          - lambda: [a, b]
            do: a

      cdr:
        lambda: [cons]
        do:
          - cons
          - lambda: [a, b]
            do: b

  cons: [Pair.cons, { :: 1 }, { :: 2 }]

in:
  rec:
    a: [Maybe.map, [(+), { :: 1 }], { :: 5 }]
    b: [Maybe.map, [(+), { :: 1 }], { :: null }]
    c: [Maybe.withDefault, { :: 0 }, { :: null }]
    d:
      - [Maybe.withDefault, { :: 0 }]
      - - [Maybe.map, [(+), { :: 1 }]]
        - - [Maybe.map, [(+), { :: 1 }]]
          - { :: 10 }
    e:
      :>:
        - { :: 10 }
        - [Maybe.map, [(+), { :: 1 }]]
        - [Maybe.map, [(+), { :: 1 }]]
        - [Maybe.withDefault, { :: 0 }]

    f: [Pair.car, cons]
    g: [Pair.cdr, cons]
```

Result:

```
{a: 6, b: null, c: 0, d: 12, e: 12, f: 1, g: 2}
```

## Things that (probably) work

### Constant

```yaml
:: foo
```

### Variable

```yaml
foo
```

### Function

```yaml
lambda: [num1, num2]
do:
  +: [num1, num2]
```

### Function Call

```yaml
[func, arg1, arg2]
```

### Chaining

```
let:
  (+):
    lambda: [x, y]
    do:
      +: [x, y]
in:
  :>:
    - { :: 1 }
    - [(+), { :: 5 }]
    - [(+), { :: 4 }]
```

### Record

```yaml
rec:
  a:
    rec:
      b: { :: { 1: bar, true: baz } }
      '10': { :: foo }
  e: { :: { y: z } }
```

### Record Field Access

```yaml
foo.a.10
```

```yaml
foo.a.b.(1)
```

```yaml
.: [foo, { :: a }, { :: b }, { :: 1 }]
```

### List

```yaml
list:
  - { :: a }
  - { :: 1 }
  - { :: 1.1 }
  - { :: -1 }
  - { :: true }
  - list:
      - { :: nested }
```

### If Else

```yaml
if:
  ==: [:: 2, :: 2]
then: { :: yes }
else: { :: no }
```

### Let In

```yaml
let:
  a: { :: foo }
  b: a
in: b
```

### With

```yaml
let:
  args1:
    rec:
      first: { :: 10 }
      second: { :: 20 }
  args2:
    ::
      third: 30
in:
  with: [args1, args2]
  do:
    +: [first, second, third]
```

### Platform Call

```yaml
platform: import
arg: { :: ./concept.yml }
```

```rust
struct MyPlatform(DefaultPlatform);

impl Platform for MyPlatform { ... }

fn main() {
    let vm = Vm::new(MyPlatform(DefaultPlatform));
    ...
}
```

## Embed into Rust

[Here's how](/examples)
