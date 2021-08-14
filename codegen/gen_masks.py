import sys


def apply_pattern(pattern, mask, bits):
    start, stop, value = map(int, pattern.split(","))
    bit_count = stop - start + 1
    shift = 31 - stop
    mask |= ((1 << bit_count) - 1) << shift
    bits |= value << shift
    return mask, bits


def dump_mask(line):
    parts = line.split(" ")
    opcode = parts[0]
    patterns = parts[1:]
    assert len(patterns) > 0
    mask, bits = 0, 0
    for pattern in patterns:
        mask, bits = apply_pattern(pattern, mask, bits)
    print(f'    "{opcode}" & {hex(mask)} == {hex(bits)};')


def main():
    with open("patterns.txt", "r") as patterns, open(
        "../lib/src/isa.rs", "w"
    ) as isa_file:
        sys.stdout = isa_file
        print("use ppc750cl_macros::isa;")
        print()
        print("isa! {")
        for line in patterns.readlines():
            dump_mask(line)
        print("}")


if __name__ == "__main__":
    main()
