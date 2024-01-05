#!/bin/bash

# Get a list of block devices
devices=$(lsblk -o NAME -n -l)

# Generate /etc/fstab entries
for device in $devices; do
    uuid=$(blkid -s UUID -o value /dev/"$device")
    type=$(blkid -s TYPE -o value /dev/"$device")

    # Skip entries without UUID or type
    if [ -n "$uuid" ] && [ -n "$type" ]; then
        echo "UUID=$uuid /mnt/$device $type defaults 0 2"
    fi
done
