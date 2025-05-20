Starting simple. This is a basic planning page for me - what/how I intend to implement things. I will return to this document folloring strict reading - this is my barebone thoughts going into the project.
I'd also imagine it sets the tone for this project. I'd like it to be professional enough I can showcase it, but in the end, I'm doing this for myself. I can have a *little* fun.
General (EARLY!!!) idea is to initialize a regex pattern as a collection of tokens - with each token implementing a "match" trait.
- I might be getting my terminology conflated here (this might not follow a strict definition of a token), but my "token"s should be able to be defined recursively. A single token could be `(abc)+`, while it in fact contains the tokens `a`,`b`, and `c`.
- - then, calling the match function on a token should recurse down until it gets to the lowest-level match. in the above example, calling match on `(abc)+` should result in the token object representing `(abc)+` attempting to call match on 
- Any ocurrence indicators should similarly take ownership of a token, and be a token themselves. As with the above example, the full heirarchy would be `(abc)+`, which contains the token `(abc)`, which contains the tokens `a`,`b`, and `c`.
- Then, any full regex pattern is just as well a single matchable token - think of it like a root node in a tree.
A token should have an idea of its state - on generation(?), it will all possible matches within its context.
- This can be saved as a vector of integers, with the ints being the length of the match.
- - This decision might make backreferences evil, so I may re-evaluate later.
- then, calling the match function on it should have it return the last match it encountered (the longest one)
- - this should hopefully be as simple as a vec.pop()
- - this should work well enough with lazy operators - just reverse the vector.
- Maybe have two phases? Generation and initialization.
This is an inherently heirarchical structure, and it's without need for parent references so I shouldn't run into any huge issues with the ~~reddit mod~~ borrow checker.

encountered problems/issues
- Can't literal match a zero space (`()` in regex)
- - Will need a custom matcher struct for that, then
- Max cleanliness means that we should really only have one string that the entire regex class can match on, rather than having each matchable token have a copy of or pointer to the string.
- - not really an issue per say but its a little bit nastier to implement
- parsing for optionals (`|` in regex) is a little bit trickier, since there's no upfront indication of their presence. 
- - Multiple passes necessary?
- - Or, have the root token (see above notes on implementation) work more like an B-Tree, and split/grow up upon encountering an option?