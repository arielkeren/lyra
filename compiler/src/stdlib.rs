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

typedef enum {
    TYPE_INT,
    TYPE_FLOAT,
    TYPE_BOOL,
    TYPE_CHAR,
} Type;

typedef struct {
    Type type;
    double value;
} Var;

typedef struct {
    size_t length;
    size_t capacity;
    Type type;
    double *data;
} List;

double _mod(double a, double b);
double _convert(Type type, double value);
void _assign(Var *var, double value);
void _append(List *list, double value);
List _create_list(Type type);
void _free_memory();

#endif"#;

const STD_C: &str = r#"#include "std.h"

#include <stdio.h>
#include <stdlib.h>

#define MAX_LISTS 1000
static double *lists_to_free[MAX_LISTS];
static size_t list_count = 0;

static unsigned char _is_char(double value) {
    return (value >= 0.0 && value <= 127.0 && value == (int)value);
}

double _mod(double a, double b) {
    if (b == 0.0) {
        fprintf(stderr, "Error: Division by zero in modulo operation.\n");
        exit(EXIT_FAILURE);
    }

    if (b < 0.0) b = -b;
    return a - (b * (double)(long)(a / b));
}

double _convert(Type type, double value) {
    switch (type) {
        case TYPE_INT:
            return (int)value;
        case TYPE_FLOAT:
            return value;
        case TYPE_BOOL:
            return (value == 0.0) ? 0.0 : 1.0;
        case TYPE_CHAR:
            if (_is_char(value)) return value;
            return 0.0;
    }
    return 0.0;
}

void _assign(Var *var, double value) {
    var->value = _convert(var->type, value);
}

void _append(List *list, double value) {
    double *new_data;
    size_t i;

    if (list->length >= list->capacity) {
        list->capacity *= 2;
        new_data = realloc(list->data, sizeof(double) * list->capacity);

        if (new_data != list->data)
            for (i = 0; i < list_count; i++)
                if (lists_to_free[i] == list->data) {
                    lists_to_free[i] = new_data;
                    break;
                }

        list->data = new_data;
    }

    list->data[list->length++] = value;
}

List _create_list(Type type) {
    List list;
    if (list_count >= MAX_LISTS) {
        fprintf(stderr, "Error: Maximum number of lists exceeded.\n");
        exit(EXIT_FAILURE);
    }

    list.length = 0;
    list.capacity = 8;
    list.data = malloc(sizeof(double) * list.capacity);
    list.type = type;

    lists_to_free[list_count++] = list.data;
    return list;
}

void _free_memory() {
    size_t i;
    for (i = 0; i < list_count; i++) free(lists_to_free[i]);
    list_count = 0;
}"#;
