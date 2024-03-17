# xensieve-py


<a href="https://github.com/flexatone/xensieve-py/actions/workflows/ci.yml">
    <img style="display: inline!important" src="https://img.shields.io/github/actions/workflow/status/flexatone/xensieve-py/ci.yml?branch=default&label=CI&logo=Github"></img>
</a>

<a href="https://codecov.io/gh/flexatone/xensieve-py">
    <img style="display: inline!important" src="https://codecov.io/gh/flexatone/xensieve-py/branch/default/graph/badge.svg"></img>
</a>


An implementation of the Xenakis Sieve, providing a Sieve from a string expression that filters integer sequences into iterators of integers, Boolean states, or interval widths. Sieves are built from Residuals, defined as a modulus (M) and a shift (S), notated `M@S`. Sieve string expressions, and Sieve structs, support complementation, intersection, symmetric difference, and union operations on Residuals with operators `!`, `&`, `^` and `|`, respectively.

The Xenakis Sieve is tool for generating discrete interval patterns. Such patterns have boundless applications in creative domains: the Xenakis Sieve can be used to generate scales or multi-octave pitch sequences, rhythms and polyrhythms, and used to control countless other aspects of pictorial or architectural design.

This Rust implementation follows the Python implementation in Ariza (2005), with significant performance and interface enhancements: https://direct.mit.edu/comj/article/29/2/40/93957

Code: https://github.com/flexatone/xensieve-py

<!-- Crate: https://crates.io/crates/xensieve -->


# Rust Implementation

Code: https://github.com/flexatone/xensieve-rs

Docs: https://docs.rs/xensieve

Crate: https://crates.io/crates/xensieve
