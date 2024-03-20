# LeetCode solutions files.

The builds here use [cargo_make](https://lib.rs/crates/cargo-make). This approach was inspired by the devs at
[serenity-rs](https://github.com/serenity-rs/serenity) where they have a Discord Bot creation framework with examples.
Here its just solutions for some LeetCode problems though. The main LeetCode
problem set targeted is Trees ([BFS](https://en.wikipedia.org/wiki/Breadth-first_search) - Breath-First Search & [DFS](https://en.wikipedia.org/wiki/Depth-first_search) - Depth-First Search)
since I also decided to make this library in order to learn how to solve them.

In order to run any of the examples you need to do the following:

```sh
cargo install --force cargo-make
git clone https://github.com/1kill2steal/leetcode-trees-rs
cd leetcode-trees-rs
```

Then to make a LeetCode (LC) problem set you need to run

```sh
cargo make 100 # Replace this number with the solution number you want.
```

You can always check which solutions are available by looking at the
subdirectories of this directory.
