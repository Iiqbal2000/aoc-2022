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
#define CYCLES_OF_INTEREST {20, 60, 100, 140, 180, 220}
#define CRT_W 40
#define CRT_H 6
#define ACTIVE_PIXEL_LENGTH 3

typedef struct {
    int x_regs;
    int cycle;
    char opcode[MAX_OPCODE_LENGTH];
    int operand;
} CPU;

typedef struct {
    int screen[CRT_H][CRT_W];
    int current_h_pos;
    int current_w_pos;
    int sprite[CRT_W];
    int active_pixel_pos[ACTIVE_PIXEL_LENGTH];
} CRT;

void update_signal_strength(int *signal_strength, CPU *cpu) {
    int cycles[] = CYCLES_OF_INTEREST;
    for (int i = 0; i < sizeof(cycles)/sizeof(cycles[0]); i++) {
        if (cpu->cycle == cycles[i]) {
            *signal_strength += cpu->cycle * cpu->x_regs;
            break;
        }
    }
}

void inc_cycle(CPU *cpu, int *signal_strength) {
    cpu->cycle += 1;
    update_signal_strength(signal_strength, cpu);
}

void update_sprite_pixel(CRT *crt, CPU *cpu) {
    memset(crt->sprite, 0, CRT_W * sizeof(int));
    int active_pixel_pos[] = {cpu->x_regs - 1, cpu->x_regs, cpu->x_regs + 1};

    for (int i = 0; i < ACTIVE_PIXEL_LENGTH; i++) {
        int current_pos = active_pixel_pos[i]; 
        if (current_pos >= 0 && current_pos < CRT_W) {
            crt->sprite[current_pos] = 1;
            active_pixel_pos[i] = current_pos;
        }
    }

    memcpy(crt->active_pixel_pos, active_pixel_pos, sizeof(active_pixel_pos));
}

void update_crt_screen(CRT *crt) {
    if (crt->active_pixel_pos[0] == crt->current_w_pos || crt->active_pixel_pos[1] == crt->current_w_pos || crt->active_pixel_pos[2] == crt->current_w_pos) {
        crt->screen[crt->current_h_pos][crt->current_w_pos] = 1;
    }

    crt->current_w_pos += 1;
    if (crt->current_w_pos == CRT_W) {
        crt->current_h_pos += 1;
        crt->current_w_pos = 0;
    }
}

void print_sprite(int sprite[CRT_W]) {
    for (int i = 0; i < CRT_W; i++) {
        printf("%c", sprite[i] ? '#' : '.');
    }
    printf("\n");
}

void print_crt_screen(CRT *crt) {
     printf("CRT screen:\n");
    for (int row = 0; row < CRT_H; row++) {
        print_sprite(crt->screen[row]);
    }
    printf("\n\n");
}

int main() {
    CPU cpu = {
        .x_regs = 1,
        .cycle = 0,
        .opcode = {'\0'},
        .operand = 0,
    };

    CRT crt = {
        .screen = {0},
        .current_h_pos = 0,
        .current_w_pos = 0,
        .active_pixel_pos = {0},
        .sprite = {0},
    };

    int signal_strength = 0;
    char line[MAX_LINE_LENGTH];

    update_sprite_pixel(&crt, &cpu);

    FILE *fp = fopen("./input.txt", "r");
    if (fp == NULL) {
        perror("Error opening file");
        return EXIT_FAILURE;
    }

    while(fgets(line, MAX_LINE_LENGTH, fp) != NULL) {   
        if (sscanf(line, "%s", cpu.opcode) != 1) {
            fprintf(stderr, "Error parsing line: %s\n", line);
            continue;
        }

        if (strcmp(cpu.opcode, "addx") == 0) {
            sscanf(line, "%s %d", cpu.opcode, &cpu.operand);
            printf("Start cycle %d: begin executing %s %d\n", cpu.cycle + 1, cpu.opcode, cpu.operand);

            for (int cycle_i = 0; cycle_i < 2; cycle_i++) {
                inc_cycle(&cpu, &signal_strength);
                
                printf("During cycle %d: CRT draws pixel in position %d\n", cpu.cycle, crt.current_w_pos);

                update_crt_screen(&crt);

                printf("Current CRT row: ");
                print_sprite(crt.screen[crt.current_h_pos]);

                if (cycle_i == 1) {
                    cpu.x_regs += cpu.operand;
                    update_sprite_pixel(&crt, &cpu);
                }
            }

            printf("Finish executing %s %d (Register X is now %d)\n", cpu.opcode, cpu.operand, cpu.x_regs);
            printf("Sprite row: ");
            print_sprite(crt.sprite);
        } else if (strcmp(cpu.opcode, "noop") == 0) {
            printf("Start cycle %d: begin executing %s\n", cpu.cycle + 1, cpu.opcode);

            inc_cycle(&cpu, &signal_strength);
            
            printf("During cycle %d: CRT draws pixel in position %d\n", cpu.cycle, crt.current_w_pos);

            update_crt_screen(&crt);

            printf("Current CRT row: ");
            print_sprite(crt.screen[crt.current_h_pos]);

            printf("Finish executing %s\n", cpu.opcode);
            printf("Sprite row: ");
            print_sprite(crt.sprite);
        } else {
            fprintf(stderr, "Invalid instruction: %s\n", cpu.opcode);
        }

        // PART 2
        print_crt_screen(&crt);
    }

    // PART 1
    printf("Signal strength: %d\n", signal_strength);

    fclose(fp);
    return EXIT_SUCCESS;
}