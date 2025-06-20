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
#include <stddef.h>

unsigned char *_alloc(size_t bits);
void _assign(unsigned char *var, size_t start, size_t end, unsigned int value);
void _print_binary(const unsigned char *var, size_t start, size_t end);
void _print_octal(const unsigned char *var, size_t start, size_t end);
void _print_hex(const unsigned char *var, size_t start, size_t end);
void _print_signed(const unsigned char *var, size_t start, size_t end);
void _print_unsigned(const unsigned char *var, size_t start, size_t end);

#endif"#;

const STD_C: &str = r#"#include <stdio.h>
#include <stdlib.h>
#include "std.h"

static unsigned int _var_to_uint(const unsigned char *var, size_t start, size_t end) {
    unsigned int result = 0;
    for (size_t i = 0; i <= end - start; i++) {
        if (var[(start + i) / 8] & (1 << ((start + i) % 8))) {
            result |= (1u << i);
        }
    }
    return result;
}

static int _var_to_int(const unsigned char *var, size_t start, size_t end) {
    size_t width = end - start + 1;
    unsigned int u = _var_to_uint(var, start, end);
    if (u & (1u << (width - 1))) {
        u |= ~((1u << width) - 1);
    }
    return (int)u;
}

unsigned char *_alloc(size_t bits) {
    unsigned char *var = calloc((bits + 7) / 8, 1);
    if (!var) {
        fprintf(stderr, "Memory allocation failed for var\n");
        exit(1);
    }
    return var;
}

void _assign(unsigned char *var, size_t start, size_t end, unsigned int value) {
    for (size_t i = 0; i <= end - start; i++) {
        if (value & (1u << i))
            var[(start + i) / 8] |= (1u << ((start + i) % 8));
        else
            var[(start + i) / 8] &= ~(1u << ((start + i) % 8));
    }
}

void _print_binary(const unsigned char *var, size_t start, size_t end) {
    for (size_t i = end - start + 1; i-- > 0;) {
        putchar((var[(start + i) / 8] & (1 << ((start + i) % 8))) ? '1' : '0');
    }
    putchar('\n');
}

void _print_octal(const unsigned char *var, size_t start, size_t end) {
    printf("%o\n", _var_to_uint(var, start, end));
}

void _print_hex(const unsigned char *var, size_t start, size_t end) {
    printf("%x\n", _var_to_uint(var, start, end));
}

void _print_signed(const unsigned char *var, size_t start, size_t end) {
    printf("%d\n", _var_to_int(var, start, end));
}

void _print_unsigned(const unsigned char *var, size_t start, size_t end) {
    printf("%u\n", _var_to_uint(var, start, end));
}"#;
