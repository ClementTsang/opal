<div align="center">
  <h1>opal</h1>
  
  <p>
    opal (Oxidized Phonetics Alphabet Lookup) is a static WASM-based webapp to look up the IPA phonetics of English words or vice versa.
  </p>
</div>

https://user-images.githubusercontent.com/34804052/175511088-68dc2f3a-f793-4446-99f3-9997838ca4fc.mp4

You can find a version hosted via GitHub Pages [here](https://clementtsang.github.io/opal/).

## Why...

See answers to some questions some people will likely have [here.](Why.md)

## Development

Note that (as of 2023-01-31) opal was developed and tested with:

- Rust 1.67.0 (it'll probably work fine with some older versions but this is what I tested on)
- Yew 0.19.3
- Tailwind 3.2.4

Additionally, opal relies on [`trunk`](https://github.com/thedodd/trunk) (0.15.0 as of writing) for building and
general development, handling the build to WASM and running the Tailwind hook for CSS.

### Installation

1. Clone the repo.
2. Run `setup.sh` to initialize all required SQLite databases. Note this will require `git`, `sqlite`, Python 3.9 or
   later, and `pandas` installed on your machine.
3. Install [`trunk`](https://github.com/thedodd/trunk). This is used to build WASM and Tailwind.
4. Install some static HTTP server tool that supports byte ranges. I personally used [`http-server`](https://www.npmjs.com/package/http-server).
5. Install [Tailwind CSS](https://tailwindcss.com/).
6. Run `trunk build`. This will build to `./dist`. If you're working on opal and want automatic builds when saving,
   use `trunk watch` instead.
7. In another terminal, run the HTTP server. For `http-server`, you can run `cd dist/` and `http-server ./` to do so.
8. For release builds, run `trunk build --release`. For more details on optimizing the generated WASM code, look at
   [the deploy workflow](./.github/workflows/deploy.yml) and [post-compilation optimization script](./scripts/optimize.py)
   for how to optimize using minify and wasm-opt.

### Deployment

1. Build with `trunk build --release`.
2. Deploy the `dist/` folder. Note that the static webpage service you deploy to **must** properly support the
   `Accept-Ranges=bytes` header - otherwise, you are **very likely to face problems**! You might face weird
   issues like SQLite complaining of a malformed database on _only_ Firefox and incognito Chrome, or something
   along those lines. Basically, weird things may happen if it isn't supported!

   As of writing, GitHub Pages should work fine, and Cloudflare Pages is supposed to support it in the future. I
   haven't looked into other static webpage services.

## Support

Currently, only English (and a specific subset at that) is supported. However, support for more sources or other
languages isn't too difficult to add, and may be added in the future. PRs are certainly welcome.

## Credits

opal builds on a lot of existing work, which I have to thank:

- The English (US) IPA mappings were created using [CMUdict](https://github.com/cmusphinx/cmudict) as a base
  (see [original license](https://github.com/cmusphinx/cmudict/blob/master/LICENSE)), converted from ARPABET to IPA following the
  mappings on [Wikipedia](https://en.wikipedia.org/wiki/ARPABET).
- The entire idea of hosting a SQLite database on a static webpage comes from [phiresky's blog](https://phiresky.github.io/blog/2021/hosting-sqlite-databases-on-github-pages/),
  using their [sql.js-httpvfs](https://github.com/phiresky/sql.js-httpvfs), which I wrote [simple bindings for](https://github.com/ClementTsang/sql.js-httpvfs-rs).
- Written using [Yew](https://yew.rs/).
- Styling from [Tailwind CSS](https://tailwindcss.com/).
- Fonts used are from [Open Sans](https://github.com/googlefonts/opensans) and [Source Code Pro](https://github.com/adobe-fonts/source-code-pro).
