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

#include <stdint.h>

typedef enum {
    TYPE_I8,
    TYPE_I16,
    TYPE_I32,
    TYPE_I64,
    TYPE_U8,
    TYPE_U16,
    TYPE_U32,
    TYPE_U64,
    TYPE_F32,
    TYPE_F64,
} VarType;

typedef struct {
    VarType type;
    union {
        int8_t i8;
        int16_t i16;
        int32_t i32;
        int64_t i64;
        uint8_t u8;
        uint16_t u16;
        uint32_t u32;
        uint64_t u64;
        float f32;
        double f64;
    } value;
} Var;

void _print(const Var *var);
void _println(const Var *var);
void _assign_var(Var *dest, const Var *src);

#endif"#;

const STD_C: &str = r#"#include "std.h"

#include <inttypes.h>
#include <stdio.h>

void _assign_var(Var *dest, const Var *src) {
    switch (dest->type) {
        case TYPE_I8:
            switch (src->type) {
                case TYPE_I8:   dest->value.i8  = src->value.i8; break;
                case TYPE_I16:  dest->value.i8  = (int8_t)src->value.i16; break;
                case TYPE_I32:  dest->value.i8  = (int8_t)src->value.i32; break;
                case TYPE_I64:  dest->value.i8  = (int8_t)src->value.i64; break;
                case TYPE_U8:   dest->value.i8  = (int8_t)src->value.u8; break;
                case TYPE_U16:  dest->value.i8  = (int8_t)src->value.u16; break;
                case TYPE_U32:  dest->value.i8  = (int8_t)src->value.u32; break;
                case TYPE_U64:  dest->value.i8  = (int8_t)src->value.u64; break;
                case TYPE_F32:  dest->value.i8  = (int8_t)src->value.f32; break;
                case TYPE_F64:  dest->value.i8  = (int8_t)src->value.f64; break;
            }
            break;
        case TYPE_I16:
            switch (src->type) {
                case TYPE_I8:   dest->value.i16 = (int16_t)src->value.i8; break;
                case TYPE_I16:  dest->value.i16 = src->value.i16; break;
                case TYPE_I32:  dest->value.i16 = (int16_t)src->value.i32; break;
                case TYPE_I64:  dest->value.i16 = (int16_t)src->value.i64; break;
                case TYPE_U8:   dest->value.i16 = (int16_t)src->value.u8; break;
                case TYPE_U16:  dest->value.i16 = (int16_t)src->value.u16; break;
                case TYPE_U32:  dest->value.i16 = (int16_t)src->value.u32; break;
                case TYPE_U64:  dest->value.i16 = (int16_t)src->value.u64; break;
                case TYPE_F32:  dest->value.i16 = (int16_t)src->value.f32; break;
                case TYPE_F64:  dest->value.i16 = (int16_t)src->value.f64; break;
            }
            break;
        case TYPE_I32:
            switch (src->type) {
                case TYPE_I8:   dest->value.i32 = (int32_t)src->value.i8; break;
                case TYPE_I16:  dest->value.i32 = (int32_t)src->value.i16; break;
                case TYPE_I32:  dest->value.i32 = src->value.i32; break;
                case TYPE_I64:  dest->value.i32 = (int32_t)src->value.i64; break;
                case TYPE_U8:   dest->value.i32 = (int32_t)src->value.u8; break;
                case TYPE_U16:  dest->value.i32 = (int32_t)src->value.u16; break;
                case TYPE_U32:  dest->value.i32 = (int32_t)src->value.u32; break;
                case TYPE_U64:  dest->value.i32 = (int32_t)src->value.u64; break;
                case TYPE_F32:  dest->value.i32 = (int32_t)src->value.f32; break;
                case TYPE_F64:  dest->value.i32 = (int32_t)src->value.f64; break;
            }
            break;
        case TYPE_I64:
            switch (src->type) {
                case TYPE_I8:   dest->value.i64 = (int64_t)src->value.i8; break;
                case TYPE_I16:  dest->value.i64 = (int64_t)src->value.i16; break;
                case TYPE_I32:  dest->value.i64 = (int64_t)src->value.i32; break;
                case TYPE_I64:  dest->value.i64 = src->value.i64; break;
                case TYPE_U8:   dest->value.i64 = (int64_t)src->value.u8; break;
                case TYPE_U16:  dest->value.i64 = (int64_t)src->value.u16; break;
                case TYPE_U32:  dest->value.i64 = (int64_t)src->value.u32; break;
                case TYPE_U64:  dest->value.i64 = (int64_t)src->value.u64; break;
                case TYPE_F32:  dest->value.i64 = (int64_t)src->value.f32; break;
                case TYPE_F64:  dest->value.i64 = (int64_t)src->value.f64; break;
            }
            break;
        case TYPE_U8:
            switch (src->type) {
                case TYPE_I8:   dest->value.u8  = (uint8_t)src->value.i8; break;
                case TYPE_I16:  dest->value.u8  = (uint8_t)src->value.i16; break;
                case TYPE_I32:  dest->value.u8  = (uint8_t)src->value.i32; break;
                case TYPE_I64:  dest->value.u8  = (uint8_t)src->value.i64; break;
                case TYPE_U8:   dest->value.u8  = src->value.u8; break;
                case TYPE_U16:  dest->value.u8  = (uint8_t)src->value.u16; break;
                case TYPE_U32:  dest->value.u8  = (uint8_t)src->value.u32; break;
                case TYPE_U64:  dest->value.u8  = (uint8_t)src->value.u64; break;
                case TYPE_F32:  dest->value.u8  = (uint8_t)src->value.f32; break;
                case TYPE_F64:  dest->value.u8  = (uint8_t)src->value.f64; break;
            }
            break;
        case TYPE_U16:
            switch (src->type) {
                case TYPE_I8:   dest->value.u16 = (uint16_t)src->value.i8; break;
                case TYPE_I16:  dest->value.u16 = (uint16_t)src->value.i16; break;
                case TYPE_I32:  dest->value.u16 = (uint16_t)src->value.i32; break;
                case TYPE_I64:  dest->value.u16 = (uint16_t)src->value.i64; break;
                case TYPE_U8:   dest->value.u16 = (uint16_t)src->value.u8; break;
                case TYPE_U16:  dest->value.u16 = src->value.u16; break;
                case TYPE_U32:  dest->value.u16 = (uint16_t)src->value.u32; break;
                case TYPE_U64:  dest->value.u16 = (uint16_t)src->value.u64; break;
                case TYPE_F32:  dest->value.u16 = (uint16_t)src->value.f32; break;
                case TYPE_F64:  dest->value.u16 = (uint16_t)src->value.f64; break;
            }
            break;
        case TYPE_U32:
            switch (src->type) {
                case TYPE_I8:   dest->value.u32 = (uint32_t)src->value.i8; break;
                case TYPE_I16:  dest->value.u32 = (uint32_t)src->value.i16; break;
                case TYPE_I32:  dest->value.u32 = (uint32_t)src->value.i32; break;
                case TYPE_I64:  dest->value.u32 = (uint32_t)src->value.i64; break;
                case TYPE_U8:   dest->value.u32 = (uint32_t)src->value.u8; break;
                case TYPE_U16:  dest->value.u32 = (uint32_t)src->value.u16; break;
                case TYPE_U32:  dest->value.u32 = src->value.u32; break;
                case TYPE_U64:  dest->value.u32 = (uint32_t)src->value.u64; break;
                case TYPE_F32:  dest->value.u32 = (uint32_t)src->value.f32; break;
                case TYPE_F64:  dest->value.u32 = (uint32_t)src->value.f64; break;
            }
            break;
        case TYPE_U64:
            switch (src->type) {
                case TYPE_I8:   dest->value.u64 = (uint64_t)src->value.i8; break;
                case TYPE_I16:  dest->value.u64 = (uint64_t)src->value.i16; break;
                case TYPE_I32:  dest->value.u64 = (uint64_t)src->value.i32; break;
                case TYPE_I64:  dest->value.u64 = (uint64_t)src->value.i64; break;
                case TYPE_U8:   dest->value.u64 = (uint64_t)src->value.u8; break;
                case TYPE_U16:  dest->value.u64 = (uint64_t)src->value.u16; break;
                case TYPE_U32:  dest->value.u64 = (uint64_t)src->value.u32; break;
                case TYPE_U64:  dest->value.u64 = src->value.u64; break;
                case TYPE_F32:  dest->value.u64 = (uint64_t)src->value.f32; break;
                case TYPE_F64:  dest->value.u64 = (uint64_t)src->value.f64; break;
            }
            break;
        case TYPE_F32:
            switch (src->type) {
                case TYPE_I8:   dest->value.f32 = (float)src->value.i8; break;
                case TYPE_I16:  dest->value.f32 = (float)src->value.i16; break;
                case TYPE_I32:  dest->value.f32 = (float)src->value.i32; break;
                case TYPE_I64:  dest->value.f32 = (float)src->value.i64; break;
                case TYPE_U8:   dest->value.f32 = (float)src->value.u8; break;
                case TYPE_U16:  dest->value.f32 = (float)src->value.u16; break;
                case TYPE_U32:  dest->value.f32 = (float)src->value.u32; break;
                case TYPE_U64:  dest->value.f32 = (float)src->value.u64; break;
                case TYPE_F32:  dest->value.f32 = src->value.f32; break;
                case TYPE_F64:  dest->value.f32 = (float)src->value.f64; break;
            }
            break;
        case TYPE_F64:
            switch (src->type) {
                case TYPE_I8:   dest->value.f64 = (double)src->value.i8; break;
                case TYPE_I16:  dest->value.f64 = (double)src->value.i16; break;
                case TYPE_I32:  dest->value.f64 = (double)src->value.i32; break;
                case TYPE_I64:  dest->value.f64 = (double)src->value.i64; break;
                case TYPE_U8:   dest->value.f64 = (double)src->value.u8; break;
                case TYPE_U16:  dest->value.f64 = (double)src->value.u16; break;
                case TYPE_U32:  dest->value.f64 = (double)src->value.u32; break;
                case TYPE_U64:  dest->value.f64 = (double)src->value.u64; break;
                case TYPE_F32:  dest->value.f64 = (double)src->value.f32; break;
                case TYPE_F64:  dest->value.f64 = src->value.f64; break;
            }
            break;
    }
}

