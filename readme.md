# AOC 2023

---

My solutions for [advent of code 2023](https://adventofcode.com/2023).

This year I'll be doing it in rust, for a change. Since I feel like I understand the language enough to make it as far as I would with any other language, like python.

Also my python is a bit rusty in a bad way, and my rust is rusty in a good way.

I'm also going to be trying out the new (for me) parsing librairy [nom](https://crates.io/crates/nom) this year. If it goes well I can try to not even use regex, but I won't let this restriction hold me back.

---

### How to run:

Use ``cargo run`` to run the newest solution,
or use ``cargo run -- --day=1 --part=1`` or ``cargo run day=1 part=1`` to run a specific day and part,
you can also alias ``--day`` with ``-d`` and ``--part`` with ``-p`` or omit them entirely.
Use ``cargo run all`` to run all days and see how fast they all get solved.

The cli will also tell you how long it took to run the code, and if it was on debug or release mode

You can also use ``cargo test`` to run the test with the provided test cases.
When I'm developing I like to use ``cargo watch -x test`` to automatically run the tests while programming,
this does require the [cargo watch](https://crates.io/crates/cargo-watch) crate.

---

### My thoughts on each day

A bit of text describing my experience with creating a solution for a specific day.

* day 1: Parsing more like suffering (I took way too long on trying to actually parse this stuff, eventhough working with strings is more effective but ugly).

* day 2: Actually way easier than day 1 and I had fun making it work.

* day 3: Challanging but still fun to solve, also my parsing in suboptimal again...

* day 4: First day parsing actually turned out good.

* day 5: I spend two entire days of my life on merging ranges, but it worked out in the end... I guess.

* day 6: Very doable I just suck but I made a giant mistake not related to the code I won't tell you about.

* day 7: Do not speak ill of the Ord Pyramid for it is holy.

* day 8: The input is set up in a highly convinient way for getting the end result, it's almost like someone made it. :thinking:

* day 9: I loved how many iterator methods I could use today, if there were just a little more I wouldn't even need the while loop.

* day 10: A [friend of mine](https://github.com/MauritsWilke/AdventOfCode) and I went completely crazy on part 2 today, most fun so far.

* day 11: I happened to take the approach which was required for part 2, which made it an easy to do part 2 for me.

* day 12: Part 2 doesn't actually work yet because it's too slow, I'll make an actual note once it's finished

* day 13: Kind of uninteresting day, not much to say about it.

---

Thanks for reading!