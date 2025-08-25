# RFC9839-rs

A rust implementation of RFC9839 to test for problematic Unicode code points

Inspired by the Go implementation <https://github.com/timbray/RFC9839/tree/main>

## What is RFC9839

[RFC9839](https://www.rfc-editor.org/rfc/rfc9839.html) includes a few
definition for accepted character classes.

* Unicode Scalars
  Any Unicode code point except high-surrogate and low-surrogate code points

* Xml Characters
  Unicode code points that excludes surrogates,
  legacy C0 controls, and the noncharacters U+FFFE and U+FFFF.

* Unicode Assignables
  Unicode code points that are not problematic. This, a proper subset of each
  of the others, comprises all code points that are currently assigned,
  excluding legacy control codes, or that might be assigned in the future.

## Why this crate

* `no_std`
  This crates does not make any allocations and thus can be used on embedded
  systems

* `const fn`
  Functions checking individual characters can all be called in a `const`
  context.

* Well tested
  Every character class is checked against the full `u32` Range of possible
  values.
