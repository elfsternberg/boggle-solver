#ifndef _BOGGLE_SOLVER_DEF_
#define _BOGGLE_SOLVER_DEF_
#endif

#include <stdint.h>
#include <stdlib.h>

struct Trie;

struct Trie* dictionary_make(char* filepath);
void dictionary_destroy(struct Trie* trie);

// It's your responsibility to ensure, that buffer size is long enough to hold the answer set.

// Warning: the highest-scoring board known looks like this:
//
// S E R S
// P A T G
// L I N E
// S E R S
//
// And the result set returned for the Linux /usr/dict/words collection
// is 4,604 bytes long.
//
// Plan accordingly.

void solve_for_dictionary(char* board_text, struct Trie* dictionary, char* buffer);
void solve(char* board_test, char* dictionary_filepath, char* buffer);
