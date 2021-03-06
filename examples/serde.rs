const VAL: &str = r#"
invoice: 34843
date   : 2001-01-23
bill-to: &id001
  given  : Chris
  family : Dumars
  address:
    lines: |
      458 Walkman Dr.
      Suite #292
    city    : Royal Oak
    state   : MI
    postal  : 48046
ship-to: *id001
product:
- sku         : BL394D
  quantity    : 4
  description : Basketball
  price       : 450.00
- sku         : BL4438H
  quantity    : 1
  description : Super Hoop
  price       : 2392.00
tax  : 251.42
total: 4443.52
comments:
  Late afternoon is best.
  Backup contact is Nancy
  Billsmer @ 338-4338.

wtf:
  1: true
  false:
    [foo, 0, 9.9]
"#;

fn main() {
    let val: yamlfun::Value = yamlfun::yaml::from_str(&VAL).unwrap();
    println!("{}", &val);

    let val = serde_yaml::to_string(&val).unwrap();
    println!("{}", &val);

    let val: serde_yaml::Value = yamlfun::yaml::from_str(&val).unwrap();
    let val = serde_yaml::to_string(&val).unwrap();
    println!("{}", &val);
}
