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

#include <stdbool.h>
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
    TYPE_BOOL,
    TYPE_CHAR,
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
        bool b;
        char c;
    } value;
} Var;

typedef struct {
    size_t length;
    size_t capacity;
    Var *data;
} List;

#define GET_VALUE(var)                             \
    ((var).type == TYPE_F64    ? ((var).value.f64) \
     : (var).type == TYPE_F32  ? ((var).value.f32) \
     : (var).type == TYPE_I64  ? ((var).value.i64) \
     : (var).type == TYPE_I32  ? ((var).value.i32) \
     : (var).type == TYPE_I16  ? ((var).value.i16) \
     : (var).type == TYPE_I8   ? ((var).value.i8)  \
     : (var).type == TYPE_U64  ? ((var).value.u64) \
     : (var).type == TYPE_U32  ? ((var).value.u32) \
     : (var).type == TYPE_U16  ? ((var).value.u16) \
     : (var).type == TYPE_U8   ? ((var).value.u8)  \
     : (var).type == TYPE_BOOL ? ((var).value.b)   \
     : (var).type == TYPE_CHAR ? ((var).value.c)   \
                               : 0)

void _assign(Var *var, double value);
void _append_var(List *list, const Var *var);
void _print(const Var *var);
void _println(const Var *var);

#endif"#;

const STD_C: &str = r#"#include "std.h"

#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

void _assign(Var *var, double value) {
    switch (var->type) {
        case TYPE_I8:
            var->value.i8 = (int8_t)value;
            break;
        case TYPE_I16:
            var->value.i16 = (int16_t)value;
            break;
        case TYPE_I32:
            var->value.i32 = (int32_t)value;
            break;
        case TYPE_I64:
            var->value.i64 = (int64_t)value;
            break;
        case TYPE_U8:
            var->value.u8 = (uint8_t)value;
            break;
        case TYPE_U16:
            var->value.u16 = (uint16_t)value;
            break;
        case TYPE_U32:
            var->value.u32 = (uint32_t)value;
            break;
        case TYPE_U64:
            var->value.u64 = (uint64_t)value;
            break;
        case TYPE_F32:
            var->value.f32 = (float)value;
            break;
        case TYPE_F64:
            var->value.f64 = (double)value;
            break;
        case TYPE_BOOL:
            var->value.b = (bool)value;
            break;
        case TYPE_CHAR:
            var->value.c = (char)value;
            break;
    }
}

void _append_var(List *list, const Var *var) {
    if (list->length >= list->capacity) {
        list->capacity *= 2;
        list->data = realloc(list->data, sizeof(Var) * list->capacity);
    }
    list->data[list->length++] = *var;
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
            printf("%f", var->value.f32);
            break;
        case TYPE_F64:
            printf("%lf", var->value.f64);
            break;
        case TYPE_BOOL:
            printf("%s", var->value.b ? "true" : "false");
            break;
        case TYPE_CHAR:
            printf("%c", var->value.c);
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
        case TYPE_BOOL:
            printf("%s\n", var->value.b ? "true" : "false");
            break;
        case TYPE_CHAR:
            printf("%c\n", var->value.c);
            break;
    }
}"#;
