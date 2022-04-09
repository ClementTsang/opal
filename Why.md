# Why...

Answers to some "why" questions I'm sure some people will have.

## Why did you make this?

Short answer: Why not?

Longer answer: I was a bit bored and wanted a small project to work on, I wanted to play with WASM and Yew,
I wanted to try using Tailwind, and I just took an intro to linguistics class. This all consolidated into working on
this overly-complicated-and-unnecessary project.

## Why "opal"?

I like making my project names silly acronyms, jokes, or puns.

## Why did you not use a conventional backend/DB setup? Why a static page?

Ever since I read [this blog post](https://phiresky.github.io/blog/2021/hosting-sqlite-databases-on-github-pages/) about
hosting an SQLite database on Github Pages, and [sql.js-httpvfs](https://github.com/phiresky/sql.js-httpvfs), I've
wanted to use it for some project - and a simple, read-only web application that I would prefer to not have to manage
or pay for upkeep sounds like a perfect use case for this concept.

## Why Rust? Why not just use JS or some derivative like a sane human being?

A few reasons I decided on Rust for this project:

- I program primarily in Rust in my free time, if you haven't seen my other repos, so I guess it's not surprising I
  thought about using it here.
- I've always wanted to take a crack at trying to build a frontend with [Yew](https://yew.rs/), and at the time of
  writing, it looks like the most mature framework (though [dioxus](https://dioxuslabs.com/) looks interesting as well).
- Exploring WASM-related stuff in general is super interesting, and while I've played with it in some capacity with things
  like [Polify](https://github.com/ClementTsang/polify), making a frontend (technically my second) that relies on WASM
  and messing with [JS-to-Rust bindings](https://github.com/ClementTsang/sql.js-httpvfs-rs), was a fun learning experience.

Of course, doing it like this added way more difficulty than I needed - primarily around having to learn bindings, and
the Rust web frontend ecosystem is immature at best.

If this was a more serious project, I would have definitely gone with just using JS (or more likely, Typescript), which
would be far easier to work with. However, since this project was more for exploration and fun, I'm happy with my
choice, and I learned a lot from it.

## Why Tailwind?

Because I can't style things for my life and Tailwind CSS seemed like an easy one to work with. For the most part, it's
been alright in my experience, though getting it to play nicely with Yew and Trunk was a bit tricky at first.
