$Command = "esptool.exe"
if (Test-Path -Path ".\esptool-v3.1-win64\esptool-v3.1-win64\esptool.exe" -PathType Leaf) {
  $Command = ".\esptool-v3.1-win64\esptool-v3.1-win64\esptool.exe"
}
$OldPreference = $ErrorActionPreference
$ErrorActionPreference = 'stop'
try {if(Get-Command $Command){"$Command exists"}}
Catch {
  "$Command not found. Downloading"
  Invoke-WebRequest https://github.com/espressif/esptool/releases/download/v3.1/esptool-v3.1-win64.zip -OutFile esptool-v3.1-win64.zip
  Expand-Archive -Path esptool-v3.1-win64.zip
  $Command = ".\esptool-v3.1-win64\esptool-v3.1-win64\esptool.exe"
  $ErrorActionPreference=$OldPreference
}

&$Command  -b 460800 --before default_reset --after hard_reset --chip esp32  write_flash --flash_mode dio --flash_size detect --flash_freq 40m 0x1000 bootloader.bin 0x8000 partition-table.bin 0x10000 esp32-hello-rust.bin
