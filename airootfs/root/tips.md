Installation guide: file:///usr/share/doc/arch-wiki/html/en/Installation_guide.html

## Encryption
file:///usr/share/doc/arch-wiki/html/en/Dm-crypt/Encrypting_an_entire_system.html#Plain_dm-crypt

Encryption in existing system:
file:///usr/share/doc/arch-wiki/html/en/Dm-crypt/Device_encryption#Encrypt_an_existing_unencrypted_file_system
file:///usr/share/doc/arch-wiki/html/en/Trusted_Platform_Module#systemd-cryptenroll

## ip
Usually it's automatically set. Try ping first.
$ ip link enp31s0 up

## iwd
$ iwctl
    device list
    station wlan0 scan
    station wlan0 get-networks
    station wlan0 connect [SSID]
    station wlan0 connect-hidden [SSID]

## Favourite mount option
mount -o compress=zstd,ssd,subvol=@root
btrfs subvolume create @root
btrfs subvolume create @home

Do not subvolume /etc, since /etc/fstab is there.

## After mount
pacstrap -K /mnt base base-devel linux amd-ucode intel-ucode kakoune iwd fish dash

genfstab -U > /mnt/etc/fstab
cp {,/mnt}/etc/pacman.conf
cp {,/mnt}/etc/pacman.d/seoul-mirrorlist
