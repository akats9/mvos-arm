#include <global_include.h>

extern uint8_t _binary_cross_framebuffer_raw_start[];
extern uint8_t _binary_cross_framebuffer_raw_end[];

void* memcpy(void* dest, const void* src, size_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    
    return dest;
}

void display_bootscreen(char* fb_addr) {
    size_t size = _binary_cross_framebuffer_raw_end - _binary_cross_framebuffer_raw_start;
    memcpy((void*)fb_addr, _binary_cross_framebuffer_raw_start, size);
}