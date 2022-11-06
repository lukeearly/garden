#!/usr/bin/env sh

mkdir -p sysroot/boot/grub
cp target/x86_64-kernel/release/garden sysroot/boot
cp grub.cfg sysroot/boot/grub

grub-mkrescue -o garden.iso sysroot
