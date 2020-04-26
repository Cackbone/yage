# The Yage decoder

This is a lib which aimed to give a convinent way to add instruction with a simple synthax

# macro synthax
`#[bind(opcodes=(0x80, 0x01, 0x01), args=(RegisterA, nn))]`


# Usage
Here is a example of usage:
```Rust
#[derive(YageInstructions)]
enum instructions {
	#[bind(opcodes=(0x00), args=())]
	NOP,

    #[bind(opcodes=(0x81, 0x01, 0x01), args=(n, n)]
    #[bind(opcodes=(0x80, 0x01, 0x02), args=(RegisterB, nn)]
    JP(Register, u16)
}
```

