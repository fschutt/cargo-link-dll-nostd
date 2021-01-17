cargo clean

cd %~dp0\azul-dll
cargo +nightly build --release
cd ..

rem BUGFIX for issue / workaround:
copy "%~dp0\target\release\azul.dll.lib" "%~dp0\target\release\azul.lib"

cargo +nightly build --release --example bug

pause > nul