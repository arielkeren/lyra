use std::io::Write;

pub fn write_stdlib() {
    let mut h_file = std::fs::File::create("build/include/std.h").expect("Failed to create std.h");
    h_file
        .write_all(STD_H.as_bytes())
        .expect("Failed to write std.h");

    let mut c_file = std::fs::File::create("build/src/std.c").expect("Failed to create std.c");
    c_file
        .write_all(STD_C.as_bytes())
        .expect("Failed to write std.c");
}

const STD_H: &str = r#"#ifndef STD_H
#define STD_H

void print_binary(const unsigned char *a, int n, int m);
void print_octal(const unsigned char *a, int n, int m);
void print_hex(const unsigned char *a, int n, int m);
void print_signed(const unsigned char *a, int n, int m);
void print_unsigned(const unsigned char *a, int n, int m);

#endif"#;

const STD_C: &str = r#"#include <stdio.h>
#include "std.h"

static unsigned int bitfield_to_uint(const unsigned char *a, int n, int m) {
    unsigned int result = 0;
    for (int i = 0; i <= m - n; i++) {
        if (a[(n + i) / 8] & (1 << ((n + i) % 8))) {
            result |= (1u << i);
        }
    }
    return result;
}

static int bitfield_to_int(const unsigned char *a, int n, int m) {
    int width = m - n + 1;
    unsigned int u = bitfield_to_uint(a, n, m);
    if (u & (1u << (width - 1))) {
        u |= ~((1u << width) - 1);
    }
    return (int)u;
}

void print_binary(const unsigned char *a, int n, int m) {
    for (int i = m - n; i >= 0; i--) {
        putchar((a[(n + i) / 8] & (1 << ((n + i) % 8))) ? '1' : '0');
    }
    putchar('\n');
}

void print_octal(const unsigned char *a, int n, int m) {
    printf("%o\n", bitfield_to_uint(a, n, m));
}

void print_hex(const unsigned char *a, int n, int m) {
    printf("%x\n", bitfield_to_uint(a, n, m));
}

void print_signed(const unsigned char *a, int n, int m) {
    printf("%d\n", bitfield_to_int(a, n, m));
}

void print_unsigned(const unsigned char *a, int n, int m) {
    printf("%u\n", bitfield_to_uint(a, n, m));
}"#;
