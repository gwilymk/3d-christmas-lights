/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#include "pico/stdlib.h"
#include "hardware/pio.h"
#include "hardware/clocks.h"
#include "ws2812.pio.h"

#include "lights.h"
#include "hsv.h"

#define IS_RGBW false
#define NUM_PIXELS 100

#define BUTTON_PIN 5

#define WS2812_PIN 3

static inline void put_pixel(uint32_t pixel_grb)
{
    pio_sm_put_blocking(pio0, 0, pixel_grb << 8u);
}

static inline uint32_t urgb_u32(uint8_t r, uint8_t g, uint8_t b)
{
    return ((uint32_t)(r) << 8) |
           ((uint32_t)(g) << 16) |
           (uint32_t)(b);
}

void go_all_up(uint len, uint t)
{
    uint up_to = (t % 200) * 10; // approximately 1000 is max y value

    if (up_to > 1000)
    {
        up_to = 2000 - up_to;
    }

    for (int i = 0; i < len; i++)
    {
        int y_position = light_positions[i * 3 + 1];

        if (y_position > up_to)
        {
            put_pixel(urgb_u32(255, 0, 0));
        }
        else
        {
            put_pixel(urgb_u32(0, 255, 0));
        }
    }
}

void left_right_sweep(uint len, uint t)
{
    // 500 is the approx range
    int t2 = (int)((t * 3) % 1000);

    // create a sweep
    if (t2 > 500)
    {
        t2 = 1000 - t2 - 250;
    }
    else
    {
        t2 -= 250;
    }

    int t3 = t % 360;

    for (int i = 0; i < len; i++)
    {
        int x_position = light_positions[i * 3];

        if (x_position > t2)
        {
            put_pixel(hsv2rgb(x_position / 20 + t3, 255, 150));
        }
        else
        {
            put_pixel(hsv2rgb(x_position / 20 + t3 + 180, 255, 150));
        }
    }
}

static void up_down_rainbow(uint len, uint t)
{
    for (int i = 0; i < len; i++)
    {
        put_pixel(hsv2rgb(t + light_positions[i * 3 + 1] / 10, 255, 150));
    }
}

static void forward_backward_rainbow(uint len, uint t)
{
    for (int i = 0; i < len; i++)
    {
        put_pixel(hsv2rgb(t + light_positions[i * 3 + 2] / 10, 255, 150));
    }
}

static void spin(uint len, uint t)
{
    uint theta = t % 100;

    float angles[len];
    for (int i = 0; i < len; i++)
    {
        angles[i] = atan2f((float)light_positions[i * 3], (float)light_positions[i * 3 + 2]);
    }

    float thetaf = ((float)theta) / (100.0 / M_TWOPI) - M_PI;

    for (int i = 0; i < len; i++)
    {
        float diff = fabsf(thetaf - angles[i]);
        if (diff < 0.2 || diff > M_TWOPI - 0.2)
        {
            put_pixel(hsv2rgb(t, 255, 150));
        }
        else
        {
            put_pixel(0);
        }
    }
}

static void double_spin(uint len, uint t)
{
    uint theta = t % 100;

    float angles[len];
    for (int i = 0; i < len; i++)
    {
        angles[i] = atan2f((float)light_positions[i * 3], (float)light_positions[i * 3 + 2]);
    }

    float thetaf = ((float)theta) / (100.0 / M_TWOPI) - M_PI;
    float theta2f = thetaf - M_PI;

    if (theta2f < -M_PI)
    {
        theta2f += M_TWOPI;
    }

    for (int i = 0; i < len; i++)
    {
        float diff = fabsf(thetaf - angles[i]);
        float diff2 = fabsf(theta2f - angles[i]);

        if (diff < 0.2 || diff > M_TWOPI - 0.2 || diff2 < 0.2 || diff > M_TWOPI - 0.2)
        {
            put_pixel(hsv2rgb(t, 255, 150));
        }
        else
        {
            put_pixel(0);
        }
    }
}

typedef void (*pattern)(uint len, uint t);
const struct
{
    pattern pat;
    const char *name;
} pattern_table[] = {
    {&go_all_up, "Up down sweep"},
    {&left_right_sweep, "Left right sweep"},
    {&up_down_rainbow, "up down rainbow"},
    {&forward_backward_rainbow, "forward backward rainbow"},
    {&spin, "spin round"},
    {&double_spin, "double spin"},
};

int current_led = 0;

// debounce control
unsigned long time;
const int delay_time = 250;

uint8_t read_char()
{
    while (true)
    {
        int32_t c = getchar_timeout_us(0);

        if (c != PICO_ERROR_TIMEOUT)
        {
            return (uint8_t)c;
        }
    }
}

void one_at_a_time()
{
    while (true)
    {
        for (int i = 0; i < NUM_PIXELS; i++)
        {
            if (i == current_led)
            {
                put_pixel(urgb_u32(255, 255, 255));
            }
            else
            {
                put_pixel(0);
            }
        }

        current_led = read_char();
        printf("Button presed %i!\n", current_led);
    }
}

int main()
{
    //set_sys_clock_48();
    stdio_init_all();
    time = to_ms_since_boot(get_absolute_time());

    gpio_init(BUTTON_PIN);
    gpio_pull_up(BUTTON_PIN);

    printf("WS2812 christmas lights, using pin %d", WS2812_PIN);

    // todo get free sm
    PIO pio = pio0;
    int sm = 0;
    uint offset = pio_add_program(pio, &ws2812_program);

    ws2812_program_init(pio, sm, offset, WS2812_PIN, 800000, IS_RGBW);

    if (!gpio_get(BUTTON_PIN))
    {
        gpio_init(PICO_DEFAULT_LED_PIN);
        gpio_set_dir(PICO_DEFAULT_LED_PIN, GPIO_OUT);
        gpio_put(PICO_DEFAULT_LED_PIN, 1);

        one_at_a_time();
    }

    while (true)
    {
        int pat = rand() % count_of(pattern_table);
        int dir = (rand() >> 30) & 1 ? 1 : -1;
        puts(pattern_table[pat].name);
        puts(dir == 1 ? "(forward)" : "(backward)");
        for (int i = 0; i < 3000; ++i)
        {
            int t = dir == 1 ? i : 3000 - i;
            pattern_table[pat].pat(NUM_PIXELS, t);
            sleep_ms(10);
        }
    }
}