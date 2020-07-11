#!/usr/bin/env python3

import glob
import json
import os
import shutil
import subprocess
import sys

def assert_result(actual, expected):
    assert(actual.stdout == expected.get('stdout', ''))
    assert(actual.stderr == expected.get('stderr', ''))
    assert(actual.returncode == expected.get('exitCode', 0))

def ensure_empty_dir(path):
    if os.path.exists(path):
        shutil.rmtree(path)

    os.mkdir(path)

def load_config(filepath):
    config = None
    with open(filepath) as f:
        config = json.load(f)

    return config

def test_wasmer(filepath, config):
    cmd = ['wasmer', 'run']
    cmd.append(filepath)

    env = config.get('env')
    if env != None:
        for key in env:
            cmd.append('--env')
            cmd.append(key + '=' + env[key])

    preopens = config.get('preopens')
    if preopens != None:
        for path in preopens:
            cmd.append('--mapdir')
            cmd.append(path + ':' + preopens[path])


    args = config.get('args')
    if args != None:
        cmd.append('--')

        for arg in args:
            cmd.append(arg)

    result = subprocess.run(cmd, encoding='utf8', input=config.get('stdin'), capture_output=True)
    assert_result(result, config)

def test_wasmtime(filepath, config):
    cmd = ['wasmtime', 'run']

    env = config.get('env')
    if env != None:
        for key in env:
            cmd.append('--env')
            cmd.append(key + '=' + env[key])

    preopens = config.get('preopens')
    if preopens != None:
        for path in preopens:
            cmd.append('--mapdir')
            cmd.append(path + '::' + preopens[path])

    cmd.append(filepath)

    args = config.get('args')
    if args != None:
        cmd.append('--')

        for arg in args:
            cmd.append(arg)

    result = subprocess.run(cmd, encoding='utf8', input=config.get('stdin'), capture_output=True)
    assert_result(result, config)

def main():
    inputs = []
    inputs.extend(glob.glob("build/**/*.wasm"))

    tests = {
            "wasmer": test_wasmer,
            "wasmtime": test_wasmtime,
    }

    for filepath in inputs:
        basename, ext = os.path.splitext(filepath)
        config = load_config(basename + '.json')

        sys.stdout.write('test ')
        sys.stdout.write(filepath)
        sys.stdout.write(' ... ')
        sys.stdout.write('\n')

        for name in tests:
            ensure_empty_dir('scratch')
            sys.stdout.write('  ')
            sys.stdout.write(name)
            sys.stdout.write(' ... ')

            try:
                tests[name](filepath, config)
                sys.stdout.write('\033[92mok\x1b[0m')
            except AssertionError as err:
                sys.stdout.write('\033[91mFAILED\x1b[0m')
            finally:
                sys.stdout.write('\n')

if __name__ == '__main__':
    main()

