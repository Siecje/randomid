Random ID
---

Inspired by [Antirez's blog post](http://antirez.com/news/99), I figured I'd try to learn some Rust by implementing his strategy for creating unique IDs.

The logic is simple. On initialization, you:

- start a counter
- read some bytes from your machine's CSPRNG as a seed

Then, every function call:

- calculate the HMAC-SHA1 of the concatenation of your seed and your counter's value
- increment the counter

I really feel like this is a lot more clever than Antirez is given credit for. In terms of collision resistance, this is exceptional: HMAC-SHA1 is already about as good as it gets, plus you will get a different seed each time your code in initialized, and if that's not already enough, you concatenate the value of your counter with your seed so that the signature of each hash is different.

Consider this: the odds of two SHA1's being the same is 1 in 2^160. As far as I know, the probability of reading two of the same values from dev/urandom is 1 in 2^128. Combined, that's 1 in 2^20480. When you add that counter into the equation, even if you reinitialized your code 2^20480 times, the probability of a collision is still 2^20480. In a real application though, your odds aren't this good, plus there's the birthday paradox (except I can't remember how to calculate it lol), but still... for trying to generate random ids, this is wayyy overkill, but efficient enough that it's totally practical.
