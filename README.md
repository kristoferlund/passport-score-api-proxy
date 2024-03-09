# passport-score-proxy

Runs a Cloudflare Worker that proxies requests to the Passport Score API. The worker is written in Rust, compiled to WebAssembly, and deployed to Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Development

### 1. Set environment variables

```bash
cp .dev.vars.template .dev.vars
```

Edit `.dev.vars` with your Gitcoin API key and your passport scorer id.

### 2. Run

```bash
npm i 
npm run dev
```

## Production


### 1. Deploy

```bash
npm run deploy
```

### 2. Set environment variables

```bash
npx wrangler secret put PASSPORT_API_KEY
npx wrangler secret put PASSPORT_SCORER_ID
```

## Wrangler

Wrangler is used to develop, deploy, and configure your Worker via CLI.

Further documentation for Wrangler can be found [here](https://developers.cloudflare.com/workers/tooling/wrangler).

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers) is meant to be executed as compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`workers-rs`](https://github.com/cloudflare/workers-rs) project README.
