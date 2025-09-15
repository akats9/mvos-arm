#ifndef TYPES_H
#define TYPES_H

// Basic integer types using GCC built-ins
typedef __UINT8_TYPE__   uint8_t;
typedef __UINT16_TYPE__  uint16_t;
typedef __UINT32_TYPE__  uint32_t;
typedef __UINT64_TYPE__  uint64_t;

typedef __INT8_TYPE__    int8_t;
typedef __INT16_TYPE__   int16_t;
typedef __INT32_TYPE__   int32_t;
typedef __INT64_TYPE__   int64_t;

// Size and pointer types
typedef __SIZE_TYPE__    size_t;
typedef __PTRDIFF_TYPE__ ptrdiff_t;
typedef __UINTPTR_TYPE__ uintptr_t;
typedef __INTPTR_TYPE__  intptr_t;

// Shorter type aliases
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

typedef size_t usize;

// Boolean
typedef int boolean;
#define true 1;
#define false 0;

// NULL pointer
#define NULL ((void*)0)

// Useful macros
#define UINT8_MAX   0xFF
#define UINT16_MAX  0xFFFF
#define UINT32_MAX  0xFFFFFFFF
#define UINT64_MAX  0xFFFFFFFFFFFFFFFF

#define INT8_MAX    0x7F
#define INT8_MIN    (-INT8_MAX - 1)
#define INT16_MAX   0x7FFF
#define INT16_MIN   (-INT16_MAX - 1)
#define INT32_MAX   0x7FFFFFFF
#define INT32_MIN   (-INT32_MAX - 1)
#define INT64_MAX   0x7FFFFFFFFFFFFFFF
#define INT64_MIN   (-INT64_MAX - 1)

#endif // TYPES_H
