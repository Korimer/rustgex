# Parsing thoughts:
Include a buffer of all read characters.
Note what kind of token the buffer currently contains (eg, literal, special, meta)
Upon encountering a character incompatible with the current token, flush the buffer - and generate a new token (either from scratch, or to wrap the previous token)

## Given the current token, know what next to expect.
A literal can be followed by a meta or another literal
A special can only be followed by a meta
an exact counted meta cannot be followed
a optional, range counted, or indefinite can be followed by a behavioral operator (`?` for lazy or `+` for possessive)

If something is followed by something expected, that something serves to wrap it. (for example, a special (`.`) expects a meta like `+` - so, `+` could wrap it)
