import * as binary from './npm/get-binary'
import { changeExtension } from './utils'


export function convertFile( filePath: string, sheetName?: string ) {
  const args = [filePath]
  if ( sheetName ) {
    args.push(sheetName)
  }

  binary.run(...args)

  return { filePath: changeExtension(filePath, '.csv') }
}

export default {
  convertFile,
}