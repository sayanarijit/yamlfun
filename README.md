# [YAML, fun]

Just an experimental project implementing embedded functional scripting language based on YAML syntax.

## Concept

Code:

```yaml
let:
  x: { :: true }
  y:
    lambda: [boolVal]
    do:
      if: x
      then:
        if: boolVal
        then: { :: yes }
        else: { :: no }
      else: { :: no }
in: [y, :: true]
```

Result:

```
"yes"
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
