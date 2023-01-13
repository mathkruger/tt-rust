CP() {
    mkdir -p $(dirname "$2") && cp "$1" "$2"
}

cargo build -r
CP target/release/tt-rust $HOME/.tt-rust/tt

echo ""
echo "--------------------"
echo "tt builded and copied to $HOME/.bin/tt-rust. Please add the following code to your .bashrc:"
echo ""
echo "export PATH=$HOME/.tt-rust/"
echo "--------------------"
echo ""