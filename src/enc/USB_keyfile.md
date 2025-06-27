# USB Stick Keyfile

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

This allows you to use a USB stick for your keyfile, with a backup in case you
want or need it. There is a setting `fallbackToPassword` that protects you in
case something fails with the USB key.

First, I'll show how to set up a dedicated USB stick for a keyfile. (i.e., one
that is only used for this). After that I will show the process of adding the
keyfile to a USB stick with existing data on it that you don't want to lose.

**Generate the keyfile**

```bash
sudo dd if=/dev/urandom of=/root/usb-luks.key bs=4096 count=1
```

This is for a dedicated USB stick that we will wipe first then add the key.

```bash
sudo cryptsetup luksAddKey /dev/disk/by-partlabel/luks /root/usb-luks.key
```

- `/dev/disk/by-partlabel/luks` refers to your encrypted partition by its
  partition label, which is stable and less likely to change than
  `/dev/nvme0n1p2`

- `/root/usb-luks.key` is the keyfile we generated.

- You'll be prompted to enter your existing LUKS passphrase to authorize adding
  the new key.

- Now our LUKS volume will accept both our existing passphrase and the new
  keyfile (from the USB stick) for unlocking.

1.  **Clear Data on USB stick and replace with 0's**

```bash
NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
sda           8:0    1   239M  0 disk
sdb           8:16   1   1.4M  0 disk  /run/media/jr/7CD1-149A # Example USB mount
zram0       253:0    0   7.5G  0 disk  [SWAP]
nvme0n1     259:0    0 476.9G  0 disk
├─nvme0n1p1 259:1    0   512M  0 part  /boot
└─nvme0n1p2 259:2    0 476.4G  0 part
  └─cryptroot 254:0  0 476.4G  0 crypt /persist  # Main Btrfs mount
                                               # (other subvolumes are within /persist and bind-mounted by impermanence)
# unplug the device and run lsblk again so your sure
```

2. Before wiping you must unmount any mounted partitions:

```bash
sudo umount /dev/sda1
```

```bash
# Overwrite with Zeros (fast, sufficient for most uses)
sudo dd if=/dev/zero of=/dev/sda bs=4M status=progress
# Or overwrite with Random Data (More Secure, Slower)
sudo dd if=/dev/urandom of=/dev/sda bs=4M status=progress
# Or for the most secure way run multiple passes of
sudo shred -v -n 3 /dev/sda
```

3. Create a New Partition and Format (Optional)

```bash
sudo fdisk /dev/sda
```

- Press `o` to create a new empty DOS partition table.

- Press `n` to create the new partition

- Press `w` to write the changes.

Formats as FAT32:

```bash
sudo mkfs.vfat /dev/sda1
# or as ext4
sudo mkfs.ext4 /dev/sda1
```

I chose `vfat` so I ran `sudo mkfs.vfat /dev/sda1`. In my case this changed the
device path to `/run/media/jr/7CD1-149A` so it's important to find your own UUID
with the following command:

```bash
sudo blkid /dev/sda1
/dev/sda1: SEC_TYPE="msdos" UUID="B7B4-863B" BLOCK_SIZE="512" TYPE="vfat" PARTUUID="7d1f9d7f-01"
```

- As you can see the above UUID is `"B7B4-863B"`

- Remove and re-insert the USB stick, this ensures the system recognizes the new
  partition and filesystem.

4. Copy the keyfile to your USB Stick

```bash
sudo cp /root/usb-luks.key /run/media/jr/B7B4-863B/
sync
```

5. Securely Remove the Keyfile from Your System

```bash
sudo shred --remove --zero /root/usb-luks.key
```

6. Update your NixOS Configuration

Note the output of `blkid /dev/sda1` and if you have a backup device list that
also:

The following is from the wiki edited for my setup, it was created by Tzanko
Matev:

```nix
let
  PRIMARYUSBID = "B7B4-863B";
  BACKUPUSBID = "Ventoy";
in {

  boot.initrd.kernelModules = [
    "uas"
    "usbcore"
    "usb_storage"
    "vfat"
    "nls_cp437"
    "nls_iso8859_1"
  ];

  boot.initrd.postDeviceCommands = lib.mkBefore ''
    mkdir -p /key
    sleep 2
    mount -n -t vfat -o ro $(findfs UUID=${PRIMARYUSBID}) /key || \
    mount -n -t vfat -o ro $(findfs UUID=${BACKUPUSBID}) /key || echo "No USB key found"
  '';

  boot.initrd.luks.devices.cryptroot = {
    device = "/dev/disk/by-partlabel/luks";
    keyFile = "/key/usb-luks.key";
    fallbackToPassword = true;
    allowDiscards = true;
    preLVM = false; # Crucial!
  };
}
```

If you have issues or just want to remove the key take note of the path used to
add it so you don't have to enter the whole key:

```bash
sudo cryptsetup luksRemoveKey /dev/disk/by-partlabel/luks --key-file /root/usb-luks.key
```

## Instructions for Using a USB Stick with Existing Data

1. Generate the Keyfile

```bash
sudo dd if=/dev/urandom of=/root/usb-luks.key bs=4096 count=1
```

2. Add the Keyfile to your LUKS Volume

```bash
sudo cryptsetup luksAddKey /dev/disk/by-partlabel/luks /root/usb-luks.key
```

(enter your existing passphrase when prompted)

3. Copy the Keyfile to the USB Stick

- Plug in the USB Stick and note its mount point
  (e.g.,`/run/media/$USER/YourLabel`)

- Copy the keyfile:

```bash
sudo cp /root/usb-luks.key /run/media/$USER/YourLabel/
sync
```

- You run the above as 2 commands, the second being `sync`.

- You can rename it if you wish (e.g., `luks.key`)

4. Securely Delete the Local Keyfile

```bash
sudo shred --remove --zero /root/usb-luks.key
```

- You need to ensure the keyfile is accessible in the initrd. Since automounting
  (like `/run/media/...`) does not happen in `initrd`, you must manually mount
  the USB in the `initrd` using its `UUID` or label.

Find the USB Partition UUID:

```bash
lsblk -o NAME,UUID
# or
blkid /dev/sda1
```

Suppose the UUID is `B7B4-863B`

Add to your `configuration.nix`:

```nix
boot.initrd.kernelModules = [ "usb_storage" "vfat" "nls_cp437" "nls_iso8859_1" ];

boot.initrd.postDeviceCommands = lib.mkBefore ''
  mkdir -p /key
  sleep 1
  mount -n -t vfat -o ro $(findfs UUID=B7B4-863B) /key || echo "USB not found"
'';

boot.initrd.luks.devices.cryptroot = {
  device = "/dev/disk/by-partlabel/luks";
  keyFile = "/key/usb-luks.key"; # or whatever you named it
  fallbackToPassword = true;
  allowDiscards = true;
};
```
