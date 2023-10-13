#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="archlinux-xnuk"
iso_label="XNUK_ARCH_DICK"
iso_publisher="Xnuk Shuman"
iso_application="Xnuk Shuman Dick"
iso_version="nightly"
install_dir="arch"
buildmodes=('iso')
bootmodes=('uefi-x64.grub.esp' 'uefi-x64.grub.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/root"]="0:0:750"
  ["/usr/bin/lwm"]="0:0:755"
  ["/usr/bin/vtoydump"]="0:0:755"
)
