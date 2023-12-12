# Day 04

I initially used a lot of recursion to solve day 04. This caused my solution to be really slow with no optimizations.
Using rust's --release flag on build reduced the time by quite a bit, here are the original results

`Original speed - Debug: 24315 milliseconds, Release: 1617 milliseconds.`

I decided to take a different approach. I realized there was a max amount of wins that could happen for a single ticket. This means that there is a set amount of cards after the current card that is being analyzed. So, instead of looping multiple times into the wins of a card like a tree I could use a circular buffer. I would fill the buffer with the max amount of wins + 1 (for the current card being looked at for wins) and fill each space with a 1 (for the original ticket). I would then take the current card, get it's wins, and also get how many cards of that id I currently have. I would then add the amount of cards I currently have to next following cards, the number of wins I have determines how many of the next amount of cards I add it too. 

With a few tweaks to the card reader as well but finding the amount of wins during the first read of the card, I was able to get the total amount of cards within `3 milliseconds` in debug mode!
