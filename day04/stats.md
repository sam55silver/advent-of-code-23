# Day 04

Lots of recursion was used to solve day 04. This caused my solution to be really slow with no optimizations.
Using rust's --release flag on build reduced the time by quite a bit, here are the original results

`Original speed - Debug: 24315 milliseconds, Release: 1617 milliseconds.`

This is still very slow so I decided to try and speed things up with caching.
