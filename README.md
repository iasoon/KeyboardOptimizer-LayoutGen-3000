KeyboardOptimizer LayoutGen 3000
================================

What is this?
-------------

Most software for optimizing keyboard layouts is quite inflexible, making
big assumptions about what your keyboard physically looks like, how you operate
it, and what is considered 'good' and 'bad'. This project is my attempt at
fixing that.


How does it work?
-----------------

As input, you specify a set of keys, a set of values that can be assigned to
those keys (so, 'letters'). Then, a set of constraints can be applied to those
keys (such as, 'a' and 'A' should be on the normal/shifted layers of the same
key). This way, we build a binary constraint satisfaction problem, of which the
solutions are all valid keyboard layouts.
Walking the solution space of such a CSP is not trivial, but since the
constraints applied here are typically not that restricting, it kind of works.

Then, we specify a way to score a layout. The main way of doing this is by
counting how often certain sequences of letters appear in the text you will be
typing (a 'language' model), and combining it with some 'effort' rating of
typing certain key sequences on your physical keyboard. The question is then to
find the mapping between letters and keys that minimizes this effort score.
This is a slight variation of the quadratic assignment problem (QAP),
so I started out by implementing a QAP solver. That's the current status.

Having the user specify the stroke efforts instead of assuming some kind of
rule for this maintains full flexibility in what kind of rules you want to use
for scoring keystrokes. Of course, you can create your own effort model and
write a small script for scoring all key runs you want to consider, and then
hand it over to this program for the heavy lifting.

Cool. So, _does_ it work?
---------------------

No. I'm low-key working on getting it to work, though!

If this project interests you in some way, please do reach out!