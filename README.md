# osufuck

A parser that turns osu! beatmap objects into [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) commands and executes them.

## Due to a bug in rust .osu parser library, the program can't parse .osu files when they don't have a bookmark added. Until it get's fixed, please add one bookmark to the map in editor before using it.


Comes with debug mode. use `--mode debug` command line argument to activate it. This will start watching the .osu file and will execute the program again whenever the map is saved.

Use `--path` argument to bypass file picker window.

Ruleset:
| osu! object  | brainfuck equivalent |
| ------------- | ------------- |
| note in top half  | + |
| note in bottom half | - |
| slider with two or less anchors * (linear or perfect circle) with last anchor placed on the **left side of the slider head** | < |
| slider with two or less anchors * (linear or perfect circle) with last anchor placed on the **right side of the slider head** | > |
| slider with three or more anchors which rotates counter-clockwise | < |
| slider with three or more anchors which rotates clockwise | > |
| spinner | . |

*: consecutive overlapped anchors are counted as one anchor. So you can have one red anchor in the slider and they will be parsed as `>` or `<`. 

In brainfuck, there is also `,` command, which takes an std input. but i think the goal is to encode any variable into the map itself so i omited that.
