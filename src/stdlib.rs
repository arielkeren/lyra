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
    Var *data;
} List;

double _mod(double a, double b);
double _convert(Type type, double value);
void _assign(Var *var, double value);
void _append_var(List *list, const Var *var);
void _append_literal(List *list, Type type, double value);
void _print(const Var *var);
void _println(const Var *var);
void _print_item(const List *list, size_t index);
void _println_item(const List *list, size_t index);
List _create_list();
void _free_memory();

#endif"#;

const STD_C: &str = r#"#include "std.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX_LISTS 1000
static Var *lists_to_free[MAX_LISTS];
static size_t list_count = 0;

static unsigned char _is_char(double value) {
    return (value >= 0.0 && value <= 127.0 && value == (int)value);
}

double _mod(double a, double b) {
    if (b == 0.0) {
        fprintf(stderr, "Error: Division by zero in modulo operation.\n");
        exit(EXIT_FAILURE);
    }
    return fmod(a, b);
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
}

void _assign(Var *var, double value) {
    var->value = _convert(var->type, value);
}

void _append_var(List *list, const Var *var) {
    Var *new_data;
    size_t i;

    if (list->length >= list->capacity) {
        list->capacity *= 2;
        new_data = realloc(list->data, sizeof(Var) * list->capacity);

        if (new_data != list->data) {
            for (i = 0; i < list_count; i++) {
                if (lists_to_free[i] == list->data) {
                    lists_to_free[i] = new_data;
                    break;
                }
            }
        }

        list->data = new_data;
    }
    list->data[list->length++] = *var;
}

void _append_literal(List *list, Type type, double value) {
    Var var;
    var.type = type;
    var.value = _convert(type, value);
    _append_var(list, &var);
}

void _print(const Var *var) {
    switch (var->type) {
        case TYPE_INT:
            printf("%d", (int)var->value);
            break;
        case TYPE_FLOAT:
            printf("%lf", var->value);
            break;
        case TYPE_BOOL:
            printf("%s", var->value == 0.0 ? "false" : "true");
            break;
        case TYPE_CHAR:
            if (_is_char(var->value)) printf("%c", (char)var->value);
            break;
    }
}

void _println(const Var *var) {
    switch (var->type) {
        case TYPE_INT:
            printf("%d\n", (int)var->value);
            break;
        case TYPE_FLOAT:
            printf("%lf\n", var->value);
            break;
        case TYPE_BOOL:
            printf("%s\n", var->value == 0.0 ? "false" : "true");
            break;
        case TYPE_CHAR:
            if (_is_char(var->value)) printf("%c\n", (char)var->value);
            break;
    }
}

void _print_item(const List *list, size_t index) {
    if (index < list->length) {
        _print(&list->data[index]);
    }
}

void _println_item(const List *list, size_t index) {
    if (index < list->length) {
        _println(&list->data[index]);
    }
}

List _create_list() {
    List list;
    if (list_count >= MAX_LISTS) {
        fprintf(stderr, "Error: Maximum number of lists exceeded.\n");
        exit(EXIT_FAILURE);
    }

    list.length = 0;
    list.capacity = 8;
    list.data = malloc(sizeof(Var) * list.capacity);

    lists_to_free[list_count++] = list.data;
    return list;
}

void _free_memory() {
    size_t i;
    for (i = 0; i < list_count; i++) free(lists_to_free[i]);
    list_count = 0;
}"#;
