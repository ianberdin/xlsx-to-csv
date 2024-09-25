import {join} from 'path';
import {spawnSync} from 'node:child_process';
import * as fs from 'node:fs';
import * as os from "node:os";

const NAME = 'xlsx-to-csv';

function getPlatform() {
    const type = os.type();
    const arch = os.arch();

    if (type === 'Windows_NT' && arch === 'x64') return 'win64';
    if (type === 'Windows_NT') return 'win32';
    if (type === 'Linux' && arch === 'x64') return 'linux';
    if (type === 'Darwin' && arch === 'x64') return 'macos-x86_64';
    if (type === 'Darwin' && arch === 'arm64') return 'macos-aarch64';

    throw new Error(`Unsupported platform: ${type} ${arch}`);
}

export function getBinary() {
    const platform = getPlatform();
    const binaryName = `${NAME}-${platform}`;

    const binaryPath = join(__dirname, binaryName);

    // Check if the binary exists in the filesystem
    if (!fs.existsSync(binaryPath)) {
        throw new Error(`Binary not found: ${binaryPath}`);
    }

    return binaryPath;
}

export function run(...args) {
    const result = spawnSync(
        getBinary(),
        args, {},
    );

    if (result.error) {
        throw result.error;
    }

    return result;
}
