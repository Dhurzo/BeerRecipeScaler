# Beer Recipe Scaler

## What?

This is a beer recipe scaler. This program helps you scale a beer recipe (to more or less quantity) using formulas that help with this process.

Some formulas are extracted from the book "How To Brew: Everything You Need to Know to Brew Great Beer Every Time" by John J. Palmer, and others are from homebrewing blogs (cervezadegaraje.com, radicaloh.com, ACCE, etc.).

## How?

1. Clone the repository
2. Run `cargo build`
3. Execute with arguments:

```bash
cargo run -- [OPTIONS]

Options:
  -p, --hops HOPS              Number of hop additions
  -m, --malts MALTS           Number of malts
  -o, --original-volume VOL   Original recipe volume
  -s, --scaled-volume VOL     Scaled recipe volume
  -b, --scale-by METHOD       Scale by "malt" or "volume"
  -h, --help                  Show this help message
```

Forks and pull requests are welcome!

Note: This is not a serious project, it's just a personal project that I want to share with the community (someone might find it helpful! 😄)

## TODO

These features might be added someday:

1. Markdown recipe output format
2. JSON recipe input format
3. ¿GUI interface?

## Status

Status: ABANDONED