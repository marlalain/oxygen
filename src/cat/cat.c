// cat made by @paulo-e, @pauloe
// paulo-e.gitlab.io

// TODO add --version
// TODO add --help

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

bool hasEnoughArgs(int argc) {
  if (argc == 1) {
    return false;
  } else {
    return true;
  }
}

bool isValid(FILE *filePointer) {
  if (filePointer == NULL) {
    return false;
  } else {
    return true;
  }
}

int main(int argc, char *argv[]) {
  bool enoughArgs = hasEnoughArgs(argc);
  char ch;

  if (enoughArgs) {
    FILE *filePointer;

    filePointer = fopen(argv[1], "r");
    if (!isValid(filePointer)) {
      perror("Error while opening the file.\n");
      exit(EXIT_FAILURE);
    }

    while ((ch = fgetc(filePointer)) != EOF) {
      printf("%c", ch);
    }

    fclose(filePointer);
    return 0;
  }

  return 1;
}
