# $\Sigma$-Types in Rust
### Types automatically checked for invariants in debug builds only.

This crate lets you explicitly represent structures like _sorted vectors_ as types of their own.

## Features
- `Deref`, `AsRef`, and `Borrow`:
  if `x` implements `x.foo(..)`,
  then a $\Sigma$-type `s` wrapping `x`
  automatically implements `s.foo(..)`.
- No runtime cost in release builds:
  When debug assertions are disabled,
  $\Sigma$-types are exactly the same size as their raw type,
  and all methods are fully inlined with no extra code added.
- All checks automatically inserted:
  No more covering your code in `debug_assert!(..)`s
  (and inevitably missing some necessary checks).
  This is an extremely simple crate,
  but its main aim is to silently insert
  nothing more or less than each necessary check,
  and to allow libraries to credibly promise
  invariants about their functions' return values
  with an easily unwrappable, lightweight type
  that disappears in release builds.
- Zero-size wrapper type (`repr(transparent)`):
  Wrapping a `T` in `Sigma<T, ..>` creates a type that uses
  exactly the same binary representation as `T`;
  all it does is add an extra `PhantomData` field.
- `no_std` (and no `alloc`):
  all features, including error messages (via `::core::fmt::Display`),
  work without the standard library and without heap allocation.

## What does this have to do with $\Sigma$-types?
Some languages, like [Coq](https://github.com/coq/coq?tab=readme-ov-file),
implement type systems that offer _sigma types_ (or $\Sigma$-types),
which represent a term (say, $A$) alongside another term (say, $B$)
whose _type_ can depend on the _value_ of $A$.
Since the type of $B$ can depend on the value of $A$,
the type of $B$ often represents a proof that
some property holds of this particular value for $A$.
(If exactly how is unclear and you have a few minutes and a mind to be blown,
see [the Curry-Howard correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence).)

We implement a subset of $\Sigma$-types in which $B$ represents a decidable proposition.
To decide whether a proof exists to inhabit the type of $B$ (which isn't directly represented),
the programmer supplies a Rust function that takes the value of $A$ and returns a `Result`.

### Why not call this library `invariant` or something more clear?

`invariant` [is already a similar library](https://github.com/pthariensflame/invariant.rs),
but it requires `std` and `alloc`, and it uses larger structs that run checks in release builds.
That's fine, but I have a different use-case, and this aims to be more generally applicable.

Plus, $\Sigma$-types are theoretically interesting, and I'd like to evangelize a bit.
