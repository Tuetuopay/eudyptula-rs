KDIR ?= /lib/modules/`uname -r`/build
ARCH ?= x86_64
EXTRAVERSION ?= -eudyptula-rs

ARCH_DEBIAN = $(ARCH:x86_64=amd64)

TASKS = $(wildcard task*/*.rs)

all: $(TASKS:%.rs=%.ko)

%.ko: %.rs
	$(MAKE) -C $(KDIR) EXTRAVERSION=$(EXTRAVERSION) M=$$PWD/$(shell dirname $*)

clean:
	$(RM) */Module.symvers */modules.order
	$(RM) */*.mod */*.mod.c */*.mod.o */*.o */*.ko

rust-project.json:
	$(MAKE) -C $(KDIR) EXTRAVERSION=$(EXTRAVERSION) rust-analyzer

debian-11-nocloud-$(ARCH_DEBIAN).qcow2:
	wget https://cloud.debian.org/images/cloud/bullseye/latest/$@

$(KDIR)/arch/$(ARCH)/boot/bzImage:
	$(MAKE) -C $(KDIR) EXTRAVERSION=$(EXTRAVERSION)

# Mount the P9 share by appending the following to /etc/fstab:
# eudyptula-rs /root/eudyptula-rs 9p trans=virtio,ro,version=9p2000.L 0 0
vm: debian-11-nocloud-$(ARCH_DEBIAN).qcow2 $(KDIR)/arch/$(ARCH)/boot/bzImage
	qemu-system-$(ARCH) \
		-machine q35,accel=kvm --nographic -cpu host -m 512 -smp 2 \
		-kernel $(KDIR)/arch/$(ARCH)/boot/bzImage \
		-append "console=ttyS0 root=/dev/vda1" \
		-drive file=debian-11-nocloud-$(ARCH_DEBIAN).qcow2,if=virtio \
		-device virtio-net,netdev=net0 \
		-netdev user,id=net0,hostfwd=tcp:127.0.0.1:2222-:22 \
		-virtfs local,path=$$PWD,mount_tag=eudyptula-rs,security_model=mapped-xattr \
		-usb -device usb-kbd,bus=usb-bus.0

.PHONY: clean vm
