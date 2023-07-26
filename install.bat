set BIN_PATH=%USERPROFILE%\.local\bin

if not exist "%BIN_PATH%" mkdir "%BIN_PATH%"

cargo clean
cargo build --release

copy ".\target\release\savedfile.exe" "%BIN_PATH%"
echo Installed to "%BIN_PATH%"

setx path "%PATH%;%BIN_PATH%"
