#ifndef APA_H
#define APA_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct
    {
        uint8_t r;
        uint8_t g;
        uint8_t b;
    } CRGB8;

    int apa_write_direct(
        void *spi_ptr,
        uint8_t *buffer_ptr,
        size_t buffer_len,
        size_t num_leds,
        const CRGB8 *colors_ptr,
        size_t colors_len);

#ifdef __cplusplus
}
#endif

#endif
