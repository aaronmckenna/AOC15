# Day 5: Doesn't He Have Intern-Elves For This?
Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
For example:

ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
jchzalrnumimnmhp is naughty because it has no double letter.
haegwjzuvuyypxyu is naughty because it contains the string xy.
dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?
## Language
I wanted to use Clojure for *something* so I think I'll go along the lines of this one. I'm working on a couple of school projects at the moment, and Clojure may take a bit 
to learn, so it may be a bit before an update happens.
## Naming Conventions
flatcase seems to be standard for Clojure, so I will follow the standard on this one.
## Tools
For this I used Light Table, which has some built-in features specifically for Clojure.