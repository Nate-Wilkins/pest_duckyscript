function build() {
  cargo build --all-targets --release

  # Generate icon.
  # TODO: Create an icon.
  # /usr/bin/convert -background none -density 1200 -resize 128x128 ./icon.svg ./target/release/icon_128.png
}

function start() {
  cargo run
}

function test() {
  cargo test
}

function publish() {
  cargo login
  cargo publish
}

