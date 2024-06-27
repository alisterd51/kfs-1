use core::arch::asm;

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

const GDT_SIZE: usize = 7;
const GDT_ADDRESS: u32 = 0x00000800;

static mut GDT: *mut [SegmentDescriptor; GDT_SIZE] =
    GDT_ADDRESS as *mut [SegmentDescriptor; GDT_SIZE];

pub fn init_gdt() {
    unsafe {
        *GDT = [
            SegmentDescriptor::null(),                // Null segment
            SegmentDescriptor::new(0, 0xFFFFF, 0x9A), // Kernel code segment
            SegmentDescriptor::new(0, 0xFFFFF, 0x92), // Kernel data segment
            SegmentDescriptor::new(0, 0xFFFFF, 0x96), // Kernel stack segment
            SegmentDescriptor::new(0, 0xFFFFF, 0xFA), // User code segment
            SegmentDescriptor::new(0, 0xFFFFF, 0xF2), // User data segment
            SegmentDescriptor::new(0, 0xFFFFF, 0xF6), // User stack segment
        ];

        let gdtr = GdtDescriptor {
            limit: (size_of::<[SegmentDescriptor; GDT_SIZE]>() - 1) as u16,
            base: GDT_ADDRESS,
        };

        asm!("lgdt [{}]", in(reg) &gdtr, options(nostack, preserves_flags));

        // Reload segment selectors
        asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            options(nostack)
        );
        asm!("push 0x08", "lea eax, [1f]", "push eax", "retf", "1:",);
    }
}

impl SegmentDescriptor {
    const fn new(base: u32, limit: u32, access: u8) -> Self {
        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | 0xC0,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    const fn null() -> Self {
        SegmentDescriptor {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}
