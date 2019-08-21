from distutils.core import setup, Extension

module1 = Extension(
    '_solveboggle',
    include_dirs = ['/usr/local/include', '../src'],
    libraries = ['boggle_solver'],
    library_dirs = ['../target/release'],
    sources = ['boggle_solve.c'])

setup (
    name = 'BoggleSolver',
    version = '0.2',
    description = 'This is a Rust to Python demonstration',
    author = 'Elf M. Sternberg',
    author_email = 'elf.sternberg@gmail.com',
    url = 'https://elfsternberg.com',
    long_description = '''
    This is really just a demo package.
    ''',
    ext_modules = [module1])
