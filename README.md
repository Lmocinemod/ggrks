# ggrks

**CONTENT WARNING:** This program and README file both contain profanity,
which is intended to be comical in nature. If you find such use of profanity
disturbing and/or unpleasant, you are encouraged not to use this program or
continue reading this README.

-   [What is ggrks?](#what-is-ggrks)
-   [What does "ggrks" mean?](#what-does-ggrks-mean)
-   [Why did you make this, anyway?](#why-did-you-make-this-anyway)
-   [Can I use ggrks as a library?](#can-i-use-ggrks-as-a-library)
-   [How do I install ggrks?](#how-do-i-install-ggrks)
-   [How do I use ggrks?](#how-do-i-use-ggrks)
-   [Can I contribute to ggrks?](#can-i-contribute-to-ggrks)

## What is ggrks?

Ggrks is a CLI program for performing Google searches that I made in a day as
a joke.

## What does "ggrks" mean?

"Ggrks" is an acronym used in Japanese internet slang. It expands to
"[ググれカス](https://en.wiktionary.org/wiki/ググれカス)" _(gugure kasu)_, which
I like to translate as "fucking Google it, bitch."

The English equivalent is "[jfgi](https://en.wiktionary.org/wiki/JFGI)."

## Why did you make this, anyway?

Because I was inspired by a
[song of the same name](https://www.nicovideo.jp/watch/sm6085510), and thought
it'd be funny to have a legitimate reason to type `ggrks` into a terminal.

## Can I use ggrks as a library?

Assuming I configured everything correctly, yes. Just add `ggrks` to your
dependencies in `Cargo.toml`, then use it like this:

```rust
use ggrks::ggrks;
// ...
ggrks("<query>");
```

Don't worry, calling `ggrks::ggrks()` will never involve any profanity. (Well,
apart from the function's name, that is.)

## How do I install ggrks?

```
cargo install ggrks
```

I can't figure out how to get cross-building to work, so I'm not providing any
prebuilt binaries. You're on your own, bitch. 😜

## How do I use ggrks?

Running `ggrks` without any arguments will display a help message.
[読めカス。](https://www.deepl.com/en/translator#ja/en/読めカス。)

## Can I contribute to ggrks?

Sure, I guess. You're probably better off spending your time doing other
things, though. (Also, I'm not very active on GitHub, so if you need my
attention, please ping me on Twitter.)

As far as things to improve upon... I guess we could use some more extras?
(See `src/ks.rs`)
