# polka.run

webgui for polkavm disassembler

## Development

```bash
rustup target add wasm32-unknown-unknown
cargo install --force cargo-make trunk
cd app && npm install
popd
cargo make dev
cargo make build
```

## Technologies
- [polkavm](https://github.com/koute/polkavm)
- [leptos](https://github.com/leptos-rs/leptos)
- [leptos-query](https://github.com/nicoburniske/leptos_query/tree/main/examples)
- [leptos-use](https://github.com/Synphonyte/leptos-use/tree/main/examples)
- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [trunk](https://github.com/thedodd/trunk)

## Screenshots
![polkarun-v04](https://github.com/rotkonetworks/polka.run/blob/master/app/public/images/polkarun-v04.png?raw=true)
![polkarun-v02](https://github.com/rotkonetworks/polka.run/assets/15621959/8234fc88-acae-4999-8c68-5a99e0b3cc58)
