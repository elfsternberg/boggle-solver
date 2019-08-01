![Language: Rust](https://img.shields.io/badge/language-Rust-green.svg)
![Topic: Interview](https://img.shields.io/badge/topic-Interview_Question-red.svg)
![Topic: Tries](https://img.shields.io/badge/topic-Tries-red.svg)
![Library Status: Complete](https://img.shields.io/badge/status-Library_Complete-green.svg)
![Command Line Status: In Progress](https://img.shields.io/badge/status-CLI_In_Progress-yellow.svg)

# Boggle Solving in Rust

Boggle™ is a popular tabletop game in which, given a square 4⨯4 (or now,
5⨯5) grid of letters with a probabilistic distribution chosen by the
manufacturer, the players must try to find as many valid words (as
defined by the Scrabble!™ dictionary) as possible within three minutes.

It's my family's favorite game.  Or at the very least, the easiest one
to pull out without a lot of set-up and ceremony.  It was also the
subject of a recent whiteboarding exercise I did for an interview.  I
believe I did okay in the interview, but it wasn't until I got home that
it hit me: dammit, the correct data structure for finding words is a
[trie](https://en.wikipedia.org/wiki/Trie)!

After quickly reminding myself how tries work, I spent a couple of hours
knocking this together.  It's probably not the greatest boggle solver in
the world; it doesn't do anything fancy, it's just a brute force
scan-the-board engine. It does terminate early if it generate a prefix
for which no words exist in the sample dictionary.  The trie is
implemented more or less the way Wikipedia explains it, and its ability
to distinguish between a whole word and a prefix is nicely clever, I
think.

## The Binary

A standalone binary is generated during the build.  Basic usage is:

`boggle [-d <path to dictionary>] <board_file>`

The program defaults to /usr/share/dict/words if no dictionary is
specified. 

The format for a board file is straightforward.  It's a text file,
with the letters per row, like so:

'arni
wier
oaer
hrpd
'

The parser can handle any amount of whitespace between the letters, and
will ignore blank rows after the data.  Those were common sources of
crashiness.  The parser will complain if the rows are not all of the
same length.

## Caveats:

The expected dictionary is the one found in /usr/share/dict/words on GNU
installations, known as the "GNU common words dictionary."  Your tests
may not pass if you use a bigger dictionary or if that path is
non-existent.  Sorry about that.

## Useful links:

- [The North American Scrabble™ word list](https://www.wordgamedictionary.com/twl06/download/twl06.txt)
- [The European Scrabble™ list of English words](https://www.wordgamedictionary.com/sowpods/download/sowpods.txt)

The file "twl06.txt.xz" included in the project root is an unofficial
copy of the North American Scrabble™ word list.

## ToDo

The binary is <s>ridiculously</s> now slightly less primitive.  Here's
the love it still needs:

- The parser should forbid non-alphabet and mixed-alphabet inputs
- The parser should take STDIN as an option

I would be tempted to do a Haskell version, but there's a *lot* of
mutation in this.  I need to think harder, and practice more Haskell,
before I attempt it.

## LICENSE 

- Boggle™ is a trademark of Parker Brothers, Inc.
- Scrabble™ is a trademark of Hasbro, Inc.

This Boggle™ solver is Copyright [Elf
M. Sternberg](https://elfsternberg.com) (c) 2019, and licensed with the
Mozilla Public License vers. 2.0.  A copy of the license file is
included in the root folder.

