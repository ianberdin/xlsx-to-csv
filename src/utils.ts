import path from 'path'


export function changeExtension( filePath, extension: string ) {
  const basename = path.basename(filePath, path.extname(filePath))
  return path.join(path.dirname(filePath), basename + extension)
}