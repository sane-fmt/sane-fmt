const { spawnSync } = require('child_process')
const process = require('process')

/**
 * Execute a command
 * @param {string} command Command to execute.
 * @param  {...string} args Arguments to pass to command.
 * @returns {void}
 */
function run(command, ...args) {
  const { error, status } = spawnSync(command, args, { stdio: 'inherit' })

  if (error) {
    console.error('Unexpected error', [command, ...args])
    console.error(error)
    return process.exit(1)
  }

  if (status !== 0) {
    console.error(`Command [${[command, ...args].join(', ')}] exited with code ${status}`)
    return process.exit(1)
  }
}

module.exports = run
