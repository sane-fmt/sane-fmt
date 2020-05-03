import path from 'path'
import fs from 'fs'
import { WASI } from '@wasmer/wasi'
import bindings from '@wasmer/wasi/lib/bindings/node'
import { WasmFs } from '@wasmer/wasmfs'

export const wasmFile = require.resolve('./sane-fmt.wasm')

export type WriteStream = NodeJS.WriteStream

export interface Process {
  readonly argv: readonly string[]
  readonly cwd: () => string
  readonly stdout: WriteStream
  readonly stderr: WriteStream
  readonly exit: (status: number) => never
}

export async function main(process: Process): Promise<void> {
  const args = process.argv.slice(2)
  const wdir = process.cwd()
  const wasmFs = new WasmFs()

  const wasi = new WASI({
    args,
    preopenDirectories: {
      '/': path.parse(wdir).root,
      '.': wdir,
    },
    bindings: {
      ...bindings,
      fs: wasmFs,
      exit: status => process.exit(status)
    },
  })

  function pipeFile (path: string, stream: WriteStream) {
    const reader = wasmFs.fs.createReadStream(path)
    reader.on('open', () => reader.pipe(stream))
  }

  pipeFile('/dev/stdout', process.stdout)
  pipeFile('/dev/stderr', process.stderr)

  const wasmBytes = fs.readFileSync(wasmFile)
  const wasmModule = await WebAssembly.compile(wasmBytes)
  const wasmInstance = await WebAssembly.instantiate(wasmModule, {
    ...wasi.getImports(wasmModule)
  })

  wasi.start(wasmInstance)
}
