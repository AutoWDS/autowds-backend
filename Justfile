## generate model
gen-model:
    sea-orm-cli generate entity --with-serde both --output-dir src/model/_entities

## build release binary
release:
    cargo build --release