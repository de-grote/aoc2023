# AOC 2023

---

My solutions for [advent of code 2023](https://adventofcode.com/2023).

This year I'll be doing it in rust, for a change. Since I feel like I understand the language enough to make it as far as I would with any other language, like python.

Also my python is a bit rusty in a bad way, and my rust is rusty in a good way.

I'm also going to be trying out the new (for me) parsing librairy [nom](https://crates.io/crates/nom) this year. If it goes well I can try to not even use regex, but I won't let this restriction hold me back.

---

### How to run:

Use ``cargo run`` to run the newest solution,
or use ``cargo run -- --day=1 --part=1`` to run a specific day and part,
you can also alias ``--day`` with ``-d`` and ``--part`` with ``-p`` or omit them entirely.
(Also as a "feature" you can omit all dashes from day and part).

You can also use ``cargo test`` to run the test with the provided test cases.
When I'm developing I like to use ``cargo watch -x test`` to automatically run the tests while programming,
this does require the [cargo watch](https://crates.io/crates/cargo-watch) crate.

---

### My thoughts on each day

A bit of text describing my experience with creating a solution for a specific day.

* day 1: parsing more like suffering (I took way too long on trying to actually parse this stuff, eventhough working with strings is more effective but ugly).

* day 2: actually way easier than day 1 and I had fun making it work.

---

Thanks for reading!