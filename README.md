# Sigma Types in Rust

This crate lets you explicitly represent structures like _sorted vectors_ as types of their own.

## Features
- `Deref`:
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
- `no_std` (and no `alloc`):
  all features, including error messages (via `::core::fmt::Display`),
  work without the standard library and without heap allocation.

## What does this have to do with $\Sigma$-types?
Some languages, like [Coq](https://github.com/coq/coq?tab=readme-ov-file),
implement type systems that offer _sigma types_ (or $\Sigma$-types),
which represent a term (say, $A$) alongside another term (say, $B$)
whose _type_ can depend on the _value_ of $A$.
Since the type of $B$ can depend on the value of $A$,
$B$ often represents a proof that some property holds of $A$.
(If exactly how is unclear and you have a few minutes and a mind to be blown,
see [the Curry-Howard correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence).)

We implement a subset of sigma types in which $B$ is decidable
and reduces to a `Result` whose error type is an `Option`al error message.
In this case, a sigma type can represent a value for which some _computable_ property is checked.

### Why not call this library `invariant` or something?

It [seems to be taken](https://github.com/pthariensflame/invariant.rs)
by a library that requires `std` and `alloc`
with larger structs that carry around their checks in release builds.
This is all fine, but I have a different use-case, and this aims to be more generally applicable.

Plus, sigma types are theoretically interesting, and I'd like to evangelize a bit.
