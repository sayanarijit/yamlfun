# (): [YAML, fun]

Just an experimental project implementing embedded functional scripting language based on YAML syntax.

## Concept

Code:

```yaml
let:
  x: {$: true}
  y:
    lambda: [boolVal]
    do:
      if: x
      then:
        if: boolVal
        then: {$: yes}
        else: {$: no}
      else: {$: no}
in:
  (): [y, $: true]
```

Result:

```
String("yes")
```

## Things that (probably) work

### Constants:

```yaml
$: foo
```

### Variables:

```yaml
foo
```

### Functions

```yaml
lambda: [num1, num2]
do:
  +: [num1, num2]
```

### Function Calls

```yaml
(): [func, arg1, arg2]
```

### If Else

```yaml
if:
  ==: [$: 2, $: 2]
then: {$: yes}
else: {$: no}
```

### Let In

```yaml
let:
  a: {$: foo}
  b: a
in:
  b
```

## Embed into Rust

[Here's how](/examples)
