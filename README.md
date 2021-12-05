# [YAML, fun]

Just an experimental project implementing embedded functional scripting language based on YAML syntax.

## Concept

Code:

```yaml
let:
  (+):
    lambda: [x, y]
    do:
      +: [x, y]

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

  List:
    rec:
      head:
        lambda: [list]
        do:
          case: list
          of:
            list:
              lambda: [head, tail]
              do: head

      tail:
        lambda: [list]
        do:
          case: list
          of:
            list:
              lambda: [head, tail]
              do: tail

  Cons:
    rec:
      new:
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

  cons: [Cons.new, { :: 1 }, { :: 2 }]
  foobar:
    - (+)
    - { :: foo }
    - { :: bar }

  things:
    - (+)
    - list:
        - foobar
        - cons
        - Maybe
        - [Cons.car, cons]
    - :: [1, 1.2, -9, null, bar]
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

    f: [Cons.car, cons]
    g: [Cons.cdr, cons]
    h: [List.head, things]
    i:
      :>:
        - [List.tail, things]
        - List.head
        - Cons.car
```

Result:

```
{a: 6, b: null, c: 0, d: 12, e: 12, f: 1, g: 2, h: "foobar", i: 1}
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

```yaml
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

### Record field update

```yaml
update: foo
set:
  a: { :: bar }
  oldFoo: foo
unset: [e]
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

### Case Of

```yaml
let:
  handle:
    lambda: [var]
    do:
      case: var
      of:
        (): { :: this null }
        bool:
          lambda: [b]
          do: { :: this is a bool }
        int:
          lambda: [n]
          do: { :: this is an int }
        float:
          lambda: [f]
          do: { :: this is a float }
        string:
          lambda: [first, rest]
          do: first
        function:
          lambda: [f]
          do: { :: this is a function }
        list:
          lambda: [head, tail]
          do: head
        rec:
          lambda: [r]
          do: r.foo
        _:
          lambda: [wtf]
          do: { :: 'wtf??' }
in:
  list:
    - [handle, { :: null }]
    - [handle, { :: true }]
    - [handle, { :: 1 }]
    - [handle, { :: 1.1 }]
    - [handle, { :: foo }]
    - [handle, handle]
    - [handle, { :: [a, b] }]
    - [handle, { :: { foo: bar } }]
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
