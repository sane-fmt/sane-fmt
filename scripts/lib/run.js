const { spawnSync } = require('child_process')
const { createPrettyExec } = require('pretty-exec')
const process = require('process')

const prettyExec = createPrettyExec({
  spawn: (command, args) => spawnSync(command, args, { stdio: 'inherit' }),
  print: console.log,
})

/**
 * Execute a command
 * @param {string} command Command to execute.
 * @param  {...string} args Arguments to pass to command.
 * @returns {void}
 */
function run(command, ...args) {
  const { error, status } = prettyExec(command, args)

  if (error) {
    console.error('Unexpected error', [command, ...args])
    console.error(error)
    return process.exit(1)
  }

  if (status !== 0) {
    console.error(
      `Command [${[command, ...args].join(', ')}] exited with code ${status}`,
    )
    return process.exit(1)
  }
}

module.exports = run