void _print(const Var *var) {
    switch (var->type) {
        case TYPE_I8:
            printf("%" PRId8, var->value.i8);
            break;
        case TYPE_I16:
            printf("%" PRId16, var->value.i16);
            break;
        case TYPE_I32:
            printf("%" PRId32, var->value.i32);
            break;
        case TYPE_I64:
            printf("%" PRId64, var->value.i64);
            break;
        case TYPE_U8:
            printf("%" PRIu8, var->value.u8);
            break;
        case TYPE_U16:
            printf("%" PRIu16, var->value.u16);
            break;
        case TYPE_U32:
            printf("%" PRIu32, var->value.u32);
            break;
        case TYPE_U64:
            printf("%" PRIu64, var->value.u64);
            break;
        case TYPE_F32:
            printf("%f\n", var->value.f32);
            break;
        case TYPE_F64:
            printf("%lf\n", var->value.f64);
            break;
    }
}

void _println(const Var *var) {
    switch (var->type) {
        case TYPE_I8:
            printf("%" PRId8 "\n", var->value.i8);
            break;
        case TYPE_I16:
            printf("%" PRId16 "\n", var->value.i16);
            break;
        case TYPE_I32:
            printf("%" PRId32 "\n", var->value.i32);
            break;
        case TYPE_I64:
            printf("%" PRId64 "\n", var->value.i64);
            break;
        case TYPE_U8:
            printf("%" PRIu8 "\n", var->value.u8);
            break;
        case TYPE_U16:
            printf("%" PRIu16 "\n", var->value.u16);
            break;
        case TYPE_U32:
            printf("%" PRIu32 "\n", var->value.u32);
            break;
        case TYPE_U64:
            printf("%" PRIu64 "\n", var->value.u64);
            break;
        case TYPE_F32:
            printf("%f\n", var->value.f32);
            break;
        case TYPE_F64:
            printf("%lf\n", var->value.f64);
            break;
    }
}"#;
