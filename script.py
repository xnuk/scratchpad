#!/usr/bin/env python3

import sys
from os import mkdir
from tempfile import TemporaryDirectory
from pathlib import PurePath
from textwrap import dedent
from typing import Callable

major, minor = sys.version_info[:2]
if not (major == 3 and minor >= 10):
	print(
	    'Your python version is sucks: %d.%d',
	    (major, minor),
	    file=sys.stderr,
	)
	exit(1)

sources = {
    'my.py':
    'def underwear(): return 42',
    'run.py':
    dedent('''\
	from .my import underwear
	if __name__ == '__main__': print(underwear())
	''')
}


def python3(
    *args: str | bytes | PurePath,
    cwd: PurePath,
    env: dict[str, str] | None = None,
) -> str:
	from subprocess import run, PIPE
	process = run(
	    ['python3', *args],
	    cwd=cwd,
	    capture_output=True,
	    text=True,
	    env=env,
	    shell=False,
	)

	stdout = process.stdout.strip()
	stderr = process.stderr.strip()

	successful = stdout == '42' and process.returncode == 0

	if successful: return ''
	return stderr


def make_sources(dir: PurePath, sources: dict[str, str]):
	for name, text in sources.items():
		with open(dir / name, mode='w') as f:
			f.write(text)


def with_tempdir(func: Callable[[PurePath], str]) -> str:
	with TemporaryDirectory() as name:
		root = PurePath(name)
		return func(root)


###################
###### TESTS ######
###################


# Fail
def test_inside(root: PurePath) -> str:
	""" python run.py """
	make_sources(root, sources)
	return python3(root / 'run.py', cwd=root)


# Fail
def test_inside_init(root: PurePath) -> str:
	''' python run.py with __init__.py '''
	make_sources(root, sources | {'__init__.py': ''})
	return python3(root / 'run.py', cwd=root)


# Fail
def test_outside(root: PurePath) -> str:
	''' python src/run.py '''
	src = root / 'src'
	mkdir(src)
	make_sources(src, sources)
	return python3(src / 'run.py', cwd=root)


# Fail
def test_outside_init(root: PurePath) -> str:
	''' python src/run.py with __init__.py '''
	src = root / 'src'
	mkdir(src)
	make_sources(src, sources | {'__init__.py': ''})
	return python3(src / 'run.py', cwd=root)


# Fail
def test_inside_module(root: PurePath) -> str:
	''' python -m run '''
	make_sources(root, sources)
	return python3('-m', 'run', cwd=root)


# Success
def test_outside_module(root: PurePath) -> str:
	''' python -m src.run '''
	src = root / 'src'
	mkdir(src)
	make_sources(src, sources)
	return python3('-m', 'src.run', cwd=root)


# Fail
def test_outside_module_slash(root: PurePath) -> str:
	''' python -m src/run.py '''
	src = root / 'src'
	mkdir(src)
	make_sources(src, sources)
	return python3('-m', src / 'run.py', cwd=root)


def test_outside_module_weird_folder_name(root: PurePath) -> str:
	''' python -m "$src_name.run" '''
	src_name = '모든 미션 완료 시, 하단의 [라이센스 획득] 버튼을 눌러주세요.'
	src = root / src_name
	mkdir(src)
	make_sources(src, sources)
	return python3('-m', src_name + '.run', cwd=root)


def main():
	global_things = globals().items()
	tests = filter(lambda x: x[0].startswith('test_'), global_things)

	for name, test in tests:
		stderr = with_tempdir(test)
		status = 'Success' if stderr == '' else 'Fail'
		print('[%s] %s (%s)' % (status, test.__name__, test.__doc__.strip()))


if __name__ == '__main__': main()
