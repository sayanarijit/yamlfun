# [YAML, fun]

Just an experimental project implementing embedded functional scripting language based on YAML syntax.

## Concept

Code:

```yaml
let:
  x: { $: true }
  y:
    lambda: [boolVal]
    do:
      if: x
      then:
        if: boolVal
        then: { $: yes }
        else: { $: no }
      else: { $: no }
in: [y, $: true]
```

Result:

```
"yes"
```

## Things that (probably) work

### Constant

```yaml
$: foo
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
      b:
        rec:
          c: { $: 1 }
      d: { $: foo }
  e: { $: { y: z } }
```

### Record Field Access

```yaml
.: [foo, a, b, c]
```

```yaml
foo.a.b.c
```

### List

```yaml
list:
  - { $: a }
  - { $: 1 }
  - { $: 1.1 }
  - { $: -1 }
  - { $: true }
  - list:
      - { $: nested }
```

### If Else

```yaml
if:
  ==: [$: 2, $: 2]
then: { $: yes }
else: { $: no }
```

### Let In

```yaml
let:
  a: { $: foo }
  b: a
in: b
```

### With

```yaml
let:
  args1:
    rec:
      first: { $: 10 }
      second: { $: 20 }
  args2:
    $:
      third: 30
in:
  with: [args1, args2]
  do:
    +: [first, second, third]
```

## Embed into Rust

[Here's how](/examples)
