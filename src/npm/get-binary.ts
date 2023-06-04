import { Binary } from 'binary-install'
import os from 'os'
import { spawnSync } from 'child_process'


const NAME = 'xlsx-to-csv'
const VERSION = '1.0.2'

function getPlatform() {
  const type = os.type()
  const arch = os.arch()

  if ( type === 'Windows_NT' && arch === 'x64' ) return 'win64'
  if ( type === 'Windows_NT' ) return 'win32'
  if ( type === 'Linux' && arch === 'x64' ) return 'linux'
  if ( type === 'Darwin' && arch === 'x64' ) return 'macos-x86_64'
  if ( type === 'Darwin' && arch === 'arm64' ) return 'macos-aarch64'

  throw new Error(`Unsupported platform: ${ type } ${ arch }`)
}

export function getBinary() {
  const platform = getPlatform()
  const url = `https://github.com/ianberdin/xlsx-to-csv/releases/download/v${ VERSION }/${ NAME }-${ platform }.tar.gz`
  return new Binary(NAME, url)
}

export function run( ...args ) {
  const result = spawnSync(
    getBinary().binaryPath,
    args, {},
  )

  if ( result.error ) {
    throw result.error
  }

  return result
}

export default {
  getPlatform,
  getBinary,
  run,
}