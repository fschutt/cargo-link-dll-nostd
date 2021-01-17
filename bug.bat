cargo clean

cd %~dp0\azul-dll
cargo build --release
cd ..

rem BUGFIX for issue / workaround:
rem copy "%~dp0\target\release\azul.dll.lib" "%~dp0\target\release\azul.lib"

cargo build --release --example bug

pause > nul