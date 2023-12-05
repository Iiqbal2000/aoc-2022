/**
 * 1. instructions:
 *      - addx V --> 2 cycles
 *        after two cycles, the X register is increased by the operand value
 *      - noop --> 1 cycles
 *        after a cycle noop
 * 2. cpu has single register X which starts with the value 1
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
            printf("operand changes CPU state!\n");
            printf("CPU: %d\n", *CPU);
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


int main() {
    int CPU = 1;
    char line[MAX_LINE_LENGTH];
    char opcode[MAX_OPCODE_LENGTH];
    int operand;
    int cycle = 0;
    int signal_strength = 0;

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
            printf("%s %d\n", opcode, operand);
            execute_addx(&CPU, &cycle, operand, &signal_strength);
        } else if (strcmp(opcode, "noop") == 0) {
            execute_noop(&cycle, &signal_strength, CPU);
        } else {
            fprintf(stderr, "Invalid instruction: %s\n", opcode);
        }
    }

    printf("Signal strength: %d\n", signal_strength);

    fclose(fp);
    return EXIT_SUCCESS;
}