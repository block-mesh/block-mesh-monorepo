import path from 'path'
import { spawn } from 'child_process'
import { Solita } from '@metaplex-foundation/solita'
import { writeFile, readFile } from 'fs/promises'

const PROGRAM_NAME = 'merkle_distributor'
const PROGRAM_ID = 'AZMc26abaSP7si1wtLaV5yPxTxpWd895M8YpJFFdQ8Qw'
const programDir = path.join('.', 'programs', 'merkle-distributor')
const generatedIdlDir = path.join('.', 'target', 'idl')
const generatedSDKDir = path.join('.', 'tests', 'generated')

// @ts-ignore
const anchor = spawn('anchor', ['build', '--idl', generatedIdlDir], {
  cwd: programDir
})
  .on('error', (err) => {
    console.error(err)
    // @ts-ignore this err does have a code
    if (err.code === 'ENOENT') {
      console.error(
        'Ensure that `anchor` is installed and in your path, see:\n  https://project-serum.github.io/anchor/getting-started/installation.html#install-anchor\n'
      )
    }
    process.exit(1)
  })
  .on('exit', () => {
    console.log(
      'IDL written to: %s',
      path.join(generatedIdlDir, `${PROGRAM_NAME}.json`)
    )
    generateTypeScriptSDK()
  })

anchor.stdout.on('data', (buf) => console.log(buf.toString('utf8')))
anchor.stderr.on('data', (buf) => console.error(buf.toString('utf8')))

async function generateTypeScriptSDK() {
  console.error('Generating TypeScript SDK to %s', generatedSDKDir)
  const generatedIdlPath = path.join(generatedIdlDir, `${PROGRAM_NAME}.json`)
  const data = await readFile(generatedIdlPath, 'utf8')
  const idl = JSON.parse(data)
  if (idl.metadata?.address == null) {
    idl.metadata = { ...idl.metadata, address: PROGRAM_ID }
    await writeFile(generatedIdlPath, JSON.stringify(idl, null, 2))
  }
  const gen = new Solita(idl, { formatCode: true })
  await gen.renderAndWriteTo(generatedSDKDir)

  console.error('Success!')

  process.exit(0)
}
