// Experiment 1: Where does my data live?
// Run with:  cargo run
//
// Everything your program touches has an ADDRESS — a number naming a byte
// in memory. We'll print real addresses so the stack/heap split becomes visible.

fn main() {
    println!("=== 1. Stack variables ===");
    // Local variables live on the STACK. Their addresses are assigned by just
    // moving a pointer (the "stack pointer") as the function runs.
    let a: u8 = 1;
    let b: u8 = 2;
    let c: u64 = 3;
    // {:p} prints a pointer/address in hex. &a is "the address of a".
    println!("a (u8)  @ {:p}", &a);
    println!("b (u8)  @ {:p}", &b);
    println!("c (u64) @ {:p}", &c);
    println!("-> notice they're clustered together in the same region.\n");

    println!("=== 2. Heap allocation (Box) ===");
    // Box<T> puts the VALUE on the heap. The Box itself (a pointer) lives on
    // the stack. So there are TWO addresses: where the pointer sits, and where
    // it points.
    let boxed: Box<u64> = Box::new(42);
    println!("the Box pointer itself sits @ {:p}  (on the stack)", &boxed);
    println!("...and it points to         @ {:p}  (on the heap)", &*boxed);
    println!("-> heap addresses are typically very different from stack ones.\n");

    println!("=== 3. How big is each type? (bytes) ===");
    println!("u8   = {} byte", std::mem::size_of::<u8>());
    println!("u64  = {} bytes", std::mem::size_of::<u64>());
    println!("&u64 = {} bytes  (a reference is just an address!)", std::mem::size_of::<&u64>());
    println!("Box<u64> = {} bytes  (also just an address)\n", std::mem::size_of::<Box<u64>>());

    println!("=== 4. Struct layout: the compiler may REORDER fields ===");
    // Declared order: x, y, z. But Rust is free to rearrange fields to pack
    // them tightly. Watch where each one actually lands.
    #[derive(Debug)]
    struct Point {
        x: u8,
        y: u32,
        z: u8,
    }
    let p = Point { x: 1, y: 2, z: 3 };
    let base = &p as *const Point as usize;
    println!("struct Point total size = {} bytes", std::mem::size_of::<Point>());
    println!("  x @ offset {}", (&p.x as *const u8 as usize) - base);
    println!("  y @ offset {}", (&p.y as *const u32 as usize) - base);
    println!("  z @ offset {}", (&p.z as *const u8 as usize) - base);
    println!("-> Rust put the 4-byte y FIRST (so it's 4-byte aligned), then packed");
    println!("   the two 1-byte fields after it. Declared order != memory order!\n");

    println!("=== 5. Same fields, but #[repr(C)]: declared order is forced ===");
    // #[repr(C)] tells the compiler to use C's layout rules: fields stay in the
    // order you wrote them. This is required for talking to C code / hardware,
    // but it can waste space on PADDING to keep alignment.
    #[repr(C)]
    struct PointC {
        x: u8,
        y: u32,
        z: u8,
    }
    let q = PointC { x: 1, y: 2, z: 3 };
    let base = &q as *const PointC as usize;
    println!("struct PointC total size = {} bytes", std::mem::size_of::<PointC>());
    println!("  x @ offset {}", (&q.x as *const u8 as usize) - base);
    println!("  y @ offset {}", (&q.y as *const u32 as usize) - base);
    println!("  z @ offset {}", (&q.z as *const u8 as usize) - base);
    println!("-> x at 0, then 3 bytes of PADDING so y lands at offset 4, then z.");
    println!("   Same data, but 12 bytes instead of 8 — that's the cost of fixed order.");
}
