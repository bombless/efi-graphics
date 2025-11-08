cargo build --release
mkdir -p target/x86_64-unknown-uefi/release/EFI/BOOT
cp  target/x86_64-unknown-uefi/release/uefi-test.efi target/x86_64-unknown-uefi/release/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 -bios /usr/share/edk2/x64/OVMF.4m.fd -drive format=raw,file=fat:rw:target/x86_64-unknown-uefi/release
