# Why...

Answers to some "why" questions I'm sure some people will have.

## Why did you make this?

Short answer: Why not?

Longer answer: Basically, a bunch of random reasons that all came together.

- I was a bit bored at the time and wanted a small/unimportant project that was easy to slowly work on whenever I had free time.
- I wanted to play with WASM and Yew, the latter of which I had been thinking of trying to use for a while.
- I wanted to try using Tailwind.
- I just took an intro to linguistics class at uni.

All this consolidated into working on
this weird, overly-complicated, and unnecessary project that I'm pretty happy with.

## Why "opal"?

I like making my project names silly acronyms, jokes, or puns. If you haven't seen the repo description, opal here stands
for "Oxidized Phonetics Alphabet Lookup". Why did I choose this acronym? I have no idea.

## Why did you not use a conventional backend/DB setup? Why a static page?

Ever since I read [this blog post](https://phiresky.github.io/blog/2021/hosting-sqlite-databases-on-github-pages/) about
hosting an SQLite database on Github Pages, and [sql.js-httpvfs](https://github.com/phiresky/sql.js-httpvfs), I've
wanted to use it for some project - and a simple, read-only web application that I would prefer to not have to manage
or pay for upkeep sounds like a perfect use case for this concept.

## Why Rust for a frontend? Why not just use JS or some derivative?

A few reasons I decided on Rust for this project:

- I program primarily in Rust in my free time, if you haven't seen my other repos, so I guess it's not surprising I
  thought about trying to use it here, even in an area its not particularly known for.
- I've always wanted to take a crack at trying to build a frontend with [Yew](https://yew.rs/), and at the time of
  writing, it looks like the most mature framework (though [dioxus](https://dioxuslabs.com/) looks interesting as well).
- Exploring WASM-related stuff in general is super interesting, and while I've played with it in some capacity with things
  like [Polify](https://github.com/ClementTsang/polify), making a frontend (technically my second) that relies on WASM
  and messing with [JS-to-Rust bindings](https://github.com/ClementTsang/sql.js-httpvfs-rs), was a fun learning experience.

Of course, doing it like this added way more difficulty than I needed - primarily around having to learn bindings, and
the Rust web frontend ecosystem is passable at best.

If this was a more serious project, I would have definitely gone with just using JS (or more likely, Typescript), which
would be far easier to work with - there is/was stuff that I did during this project that would have definitely been
easier in JS-land, especially with a framework like Svelte/Vue/React. However, since this project was more for exploration and fun, I'm fine with going with unconventional
choices for the heck of it, and I learned a lot from doing it like this.

## Why Tailwind?

Because I can't style things for my life and Tailwind CSS seemed like an easy one to try to work with in conjunction with Yew. For the most part, it's
been alright in my experience, though getting it to play nicely with Yew and Trunk was a bit tricky at first. Feel free to refer to the repo if you're trying to figure that out in your own projects.
