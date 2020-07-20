#!/usr/bin/env python3

import glob
import json
import os
import shutil
import subprocess
import sys
import textwrap

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

def test(cmd, config):
    result = subprocess.run(cmd, encoding='utf8', input=config.get('stdin'), timeout=config.get('timeout', 5), capture_output=True)
    assert_result(result, config)

def test_deno(filepath, config):
    cmd = ['deno', 'run']

    cmd.append('--quiet')
    cmd.append('--allow-all')
    cmd.append('--unstable')

    with open('.deno.ts', 'w') as f:
        f.write(textwrap.dedent('''
          import WASI from "https://deno.land/std/wasi/snapshot_preview1.ts";

          const config = JSON.parse(Deno.args[0]);
          const buffer = Deno.readFileSync(Deno.args[1]);

          const wasi = new WASI({
            env: config.env,
            args: [Deno.args[1], ...config.args],
            preopens: config.preopens,
          });

          WebAssembly.instantiate(buffer, {
            wasi_snapshot_preview1: wasi.exports,
          }).then(function({ instance }) {
              wasi.memory = instance.exports.memory;
              instance.exports._start();
          });
        '''))

    cmd.append('.deno.ts')

    if config.get('env') == None:
        config['env'] = {}

    if config.get('args') == None:
        config['args'] = []

    cmd.append(json.dumps(config))
    cmd.append(filepath)

    test(cmd, config)

def test_node(filepath, config):
    cmd = ['node']

    cmd.append('--no-warnings')
    cmd.append('--experimental-wasi-unstable-preview1')
    cmd.append('--experimental-wasm-bigint')

    with open('.node.js', 'w') as f:
        f.write(textwrap.dedent('''
          const fs = require("fs");
          const { WASI } = require("wasi");

          const config = JSON.parse(process.argv[2]);
          const buffer = fs.readFileSync(process.argv[3]);

          const wasi = new WASI({
            env: config.env,
            args: [process.argv[3], ...config.args],
            preopens: config.preopens,
          });

          WebAssembly.instantiate(buffer, {
            wasi_snapshot_preview1: wasi.wasiImport,
          }).then(function({ instance }) {
              wasi.start(instance);
          });
        '''))

        cmd.append('.node.js')

    if config.get('env') == None:
        config['env'] = {}

    if config.get('args') == None:
        config['args'] = []

    cmd.append(json.dumps(config))
    cmd.append(filepath)

    test(cmd, config)

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

    test(cmd, config)

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

    test(cmd, config)

def main():
    inputs = []
    inputs.extend(sorted(glob.glob("target/wasm32-wasi/**/*.wasm")))

    tests = {
            "deno": test_deno,
            "node": test_node,
            "wasmer": test_wasmer,
            "wasmtime": test_wasmtime,
    }

    for filepath in inputs:
        filename, ext = os.path.splitext(filepath)
        dirname = os.path.dirname(filepath)
        basename = os.path.basename(filename)

        pattern = os.path.join(dirname, '**', '*', basename + '.json')
        matches = glob.glob(pattern, recursive=True)
        config = load_config(matches[0])

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
            except Exception as err:
                sys.stdout.write('\033[91mFAILED\x1b[0m')
            finally:
                sys.stdout.write('\n')

if __name__ == '__main__':
    main()

