//! Module `ks` contains extras for yelling at the ks (user).

use crate::actual_help;

/// Compares against a list of known queries, and prints extras for them.
pub fn maybe_print_extras(query: &str) {
    let maybe_message = match query {
        "cover" => Some(EXTRA_COVER),
        "extras" => Some(EXTRA_EXTRAS),
        "ggrks" => Some(EXTRA_GGRKS),
        "help" => Some(actual_help::HELP_MESSAGE),
        "song" => Some(EXTRA_SONG),
        "zen" => Some(EXTRA_ZEN),
        _ => None,
    };

    if let Some(message) = maybe_message {
        println!("{}", message);
    }
}

// Why didn't I make a module for these? Why don't you go module some bitches?
// (The actual reason is to make it easier to respect an 80-column limit.)
const EXTRA_COVER: &str = "\
https://www.youtube.com/watch?v=eOY515ypArk
";

const EXTRA_EXTRAS: &str = "\
Didn't I just fucking tell you not to ggrks for \"extras?\"
Were you just not fucking listening or something?
Or do you genuinely just have no respect for my kind, thoughtful suggestions?
Either way, you're a ks.

Fine. You want to throw away all the fun of reading the source code?
Well don't let me stop you. Here's the fucking extras list, bitch:
    cover
    extras (you're reading it right now, dumbass)
    ggrks
    help
    song
    zen
";

const EXTRA_GGRKS: &str = "\
\"ggrks\" is Japanese internet slang for ググれカス (gugure kasu).

Want to know what THAT means? ggrks. In fact, I already did it for you.
Check your browser, ks.

Now that you actually UNDERSTAND the insults I've been hurling at you,
go listen to the song, ks:
https://www.nicovideo.jp/watch/sm6085510
";

const EXTRA_SONG: &str = "\
https://www.nicovideo.jp/watch/sm6085510
";

const EXTRA_ZEN: &str = "\
~Please do not ask me why~
~All of you wants to know me, I think~

n00bs are supposed to lurk before posting, dumbass
if you're gonna get all friendly, do it on the VIP board
this isn't your personal diary, dipshit
mods wyd ban this asshole already

don't ask me, fucking google it, bitch
google it on yahoo, bitch
google it on rakuten, bitch

~Please do not ask me why~
~All of you wants to know me, I think~
~~~You search it with this Google~~~
";
