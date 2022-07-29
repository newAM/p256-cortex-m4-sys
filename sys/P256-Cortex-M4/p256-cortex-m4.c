/*
 * Copyright (c) 2017-2021 Emil Lenngren
 * Copyright (c) 2021 Shortcut Labs AB
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#include <stdint.h>
#include <stdbool.h>
#include <string.h>

#include "p256-cortex-m4-config.h"
#include "p256-cortex-m4.h"

typedef const uint32_t (*constarr)[8];

struct FGInteger {
    // To get the value this struct represents,
    // interpret signed_value as a two's complement 288-bit little endian integer,
    // and negate if flip_sign is -1
    int flip_sign; // 0 or -1
    uint32_t signed_value[9]; // of 288 bits, 257 are useful (top 31 bits are sign-extended from bit 256)
};

struct XYInteger {
    // To get the value this struct represents,
    // interpret signed_value as an unsigned 288-bit little endian integer,
    // and negate if flip_sign is -1
    int flip_sign; // 0 or -1
    uint32_t value[8]; // unsigned value, 0 <= value < P256_order
};

int P256_divsteps2_31(int delta, uint32_t f, uint32_t g, uint32_t res_matrix[4]);
void P256_matrix_mul_fg_9(uint32_t a, uint32_t b, const struct FGInteger fg[2], struct FGInteger *res);
void P256_matrix_mul_mod_n(uint32_t a, uint32_t b, const struct XYInteger xy[2], struct XYInteger *res);

void P256_to_montgomery(uint32_t aR[8], const uint32_t a[8]);
void P256_from_montgomery(uint32_t a[8], const uint32_t aR[8]);
bool P256_check_range_p(const uint32_t a[8]);

bool P256_check_range_n(const uint32_t a[8]);
void P256_mul_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
void P256_add_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
void P256_mod_n_inv_vartime(uint32_t res[8], const uint32_t a[8]);
void P256_reduce_mod_n_32bytes(uint32_t res[8], const uint32_t a[8]);

void P256_select_point(uint32_t (*output)[8], uint32_t* table, uint32_t num_coordinates, uint32_t index);

void P256_jacobian_to_affine(uint32_t affine_mont_x[8], uint32_t affine_mont_y[8], const uint32_t jacobian_mont[3][8]);
bool P256_point_is_on_curve(const uint32_t x_mont[8], const uint32_t y_mont[8]);
bool P256_decompress_point(uint32_t y[8], const uint32_t x[8], uint32_t y_parity);
void P256_double_j(uint32_t jacobian_point_out[3][8], const uint32_t jacobian_point_in[3][8]);
void P256_add_sub_j(uint32_t jacobian_point1[3][8], const uint32_t (*point2)[8], bool is_sub, bool p2_is_affine);
bool P256_verify_last_step(const uint32_t r[8], const uint32_t jacobian_point[3][8]);

void P256_negate_mod_p_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
void P256_negate_mod_n_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);

extern uint32_t P256_order[9];

// Creates a representation of a (little endian integer),
// so that r[0] + 2*r[1] + 2^2*r[2] + 2^3*r[3] + ... = a,
// where each r[i] is -15, -13, ..., 11, 13, 15 or 0.
// Only around 1/5.5 of the r[i] will be non-zero.
void slide_257(signed char r[257], const uint8_t a[32]) {
    for (int i = 0; i < 256; ++i) {
        r[i] = 1 & (a[i >> 3] >> (i & 7));
    }
    r[256] = 0;

    for (int i = 0; i < 256; i++) {
        if (r[i] != 0) {
            for (int b = 1; b <= 4 && i + b < 256; b++) {
                if (r[i + b] != 0) {
                    if (r[i] + (r[i + b] << b) <= 15) {
                        r[i] += r[i + b] << b; r[i + b] = 0;
                    } else if (r[i] - (r[i + b] << b) >= -15) {
                        r[i] -= r[i + b] << b;
                        for (;;) {
                            r[i + b] = 0;
                            b++;
                            if (!r[i + b]) {
                                r[i + b] = 1;
                                b--; // Will be added back after loop footer b++
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

void P256_mod_n_inv(uint32_t out[8], const uint32_t in[8]) {
    // This function follows the algorithm in section 12.1 of https://gcd.cr.yp.to/safegcd-20190413.pdf.
    // It has been altered in the following ways:
    //   1. Due to 32-bit cpu, we use 24 * 31 iterations instead of 12 * 62.
    //   2. P-256 modulus instead of 2^255-19.
    //      744 iterations are still enough and slightly more than the required 741 (floor((49*256+57)/17)).
    //   3. Step 5 has been corrected to go back to step 2 instead of step 3.
    //   4. The order of the matrix multiplications in step 6 has been changed to (T24*(T23*(T22*(...*(T1*[0, 1]))))),
    //      where [0, 1] is a column vector to make it possible to be able to extract the "top-right corner", v, of T24*T23*...*T1.
    //      The result v will then be contained in the first element of the resulting column vector.
    
    struct {
        struct FGInteger fg[2]; // f and g
        struct XYInteger xy[2]; // x and y
    } state[2]; // "current" and "next"
    
    state[0].fg[0].flip_sign = 0; // non-negative f
    memcpy(&state[0].fg[0].signed_value, P256_order, 36); // f
    state[0].fg[1].flip_sign = 0; // non-negative g
    memcpy(&state[0].fg[1].signed_value, in, 32); // g
    state[0].fg[1].signed_value[8] = 0; // upper bits of g are 0
    memset(&state[0].xy, 0, sizeof(state[0].xy));
    // We later need a factor 2^-744. The montgomery multiplication gives 2^(24*-32)=2^-768, so multiply the init value (1) by 2^24 here.
    state[0].xy[1].value[0] = 1U << 24;
    
    int delta = 1;
    for (int i = 0; i < 24; i++) {
        // Scaled translation matrix Ti
        uint32_t matrix[4]; // element range: [-2^30, 2^31] (negative numbers are stored in two's complement form)
        
        // Decode f and g into two's complement representation and use the lowest 32 bits in the P256_divsteps2_31 calculation
        uint32_t negate_f = state[i % 2].fg[0].flip_sign;
        uint32_t negate_g = state[i % 2].fg[1].flip_sign;
        delta = P256_divsteps2_31(delta, (state[i % 2].fg[0].signed_value[0] ^ negate_f) - negate_f, (state[i % 2].fg[1].signed_value[0] ^ negate_g) - negate_g, matrix);
        
        // "Jump step", calculates the new f and g values that applies after 31 divstep2 iterations
        P256_matrix_mul_fg_9(matrix[0], matrix[1], state[i % 2].fg, &state[(i + 1) % 2].fg[0]);
        P256_matrix_mul_fg_9(matrix[2], matrix[3], state[i % 2].fg, &state[(i + 1) % 2].fg[1]);
        
        // Iterate the result vector
        // Due to montgomery multiplication inside this function, each step also adds a 2^-32 factor
        P256_matrix_mul_mod_n(matrix[0], matrix[1], state[i % 2].xy, &state[(i + 1) % 2].xy[0]);
        P256_matrix_mul_mod_n(matrix[2], matrix[3], state[i % 2].xy, &state[(i + 1) % 2].xy[1]);
    }
    // Calculates val^-1 = sgn(f) * v * 2^-744, where v is the "top-right corner" of the resulting T24*T23*...*T1 matrix.
    // In this implementation, at this point x contains v * 2^-744.
    P256_negate_mod_n_if(out, &state[0].xy[0].value[0], (state[0].xy[0].flip_sign ^ state[0].fg[0].flip_sign ^ state[0].fg[0].signed_value[8]) & 1);
}
