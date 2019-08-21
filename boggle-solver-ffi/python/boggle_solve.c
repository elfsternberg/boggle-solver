//Python.h has all the required function definitions to manipulate the Python objects
#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "boggle_solver.h"

static void del_Dictionary(PyObject * obj) 
{ 
  dictionary_destroy(PyCapsule_GetPointer(obj, "ScrabbleDictionary")); 
} 

//This is the function that is called from your python code
static PyObject* boggleSolve_make_dictionary(PyObject* self, PyObject* args) {
  
  char* path;
  
  //The input arguments come as a tuple, we parse the args to get the various variables
  //In this case it's only one list variable, which will now be referenced by listObj
  if (! PyArg_ParseTuple(args, "s", &path))
    return NULL;

  return PyCapsule_New(dictionary_make(path), "ScrabbleDictionary", del_Dictionary);
}

//This is the docstring that corresponds to our 'add' function.
static char boggleSolve_make_dictionary_docs[] =
  "make_dictionary( ): Given a path to a valid wordlist file, return a dictionary object suitable to reuse.\n";

static PyObject *boggleSolve_solve_board(PyObject *self, PyObject *args) {

  PyObject *dict;
  char buffer[8192];
  char* board;
  struct Trie* trie;
  
  if (! PyArg_ParseTuple(args, "sO", &board, &dict)) {
    return NULL;
  }

  if (! (trie = (struct Trie *) PyCapsule_GetPointer(dict, "ScrabbleDictionary"))) {
    return NULL;
  }

  solve_for_dictionary(board, trie, buffer);
  return Py_BuildValue("s", buffer);
}

static char boggleSolve_solve_board_docs[] =
  "solve_board(board, dictionary): Given a string containing a valid boggle board of rows separated by linefeeds, return a string of all words found separated by linefeeds.";


static PyMethodDef boggle_funcs[] =
  {
   {
     "make_dictionary",
     (PyCFunction) boggleSolve_make_dictionary,
     METH_VARARGS,
     boggleSolve_make_dictionary_docs
   },
   {
     "solve",
     (PyCFunction) boggleSolve_solve_board,
     METH_VARARGS,
     boggleSolve_solve_board_docs
   },
   {NULL, NULL, 0, NULL}
};

static struct PyModuleDef bogglemodule = 
{ 
 PyModuleDef_HEAD_INIT, 
 "_solveboggle",
 "A low-level Boggle board solver library",
 -1, /* Size of per-interpreter state or -1 */
 boggle_funcs
}; 
  
/* Module initialization function */
PyMODINIT_FUNC 
PyInit__solveboggle(void) 
{ 
  return PyModule_Create(&bogglemodule); 
} 

