# yscalc - Yemeksepeti Joker Calculator

yscalc helps resolve how much each person needs to pay in Yemeksepeti's Joker
discounted prices. It rounds what each person needs to pay to 0.25 and divides
the remainder fairly.  Any remaining extra is left as tip.

## Example Run

Format for each entry is `[ price, [List of Names]]`. Where names are the list
of people sharing the item. `null` is interpreted as "shared amongst all".

In the example below, the 12.0 item is shared amongst everyone, the 22.0 item
is shared by A and B.

Example JSON:

```
[
    [ 12.0, null ],
    [ 17.0, [ "Person A" ] ],
    [ 22.0, [ "Person A", "Person B" ] ],
    [ 10.0, [ "Person B"] ],
    [ 9.0, [ "Person C"] ]
]
```

### Running


```
$ cargo run -- list.json

Total: 45.00
Ratio = 45.00 / 70.00 = 0.64

1 Person A: 32.00 (4.00, 17.00, 11.00)
  20.50 (20.57)

2 Person B: 25.00 (4.00, 11.00, 10.00)
  16.00 (16.07)

3 Person C: 13.00 (4.00, 9.00)
  8.50 (8.36)

Sum: 45.00
Remainder: 0.00
```

For example, here Person C will be paying 8.50 which was rounded up from 8.36.
