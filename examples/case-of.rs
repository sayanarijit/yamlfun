use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

const CASE: &str = r#"
:let:
  handle:
    :lambda: [var]
    :do:
      :case: var
      :of:
        :==:
          1: {:: this is one}
          []: {:: this is empty list}
          bar: {:: this is bar}
          null: {:: this null}
          true: {:: this is a bool}
          false: {:: this is a bool}
        :int:
          :as: n
          :do: {:: this is an int}
        :float:
          :as: f
          :do: {:: this is a float}
        :string:
          :as: [first, rest]
          :do: first
        :function:
          :as: f
          :do: {:: this is a function}
        :list:
          :as: [head, tail]
          :do: head
        :rec:
          :as: {foo_: {:: foo}}
          :do: foo_
:in:
  :list:
    - [handle, {:: null}]
    - [handle, {:: true}]
    - [handle, {:: 1}]
    - [handle, {:: 2}]
    - [handle, {:: 1.1}]
    - [handle, {:: foo}]
    - [handle, {:: bar}]
    - [handle, handle]
    - [handle, {:: []}]
    - [handle, {:: [a, b]}]
    - [handle, {:: {foo: bar}}]
"#;

fn main() {
    let vm = Vm::new(DefaultPlatform).unwrap();

    let case: Expr = yaml::from_str(CASE.trim()).unwrap();
    let case = vm.eval(case).unwrap();
    println!("{}", case);
}
