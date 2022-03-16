# opal

opal is a simple, static webapp to look up the IPA phonetics of English words, or look up English words from
some IPA phonetics. Written in Yew and styled with Tailwind.

Support for other languages may be added in the future.

## Why...?

[See why here.](Why.md)

## Thanks

- The English word-to-phonetic mappings were determined using [CMUdict](https://github.com/cmusphinx/cmudict) as a base,
  converted using a script to SQLite.
- The technique for hosting a local SQLite database is from [phiresky's blog](https://phiresky.github.io/blog/2021/hosting-sqlite-databases-on-github-pages/),
  using [sql.js-httpvfs](https://github.com/phiresky/sql.js-httpvfs).
