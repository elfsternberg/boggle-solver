import re
from collections import defaultdict

class Node:
    def __init__(self):
        self.word = False

        # By using defaultdict here, every time I ASK for a letter
        # that doesn't already have a suffix handler, the suffix
        # handler is automagically created.
        self.suff = defaultdict(Node)

    # If the word is exhausted, then this suffix handler marks the end
    # of a valid word.  Otherwise, keep going, recursing down the
    # system.  This where that defaultdict magic works: we know the
    # letter (car word) is part of a valid word, so defaultdict
    # automagically creates the node if it doesn't already exist, then
    # recurses down into it with the (cdr word).
    def insert(self, word):
        if len(word) == 0:
            self.word = True
            return
        self.suff[word[0]].insert(word[1:])

    # A word exists if, when the sample word is exhausted, we're
    # at a node where we previously marked a terminator.
    def find(self, word):
        if len(word) == 0:
            return self.word
        return word[0] in self.suff and self.suff[word[0]].find(word[1:])

    # A word is a prefix of another word if the same word is
    # successfully exhausted.
    def pref(self, word):
        if len(word) == 0:
            return True
        return word[0] in self.suff and self.suff[word[0]].pref(word[1:])

    
class Scanned:
    def __init__(self, word = "", positions = []):
        self.positions = positions
        self.word = word

    def add(self, c, i, j):
        if (i, j) in self.positions:
            return None
        return Scanned(self.word + c, self.positions + [(i, j)])


class Board:
    def __init__(self, board, words):
        self.board = board
        self.words = words
        self.mx = len(board)
        self.my = len(board[0])
        self.solutions = []

    def solveforpos(self, posx, posy, cur):
        cur = cur.add(self.board[posx][posy], posx, posy)
        if cur is None:
            return

        if len(cur.word) > 2 and self.words.find(cur.word):
            self.solutions.append(cur.word)

        if not self.words.pref(cur.word):
            return

        for x in [-1, 0, 1]:
            for y in [-1, 0, 1]:
                nx = posx + x
                ny = posy + y
                if nx >= 0 and nx < self.mx and ny >= 0 and ny < self.my and not (x == 0 and x == y):
                    self.solveforpos(nx, ny, cur)

    def solve(self):
        self.solutions = []
        for x in xrange(0, self.mx):
            for y in xrange(0, self.my):
                self.solveforpos(x, y, Scanned())
        return sorted(list(set(self.solutions)))

isword = re.compile(r'^[a-z][a-z][a-z][a-z]*$')
words = Node()
for word in [i[0:len(i)-1] for i in open("/usr/share/dict/words").readlines() if isword.match(i)]:
    words.insert(word)

demo = ['mapo',
        'eter',
        'deni',
        'ldhc']
demo = [[i for i in r] for r in demo]
print(Board(demo, words).solve())

