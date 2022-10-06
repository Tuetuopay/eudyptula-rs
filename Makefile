KDIR ?= /lib/modules/`uname -r`/build

TASKS = $(wildcard task*/task*.rs)

all: $(TASKS:%.rs=%.ko)

%.ko: %.rs
	$(MAKE) -C $(KDIR) M=$$PWD/$(shell dirname $*)

clean:
	$(RM) */Module.symvers */modules.order
	$(RM) */*.mod */*.mod.c */*.mod.o */*.o */*.ko

rust-project.json:
	$(MAKE) -C $(KDIR) rust-analyzer

.PHONY: clean
