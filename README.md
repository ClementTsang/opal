# opal

opal is a simple, static webapp to look up the IPA phonetics of English words, or look up English words from
some IPA phonetics. Written in Yew and styled with Tailwind.

Support for other languages may be added in the future.

## Why...

See answers to some questions some people will likely have [here.](Why.md)

## Development

Note that opal was developed and built with:

- Rust 1.60.0 - it'll probably work with older versions but this is what I tested on
- Uses Tailwind 3.0.18

### Installation

1. Clone the repo.
2. Run `setup.sh` to initialize all required SQLite databases. Note this will require `git`, `sqlite`, Python 3.9 or later, and `pandas` installed on your machine.
3. Install [`trunk`](https://github.com/thedodd/trunk). This is used to build things, as well as be a convenient tool for development.
4. Install some static HTTP server tool that supports byte ranges. I personally use [http-server](https://www.npmjs.com/package/http-server).
5. Install [Tailwind CSS](https://tailwindcss.com/).
6. Run `trunk build`. If you're developing and want builds upon saving, use `trunk watch` instead. This will build to `./dist`.
7. In another terminal, run the HTTP server. For `http-server`, you can `cd dist/` and `http-server ./`
8. For release builds, run `trunk build --release`. For more details on optimizations, look at [the deploy workflow.](./.github/workflows/deploy.yml)

### Deployment

1. Build with `trunk build --release`.
2. Deploy the `dist/` folder. Note that the static webpage service you deploy to **must** properly support byte ranges - otherwise, you are **very likely to face problems**!
   Trust me, you don't want to try troubleshooting why SQLite complains of a malformed database only on Firefox and Incognito Chrome, but not Chromium or Chrome.

   As of writing, GitHub Pages should work fine, and Cloudflare Pages is supposed to support byte ranges in the future. I haven't looked into other static webpage services.

## Credits

- The English (US) IPA mappings were created using [CMUdict](https://github.com/cmusphinx/cmudict) as a base
  ([license](https://github.com/cmusphinx/cmudict/blob/master/LICENSE)), converted from ARPABET to IPA following the
  mappings on [Wikipedia](https://en.wikipedia.org/wiki/ARPABET).
- The entire idea of hosting a SQLite database on a static webpage comes from [phiresky's blog](https://phiresky.github.io/blog/2021/hosting-sqlite-databases-on-github-pages/),
  using their [sql.js-httpvfs](https://github.com/phiresky/sql.js-httpvfs), which I wrote [simple bindings for](https://github.com/ClementTsang/sql.js-httpvfs-rs).
