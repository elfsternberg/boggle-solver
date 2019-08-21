#include "boggle_solver.h"
#include <stdio.h>
#include <string.h>

#define MAXBOARDSIZE 64
#define MAXRETURNSIZE 8192

int main (int argc, char** argv) {
  char board[MAXBOARDSIZE];
  char buffer[MAXRETURNSIZE];
  FILE *fp;
  
  if (argc < 2) {
    fputs("ERROR: No board was specified.\n", stderr);
    exit(-1);
  }

  fp = fopen(argv[1], "r");
  if (fp == NULL) {
    fputs("ERROR: Could not find board file as specified.\n", stderr);
    exit(-1);
  }

  size_t len = fread(board, sizeof(char), MAXBOARDSIZE, fp);
  if ( ferror( fp ) != 0 ) {
    fputs("ERROR: Could not read file.", stderr);
    exit(-1);
  } else {
    board[len++] = '\0'; /* Just to be safe. */
  }

  fclose(fp);

  solve(board, "/usr/share/dict/words", buffer);
  printf("%s\n", buffer);
  return 0;
}
