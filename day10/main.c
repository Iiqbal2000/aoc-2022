/**
 * PART 1:
 * 1. instructions:
 *      - addx V --> 2 cycles
 *        after two cycles, the X register is increased by the operand value
 *      - noop --> 1 cycles
 *        after a cycle noop
 * 2. cpu has single register X which starts with the value 1
 * 3. the value of the signal strength will be decided by multiplying the cycle number by the value of the register
 * 4. write the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles
 * 5. get a final result of the signal strength
 * 
 * PART 2:
 * 1. Untuk menentukan apakah sebuah piksel pada layar CRT terlihat atau tidak, perhatikan posisi sprite relatif terhadap posisi piksel yang sedang digambar oleh CRT
 * 2. lebar sprite 3 piksel, dan posisi tengahnya ditentukan oleh nilai X di register CPU. Misalnya, jika X adalah 5, maka sprite akan menempati posisi 4, 5, dan 6 (karena 5 adalah tengahnya).
 * 3. CRT menggambar satu piksel per siklus clock. Piksel-piksel ini digambar dari kiri ke kanan, dimulai dari baris paling atas. Jadi, pada siklus pertama, piksel di posisi 0 (paling kiri atas) akan digambar, dan seterusnya sampai posisi 39 (paling kanan atas) di siklus ke-40.
 * 4. Menentukan Visibilitas Piksel:
 *  - Pada setiap siklus, cek posisi sprite dan piksel yang sedang digambar oleh CRT.
 *  - Jika salah satu dari tiga piksel sprite (berdasarkan posisi tengahnya yang dikontrol oleh X) adalah piksel yang sedang digambar, maka piksel tersebut terlihat (#).
 *  - Jika tidak, maka piksel tersebut tetap gelap (.).
 *  - Contoh:
 *      - Misalkan pada siklus tertentu, nilai X adalah 5. Maka, sprite akan menempati posisi 4, 5, dan 6.
 *      - Jika CRT sedang menggambar piksel di salah satu dari posisi-posisi ini (misalnya posisi 5), piksel itu akan terlihat.
 *      - Jika CRT menggambar piksel di posisi lain (misalnya posisi 3 atau 7), piksel itu akan tetap gelap.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINE_LENGTH 20
#define MAX_OPCODE_LENGTH 10
#define CYCLE_20TH 20
#define CYCLE_60TH 60
#define CYCLE_100TH 100
#define CYCLE_140TH 140
#define CYCLE_180TH 180
#define CYCLE_220TH 220
#define CRT_W 40
#define CRT_H 6

void update_signal_strength(int *signal_strength, int cycle, int CPU) {
    if (cycle == CYCLE_20TH || cycle == CYCLE_60TH || cycle == CYCLE_100TH ||
        cycle == CYCLE_140TH || cycle == CYCLE_180TH || cycle == CYCLE_220TH) {
        *signal_strength += cycle * CPU;
    }
}

void execute_addx(int *CPU, int *cycle, int operand, int *signal_strength) {
    const int cycle_n = 2;
    for (int cycle_i = 0; cycle_i < cycle_n; cycle_i++) {
        (*cycle)++;
        update_signal_strength(signal_strength, *cycle, *CPU);
        if (cycle_i == 1) {
            *CPU += operand;
        }
    }
}

void execute_noop(int *cycle, int *signal_strength, int CPU) {
    const int cycle_n = 1;
    for (int cycle_i = 0; cycle_i < cycle_n; cycle_i++) {
        (*cycle)++;
        update_signal_strength(signal_strength, *cycle, CPU);
    }
}

void print_sprite(int sprite[CRT_W]) {
    // printf("sprite: ");
    for (int i = 0; i < CRT_W; i++) {
        if (sprite[i] == 1) {
            printf("#");
        } else {
            printf(".");
        }
    }
    printf("\n");
}

void print_crt_screen(int crt_screen[CRT_H][CRT_W]) {
    // printf("CRT screen:\n");
    for (int row = 0; row < CRT_H; row++) {
        for (int col = 0; col < CRT_W; col++) {
            if (crt_screen[row][col] == 1) {
                printf("#");
            } else {
                printf(".");
            }   
        }
        printf("\n");
    }
    printf("\n\n");
}


int main() {
    int CPU = 1;
    char line[MAX_LINE_LENGTH];
    char opcode[MAX_OPCODE_LENGTH];
    int operand;
    int cycle = 0;
    int signal_strength = 0;
    int sprite[CRT_W] = {0};
    int sprite_i[3];
    int crt_screen[CRT_H][CRT_W] = {0};

    int crt_h_i = 0;
    int crt_w_i = 0;

    int left_pixel_pos = CPU - 1;
    int center_pixel_pos = CPU;
    int right_pixel_pos = CPU + 1;
    if ((left_pixel_pos >= 0 && left_pixel_pos < CRT_W)) {
        sprite[left_pixel_pos] = 1;
    } 
    if ((center_pixel_pos >= 0 && center_pixel_pos < CRT_W)) {
        sprite[center_pixel_pos] = 1;
    } 
    if ((right_pixel_pos >= 0 && right_pixel_pos < CRT_W)) {
        sprite[right_pixel_pos] = 1;
    }

    sprite_i[0] = left_pixel_pos;
    sprite_i[1] = center_pixel_pos;
    sprite_i[2] = right_pixel_pos;

    if (sprite_i[0] == crt_w_i || sprite_i[1] == crt_w_i || sprite_i[2] == crt_w_i) {
        crt_screen[crt_h_i][crt_w_i] = 1;
    }

    FILE *fp = fopen("./input.txt", "r");
    if (fp == NULL) {
        perror("Error opening file");
        return EXIT_FAILURE;
    }

    while(fgets(line, 20, fp) != NULL) {
        if (sscanf(line, "%s", opcode) != 1) {
            fprintf(stderr, "Error parsing line: %s\n", line);
            continue;
        }

        if (strcmp(opcode, "addx") == 0) {
            sscanf(line, "%s %d", opcode, &operand);
            // print_sprite(sprite);
            // execute_addx(&CPU, &cycle, operand, &signal_strength);
            printf("Start cycle  %d: begin executing %s %d\n", cycle+1, opcode, operand);
            const int cycle_n = 2;
            for (int cycle_i = 0; cycle_i < cycle_n; cycle_i++) {
                cycle++;
                printf("During cycle %d: CRT draws pixel in position %d\n", cycle, crt_w_i);
                
                if (sprite_i[0] == crt_w_i || sprite_i[1] == crt_w_i || sprite_i[2] == crt_w_i) {
                    crt_screen[crt_h_i][crt_w_i] = 1;
                }
                printf("Current CRT row: ");
                print_sprite(crt_screen[crt_h_i]);

                crt_w_i += 1;
                if (crt_w_i == CRT_W) {
                    crt_h_i += 1;
                    crt_w_i = 0;
                }

                update_signal_strength(&signal_strength, cycle, CPU);
                if (cycle_i == 1) {
                    CPU += operand;
                }
            }
            
            int local_sprite[CRT_W] = {0};
            int left_pixel_pos = CPU - 1;
            int center_pixel_pos = CPU;
            int right_pixel_pos = CPU + 1;
            if ((left_pixel_pos >= 0 && left_pixel_pos < CRT_W)) {
                local_sprite[left_pixel_pos] = 1;
            } 
            if ((center_pixel_pos >= 0 && center_pixel_pos < CRT_W)) {
                local_sprite[center_pixel_pos] = 1;
            } 
            if ((right_pixel_pos >= 0 && right_pixel_pos < CRT_W)) {
                local_sprite[right_pixel_pos] = 1;
            }

            sprite_i[0] = left_pixel_pos;
            sprite_i[1] = center_pixel_pos;
            sprite_i[2] = right_pixel_pos;

            memcpy(sprite, local_sprite, sizeof sprite);
            printf("Finish executing %s %d (Register X is now %d)\n", opcode, operand, CPU);
            printf("Sprite row: ");
            print_sprite(sprite);
        } else if (strcmp(opcode, "noop") == 0) {
            // print_sprite(sprite);
            // execute_noop(&cycle, &signal_strength, CPU);
            printf("Start cycle  %d: begin executing %s\n", cycle+1, opcode);

            const int cycle_n = 1;
            for (int cycle_i = 0; cycle_i < cycle_n; cycle_i++) {
                cycle++;
                printf("During cycle %d: CRT draws pixel in position %d\n", cycle, crt_w_i);

                if (sprite_i[0] == crt_w_i || sprite_i[1] == crt_w_i || sprite_i[2] == crt_w_i) {
                    crt_screen[crt_h_i][crt_w_i] = 1;
                }
                printf("Current CRT row: ");
                print_sprite(crt_screen[crt_h_i]);
                crt_w_i += 1;
                if (crt_w_i == CRT_W) {
                    crt_h_i += 1;
                    crt_w_i = 0;
                }
                update_signal_strength(&signal_strength, cycle, CPU);
            }

            int local_sprite[CRT_W] = {0};
            int left_pixel_pos = CPU - 1;
            int center_pixel_pos = CPU;
            int right_pixel_pos = CPU + 1;
            if ((left_pixel_pos >= 0 && left_pixel_pos < CRT_W)) {
                local_sprite[left_pixel_pos] = 1;
            } 
            if ((center_pixel_pos >= 0 && center_pixel_pos < CRT_W)) {
                local_sprite[center_pixel_pos] = 1;
            } 
            if ((right_pixel_pos >= 0 && right_pixel_pos < CRT_W)) {
                local_sprite[right_pixel_pos] = 1;
            }

            sprite_i[0] = left_pixel_pos;
            sprite_i[1] = center_pixel_pos;
            sprite_i[2] = right_pixel_pos;

            memcpy(sprite, local_sprite, sizeof sprite);
            printf("Finish executing %s\n", opcode);
            printf("Sprite row: ");
            print_sprite(sprite);
        } else {
            fprintf(stderr, "Invalid instruction: %s\n", opcode);
        }
    
        print_crt_screen(crt_screen);
    }

    printf("Signal strength: %d\n", signal_strength);

    fclose(fp);
    return EXIT_SUCCESS;
}