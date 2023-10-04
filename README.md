# pokerus

`pokerus` is a collection of crates for querying and working with data from and
about the Pokémon core series games, and the file formats and data structures
used by them.

## Current State

Please note that `pokerus` is presently pre-pre-release. Its internal and
external data structures are in a state of high churn and its functionality is
nowhere close to complete. No APIs should be considered stable.

## Components

### `pokerus_data`

This crate provides programmatic access to game data, such as base stats for a
given species in a given set of games, the name of a given move in a given
official language, etc. It provides structures representing game mechanics and
APIs for querying them.

### `pokerus_formats`

This crate provides tools for reading and writing save files and the Pokémon
contained therein, including the common formats for distributing individual
Pokémon data.

### `protean`

This crate provides APIs for converting various sources of game data into static
Rust structures and functions. It is the basis of code generation for
`pokerus_data`.

## Intended Scope

### What We Want It to Be

There are quite a few sources of Pokémon data out there, with a variety of
advantages and disadvantages. However, few of them appear to target programmatic
access to data across the core Pokémon games, and the ones that do often rely on
manual contribution processes.

`pokerus` is intended to provide the means for fans of the Pokémon series to
create tools for viewing and managing save files, RNG manipulation, and a great
many other things. It is our hope that it serve as a comprehensive and accurate
source for data and file format support, building on the work that many others
have done to build existing tools.

### What It Will Not Be

What belongs in `pokerus` isn't settled or even clear at this point, but a few
things are definitely not in scope:

- It is not a source of information for the franchise outside of the video
  games, nor does it contain information about the games beyond their mechanics.
  [Bulbapedia](https://bulbapedia.bulbagarden.net/) is a better format for that.
- It does not include support for ROM hacks, fan games, or any other unofficial
  versions of the games.
- It is not an engine for recreating the Pokémon games. Any functionality
  outside of querying game data and reading or writing game data structures is
  out of scope.
- It is not a repository of source data files. While it is intended that anyone
  with the needed data sources be able to run codegen, we cannot legally
  distribute those sources.
