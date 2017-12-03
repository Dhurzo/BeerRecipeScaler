# Beer_Recipe_Scaler


## What?

This is a beer recipe scaler. This program can help you to scale a beer recipe (to more quantity or to less quantity) using some formulas that helped my with this process.

Some formulas are extracted from the book '' How To Brew: Everything You Need to Know to Brew Great Beer Every Time - John J. Palmer '' and some others are extracted from home brew blogs (cervezadegaraje.com,radicaloh.com,ACCE,etc...)

Version: 0.0.1 Beta

## Why?

I'm tired of doing this process (by hand) every single time I make beer with my tiny 12 liters kit. And of course I want to train Rust (this is my third project using Rust and I like it so much)

## How?

1. clone
2. cargo build
3. execute

```

Hello, beer!
Usage:
        beer_recipe_scaler [OPTIONS]

Scale beer recipe (malts and hops for now)

optional arguments:
  -h,--help             show this help message and exit
  -l,--hops HOPS        Number of hop aditions
  -m,--malts MALTS      Numer of malts
  -o,--originalVolume ORIGINALVOLUME
                        Original recipe volume
  -s,--scaledVolume SCALEDVOLUME
                        Scaled recipe volume / malt
  -b,--scaleby SCALEBY  Scale by malt (malt) or by final volume (volume)

```

Forks and pull requests are welcome.

Note: This is not a serious project it just a personal project that I want to share with the comunitiy (someone can find it helpfull :D)

TODO (This features could be real some day or never :D):

1. Markdown recipe output format
2. JSON recipe input format
3. GUI (98 % sure this never will be implemented ;D)


