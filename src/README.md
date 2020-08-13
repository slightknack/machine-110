# machine-110
The other day, I ran across a generic competition
to complete some simple in an as obfuscated manner possible.
The specifics of the contests are irrelevant -
needless to say, having heard of rule-110-based Turing machines,
I though the best route would be to implement one,
then write a program that compiled some arbitrary simple language
down to a rule 110 starting seed.
(Rule 110 is a common form of 1d cellular automata, by the way.)

Quite obviously, I quickly ran into some roadblocks.
Although implementing rule 110 is quite trivial,
compiling to rule 110 is nigh impossible,
especially given the scattered state of the field.
(Not many people dedicate their lives to studying trivial automata;
those that do don't spend much time trying to explain it in sensible terms.)

As a result, I've been working through a couple of papers
(see the [Original Introduction](https://wpmedia.wolfram.com/uploads/sites/13/2018/02/15-1-1.pdf),
a [More Thorough Explanation](https://web.archive.org/web/20160303190954/https://uncomp.uwe.ac.uk/genaro/Papers/Papers_on_CA_files/repCTSR110.pdf), and
the [Discussion of P-Completeness](https://link.springer.com/chapter/10.1007/11786986_13).)
These papers, while enlightening, act as all papers do -
they'll never answer any specific questions you have that aren't clarified
in the one or two sentences they provide.

I've been thinking about reaching out the respective experts to get some help
on this matter, but for now I assume it's best I read and work through
the provided work the best I can first.

## Is it done yet?
No.

## How far along are you?
I've implemented:
- All basic 'rule `{ x | 0 <= x < 256 }`' automata (including rule 110).
- A tiling strategy that's required for rule 110 for function as a Turing machine.
- Various gliders/etc. for rule 110.
- A Cyclic Tag System and the requisite infrastructure to compile said system
  to rule 110.

What's left? (that I know of):
- Implementing methods that can generate `up` and `over` spacing
  as referenced in the original paper.
- Verifying that the various interactions between gliders are indeed correct.
- Implementing additional gliders and so on.
- Compiling CTSs into rule 110 states.
- Extracting information from rule 110 programs.

So yeah, quite a lot.

## Why the heck are you doing this, masochist?
I really don't know.
If you'd like to help, or just even offer moral support,
that'd be much appreciated.

## Who the heck are you, anyway?
Just a random high-school student with too much time on his hands and a
penchant for doing things the hard way.

## What are you even doing? This makes 0 sense, silly.
I hope to explain some things about rule 110 and my understanding of
what I've read so far. If you read closely, you'll see some serious holes
in my understanding. If you just skim it, it might look pretty impressive.

### Part I - What are *Cellular Automata*, Anyway
Cellular Automata are a class of rules that when applied to a structure
will produce a new similar structure.
The most famous is arguably the late Conway's *Game of Life*,
though *many* (dare I say infinitely) more exist.
Rule 110 is a one-dimensional automata that has
been proven to be Turing complete.
for a long time, it was though to be the simplest Turing-complete system
that existed, but most self-respecting people don't believe that to be
the case anymore after the proof of a simpler system.

### Part II - Wolfram's Rules (a brief history)
Steven Wolfram, the same *extraordinaire* that recently received some criticism
for saying that space-time is just a (beautiful) jumble of graphs,
has always been fascinated with cellular automata.

Being the grandfather of modern cellular automata,
Wolfram discovered Rule 110 *a long* time ago.
He conjectured that it was Turing complete, which was proved in 2004.

### Part III - The Proof
To understand how rule 110 is Turing complete, we first need to understand
Cyclic Tag Systems (CTS).
A CTS is a simple machine that works as follows:

There are a set of rules which are binary strings. For example:
```
"0"
"1010"
"111"
""
"1100101"
```

are all valid rules.
These rules are ordered.
Additionally, CTSs have memory, which is also just a binary string.
To evaluate a CTS, simply do the following:

1. Remove the first item from the memory binary string.
2. If that item is true (i.e. `'1'`), right-append the next rule to memory
3. Rotate the rules so the first rule becomes the last.
4. Repeat.

Some very intelligent person figured out that this system was Turing complete
by relating it to an extended version of the Collatz Conjecture that is
also Turing complete.

As it turns out, CTSs can be simulated inside rule 110 through the use of
gliders, making the rule itself Turing complete.

### Part IV - Gliders
Rule 110 is unique as it is one of few rules to contain gliders,
or stable structures that cyclically change positions over time.

![Rule 110 Gliders](https://upload.wikimedia.org/wikipedia/commons/0/0d/Ca110-structures2.png)

Rule 110 has a quite a few gliders, the most important ones in this construction
being the A4 glider, the Ē glider, and the C2 glider, as seen in the picture
above, when read left-to-right. (Silly names, right? I'll call them A4 -> lazer,
Ē -> wiggler, and C2 -> stacker from now on.)

When these gliders collide, new gliders emerge from the collision.
Which gliders are produced is dependent on the spacing between the gliders.

![Rule 110 Collisions](https://upload.wikimedia.org/wikipedia/commons/1/18/Ca110-interaction2.png)

For example, a lazer hitting a wiggler might create a stacker,
as seen on the left, or may re-emit the lazer, as seen on the right.
By shooting lazers at wigglers and stackers, complex behaviour emerges.

But how can we control this complexity?

### Part V - The Space between atoms: Up, and Over
As you can see in the above pictures, there's this repeating triangular pattern.
Skilled proponents of archaic automata call this pattern ether.

> TODO: expand on this.

### Part VI - Classifying Glider Interactions
> TODO: write.

### Part VI - The architecture of a beastly machine
With that all cleared up, how can we embed CTSs inside rule 110?
We need to figure out how to produce an initial starting state
that contains a set of gliders that when run interact in complex ways
to perform the computation we so desire.

> TODO: elaborate.

# Closing thoughts
This is proving to be quite an adventure.
If you'd like to reach out and contact me,
you can find my contact details on my [website](https://www.slightknack.dev/home).

Have a nice day!
