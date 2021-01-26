# Upload iso to cd-r / dvd

sudo dd bs=4M if=Downloads/ubuntu-19.04-desktop-amd64.iso of=/dev/sdb conv=fdatasync

# sudo: You need to be a superuser to issue dd commands. You will be prompted for your password.
# dd: The name of the command we’re using.
# bs=4M: The -bs (blocksize) option defines the size of each chunk that is read from the input file and wrote to the output device. 4 MB is a good choice because it gives decent throughput and it is an exact multiple of 4 KB, which is the blocksize of the ext4 filesystem. This gives an efficient read and write rate.
# if=Downloads/ubuntu-19.04-desktop-amd64.iso: The -if (input file) option requires the path and name of the Linux ISO image you are using as the input file.
# of=/dev/sdb: The -of (output file) is the critical parameter. This must be provided with the device that represents your USB drive. This is the value we identified by using the lsblk command previously. in our example it is sdb, so we are using /dev/sdb. Your USB drive might have a different identifier. Make sure you provide the correct identifier.
# conv=fdatasync: The conv parameter dictates how dd converts the input file as it is written to the output device. dd uses kernel disk caching when it writes to the USB drive. The fdatasync modifier ensure the write buffers are flushed correctly and completely before the creation process is flagged as having finished.



# Type in the ISO creation command. Type in mkisofs -o destination-filename.iso /home/username/folder-name, making sure to replace "destination-filename" with whatever you want to name the ISO file and "folder-name" with the name of the folder in which your ISO's files are stored.
# For example: to create an ISO file named "blueberry" from files in a folder called "pie", you would type in mkisofs -o blueberry.iso /home/username/pie.
# File names and folder names are case-sensitive, so make sure you capitalize anything that needs to be capitalized.
# To create a multiple-word name, place underscores between words (e.g., "blueberry pie" becomes "blueberry_pie").


mkisofs -o blueberry.iso /home/username/pie
