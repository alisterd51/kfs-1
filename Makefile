TRIPLE	:= i686-elf
BUILD	:= debug

# utlis
AS		:= ${TRIPLE}-as
LD		:= ${TRIPLE}-ld
OBJDUMP	:= ${TRIPLE}-objdump
CARGO	:= cargo
QEMU	:= qemu-system-x86_64

ARCH	:= x86

LINKSCRIPT	:= arch/$(ARCH)/link.ld
BIN			:= gratos.${ARCH}.bin
ISO			:= gratos.${ARCH}.iso
GRUBCFG		:= grub/grub.cfg

# flags
ASFLAGS				:=
LDFLAGS				:= --script ${LINKSCRIPT}
CARGOFLAGS.debug	:=
CARGOFLAGS.release	:= --release

BUILDDIR	:= build/${BUILD}
OBJDIR		:= ${BUILDDIR}/obj/${ARCH}
ISODIR		:= ${BUILDDIR}/isodir

all: iso

bin: ${BUILDDIR}/${BIN}

iso: ${BUILDDIR}/${ISO}

${BUILDDIR}:
	mkdir -p $@

${OBJDIR}:
	mkdir -p $@

${BUILDDIR}/${BIN}: ${OBJDIR}
	${CARGO} build ${CARGOFLAGS.${BUILD}}
	${AS} arch/${ARCH}/start.s -o ${OBJDIR}/start.o
	${LD} -o ${BUILDDIR}/${BIN} -T arch/${ARCH}/link.ld ${OBJDIR}/start.o target/${BUILD}/libgratos.a
	grub-file --is-${ARCH}-multiboot ${BUILDDIR}/${BIN}

${BUILDDIR}/${ISO}: ${BUILDDIR}/${BIN}
	mkdir -p ${ISODIR}/boot/grub
	cp ${BUILDDIR}/${BIN} ${ISODIR}/boot/${BIN}
	cp ${GRUBCFG} ${ISODIR}/boot/grub/grub.cfg
	grub-mkrescue --compress=xz -o ${BUILDDIR}/${ISO} ${ISODIR}

clean:
	${CARGO} clean
	rm -rf build

run: iso
	${QEMU} -cdrom ${BUILDDIR}/${ISO}

.PHONY: all bin iso clean run ${BUILDDIR}/${BIN}
