# [YAML, fun]

Just an experimental project implementing embedded functional scripting language based on YAML syntax.

API docs for the standard library: [src/Yaml](https://github.com/sayanarijit/yamlfun/tree/main/src/Yaml).

## Why?

Being fully compatible with YAML syntax means that, it will be very easy to
implement the runtime in a language that can parse YAML, because most of the
parsing logic will be taken care of by the YAML parser. So, we can expect
yamlfun code to be runnable in a wide range of platforms.

YAML has a rich ecosystem of tooling. So, if a formatter can format YAML code,
it can also format yamlfun code.

YAML is widely popular. So, the learning curve of yamlfun syntax will be lower.

The script can be embedded into plain YAML, without losing all the benefits of
pure YAML code. Which makes it a great alternative configuration language.

YAML syntax is surprisingly good at expressing functional programming
logic. Anyone familiar with Elm, Haskell, Nix or Lisp should be able to read
the code without much effort investment.

But most important of all, it's fun.

## Concept

Code:

```yaml
:let:
  (++):
    :lambda: [a, b]
    :do:
      :++: [a, b]

  Cons:
    :rec:
      new:
        :lambda: [a, b, op]
        :do: [op, a, b]

      car:
        :lambda: [cons]
        :do:
          - cons
          - :lambda: [a, b]
            :do: a

      cdr:
        :lambda: [cons]
        :do:
          - cons
          - :lambda: [a, b]
            :do: b

  cons: [Cons.new, { :: 1 }, { :: 2 }]
  foobar:
    - (++)
    - { :: foo }
    - { :: bar }

  things:
    - (++)
    - :list:
        - foobar
        - cons
        - Maybe
        - [Cons.car, cons]
    - :: [1, 1.2, -9, null, bar]
:in:
  :rec:
    a:
      :|>:
        - [Maybe.just, { :: 10 }]
        - [Maybe.map, [(+), { :: 1 }]]
        - [Maybe.map, [(+), { :: 1 }]]
        - [Maybe.withDefault, { :: 0 }]

    b: [Cons.car, cons]
    c: [Cons.cdr, cons]
    d:
      - [Maybe.withDefault, { :: null }]
      - - [List.head, things]
    e:
      :|>:
        - [List.tail, things]
        - List.head
        - [Maybe.withDefault, { :: null }]
        - Cons.car
```

Result:

```
{a: 12, b: 1, c: 2, d: "foobar", e: 1}
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
:lambda: [num1, num2]
:do:
  :+: [num1, num2]
```

### Function Call

```yaml
[func, arg1, arg2]
```

### Chaining

```yaml
:|>:
  - { :: 1 }
  - [(+), { :: 5 }]
  - [(+), { :: 4 }]
```

### Record

```yaml
:rec:
  a:
    :rec:
      b: { :: { 1: bar, true: baz } }
      '10': { :: foo }
  e: { :: { y: z } }
```

### Record Field Access

```yaml
foo.a.10
```

```yaml
foo.a.b.$1
```

```yaml
:get: [foo, { :: a }, { :: b }, { :: 1 }]
```

### Record Field Update

```yaml
:update: foo
:set:
  a: { :: bar }
  oldFoo: foo
:unset: [e]
```

### List

```yaml
:list:
  - { :: a }
  - { :: 1 }
  - { :: 1.1 }
  - { :: -1 }
  - { :: true }
  - :list:
      - { :: nested }
```

### If Else

```yaml
:if:
  :==: [:: 2, :: 2]
:then: { :: yes }
:else: { :: no }
```

### Let In

```yaml
:let:
  a: { :: foo }
  b: a
:in: b
```

### With

```yaml
:let:
  args1:
    :rec:
      first: { :: 10 }
      second: { :: 20 }
  args2:
    ::
      third: 30
:in:
  :with: [args1, args2]
  :do:
    :+: [first, second, third]
```

### Case Of

```yaml
:let:
  handle:
    :lambda: [var]
    :do:
      :case: var
      :of:
        :==:
          1: { :: this is one }
          []: { :: this is empty list }
          bar: { :: this is bar }
          null: { :: this null }
          true: { :: this is a bool }
          false: { :: this is a bool }
        :int:
          :as: n
          :do: { :: this is an int }
        :float:
          :as: f
          :do: { :: this is a float }
        :string:
          :as: [first, rest]
          :do: first
        :function:
          :as: f
          :do: { :: this is a function }
        :list:
          :as: [head, tail]
          :do: head
        :rec:
          :as: r
          :do: r.foo
:in:
  :list:
    - [handle, { :: null }]
    - [handle, { :: true }]
    - [handle, { :: 1 }]
    - [handle, { :: 2 }]
    - [handle, { :: 1.1 }]
    - [handle, { :: foo }]
    - [handle, { :: bar }]
    - [handle, handle]
    - [handle, { :: [] }]
    - [handle, { :: [a, b] }]
    - [handle, { :: { foo: bar } }]
```

### Platform Call

```yaml
:platform: import
:arg: { :: ./concept.yml }
```

```rust
struct MyPlatform(DefaultPlatform);

impl Platform for MyPlatform { ... }

fn main() {
    let vm = Vm::new(MyPlatform(DefaultPlatform)).unwrap();
    ...
}
```

## Embed into Rust

[Here's how](https://github.com/sayanarijit/yamlfun/tree/main/examples)

## Contribute

See [CONTRIBUTING.md](https://github.com/sayanarijit/yamlfun/tree/main/CONTRIBUTING.md)
