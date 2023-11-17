# osufuck

A parser that turns osu! beatmap objects into [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) commands and executes them.

## Due to a bug in rust .osu parser library, the program can't parse .osu files when they don't have a bookmark added. Until it get's fixed, please add one bookmark to the map in editor before using it.


Comes with debug mode. use `--mode debug` command line argument to activate it. This will start watching the .osu file and will execute the program again whenever the map is saved.

Use `--path` argument to bypass file picker window.

## Ruleset

| osu! object                                                                                                                   | brainfuck equivalent |
| ----------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| note in top half                                                                                                              | +                    |
| note in bottom half                                                                                                           | -                    |
| slider with two or less anchors * (linear or perfect circle) with last anchor placed on the **left side of the slider head**  | <                    |
| slider with two or less anchors * (linear or perfect circle) with last anchor placed on the **right side of the slider head** | >                    |
| slider with three or more anchors which rotates counter-clockwise                                                             | [                    |
| slider with three or more anchors which rotates clockwise                                                                     | ]                    |
| spinner                                                                                                                       | .                    |

*: consecutive overlapped anchors are counted as one anchor. So you can have one red anchor in the slider and they will be parsed as `>` or `<`. 

In brainfuck, there is also `,` command, which takes an std input. but i think the goal is to encode any variable into the map itself so i omited that.

## DIY Compiling
If you don't trust the pre-compiled executable. You can build it yourself! installing rust and building projects are really easy.

1. Clone the repository
2. Install rust https://www.rust-lang.org/tools/install
3. Open a terminal in the repository root
4. Type `cargo build --release` to build or type `cargo run --release` to build and run the code!

I didn't provide binaries for other operating systems because the playerbase should be mostly on Windows. Unless you are in `much less audio delay, such wow` group (yeah), then you can use this directions to build it yourselves. Libraries I used are supporting multiple platforms so it shouldn't be a problem. 

Except for `rfd` (file picker) for which you need to install `gtk3` on linux.

| Distribution    | Installation Command     |
| --------------- | ------------------------ |
| Fedora          | dnf install gtk3-devel   |
| Arch            | pacman -S gtk3           |
| Debian & Ubuntu | apt install libgtk-3-dev |

https://docs.rs/rfd/latest/rfd/ for more details
