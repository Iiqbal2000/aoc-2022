#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define STR_LEN_MAX 10
#define STR_BUF_SIZE 6000

typedef struct {
  int y, x;
} Point;

void move_rope_by_cmd(Point *head, char direction);
void follow_rope(Point *followed, Point *follower, int is_tracked, char bucket[STR_BUF_SIZE][STR_LEN_MAX]);
int count_rope_position(char result[STR_BUF_SIZE][STR_LEN_MAX]); 
void mark_rope_position(Point *tail, char result[STR_BUF_SIZE][STR_LEN_MAX]);

int main() {
  char result[STR_BUF_SIZE][STR_LEN_MAX] = {'\0'};

  Point head = {.y = 0, .x = 0};
  Point tail = {.y = 0, .x = 0};

  mark_rope_position(&tail, result);

  char direction_cmd;
  int steps_cmd;

  FILE *fp = fopen("./input.txt", "r");
  if (fp == NULL) {
    perror("Error opening file");
    return EXIT_FAILURE;
  }

  // PART 1
  while (fscanf(fp, "%c %d\n", &direction_cmd, &steps_cmd) != EOF) {
    for (int step = 0; step < steps_cmd; step++) {
      move_rope_by_cmd(&head, direction_cmd);
      follow_rope(&head, &tail, 1, result);
    }
  }

  printf("Result of part 1: %d\n", count_rope_position(result));

  // Reset for Part 2
  rewind(fp); // reset file pointer to beginning of the file for re-reading
  memset(result, '\0', sizeof(result)); // clear result for fresh start
  head = (Point){.y = 0, .x = 0}; // clear head
  Point tails[9] = {0};

  // PART 2
  while (fscanf(fp, "%c %d\n", &direction_cmd, &steps_cmd) != EOF) {
    for (int step = 0; step < steps_cmd; step++) {
      move_rope_by_cmd(&head, direction_cmd);
      follow_rope(&head, &tails[0], 0, result);
      follow_rope(&tails[0], &tails[1], 0, result);
      follow_rope(&tails[1], &tails[2], 0, result);
      follow_rope(&tails[2], &tails[3], 0, result);
      follow_rope(&tails[3], &tails[4], 0, result);
      follow_rope(&tails[4], &tails[5], 0,  result);
      follow_rope(&tails[5], &tails[6], 0, result);
      follow_rope(&tails[6], &tails[7], 0, result);
      follow_rope(&tails[7], &tails[8], 1, result);
    }
  }

  printf("Result of part 2: %d\n", count_rope_position(result));
  fclose(fp);
  return 0;
}

void follow_rope(Point *followed, Point *follower, int is_tracked, char bucket[STR_BUF_SIZE][STR_LEN_MAX]) {
  int dy = abs(followed->y - follower->y);
  int dx = abs(followed->x - follower->x);

  if (followed->x == follower->x && dy == 2) {
    // bergerak vertikal
    follower->y += (followed->y > follower->y) ? 1 : -1;
    if (is_tracked == 1) mark_rope_position(follower, bucket);
  } else if (followed->y == follower->y && dx == 2) {
    // bergerak horizontal
    follower->x += (followed->x > follower->x) ? 1 : -1;
    if (is_tracked == 1) mark_rope_position(follower, bucket);
  } else if ((dy == 2 || dx == 2)) {
    // bergerak diagonal
    follower->y += (followed->y > follower->y) ? 1 : -1;
    follower->x += (followed->x > follower->x) ? 1 : -1;
    if (is_tracked == 1) mark_rope_position(follower, bucket);
  }
}

void move_rope_by_cmd(Point *head, char direction) {
  switch (direction) {
  case 'R':
    head->x += 1;
    break;
  case 'L':
    head->x -= 1;
    break;
  case 'U':
    head->y -= 1;
    break;
  case 'D':
    head->y += 1;
    break;
  default:
    fprintf(stderr, "ERROR: Invalid direction!\n");
    exit(EXIT_FAILURE);
  }
}

void mark_rope_position(Point *tail, char result[STR_BUF_SIZE][STR_LEN_MAX]) {
  char str_buf[STR_LEN_MAX];
  snprintf(str_buf, STR_LEN_MAX, "%d%d", tail->y, tail->x);
  // Check apakah tail_str sudah ada di result
  for (int i = 0; i < STR_BUF_SIZE; i++) {
    if (result[i][0] != '\0' && strcmp(&str_buf, result[i]) == 0) {
      return;
    }
  }

  // Tambahkan str_buf ke result jika belum ada
  for (int i = 0; i < STR_BUF_SIZE; i++) {
    if (result[i][0] == '\0') {
      strncpy(result[i], &str_buf, STR_LEN_MAX - 1);
      result[i][STR_LEN_MAX - 1] = '\0'; // Pastikan null-terminated
      return; // Hentikan loop setelah menambahkan string
    }
  }
}

int count_rope_position(char result[STR_BUF_SIZE][STR_LEN_MAX]) {
  int count = 0;
  for (int i = 0; i < STR_BUF_SIZE; i++) {
    if (result[i][0] != '\0') {
      count++;
    }
  }
  return count;
}