echo "Prepping..."
mkdir yatag-game
echo "Building..."
cargo build --release
echo "Copying..."
cp ./target/release/yatag ./yatag-game/yatag
echo "Creating..."
pkgbuild --identifier io.interfiber.yatag --root yatag-game/ --install-location /usr/local/bin Yet\ Another\ Text\ Adventure\ Game.pkg
echo "Cleaning..."
rm -rf yatag-game
echo "Done."