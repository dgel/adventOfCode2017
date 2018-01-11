struct GeneratorA {
    value: u64,
}

impl GeneratorA {
    fn new() -> GeneratorA {
        GeneratorA { value: 591 }
    }
}

impl Iterator for GeneratorA {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * 16_807) % 2_147_483_647;
            if self.value & 0b11 == 0 {
                break;
            }
        }
        Some(self.value)
    }
}

struct GeneratorB {
    value: u64,
}

impl GeneratorB {
    fn new() -> GeneratorB {
        GeneratorB { value: 393 }
    }
}

impl Iterator for GeneratorB {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * 48_271) % 2_147_483_647;
            if self.value & 0b111 == 0 {
                break;
            }
        }
        Some(self.value)
    }
}

fn main() {
    let count: u32 = GeneratorA::new()
        .zip(GeneratorB::new())
        .map(|(a, b)| if (a & 0xFFFF) == (b & 0xFFFF) { 1 } else { 0 })
        .take(5_000_000)
        .sum();

    println!("{}", count);
}
