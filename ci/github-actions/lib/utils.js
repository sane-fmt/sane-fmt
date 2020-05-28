const { env, exit } = require('process')
const { setFailed } = require('@actions/core')

/**
 * Get a mandatory environment variable.
 *
 * If the variable does not exist or exists but empty, emit a github action error and exit.
 *
 * @param {string} name Name of the environment variable.
 */
function requireEnv(name) {
  const value = env[name]
  if (!value) {
    setFailed(`Environment variable ${name} is required but missing`)
    throw exit()
  }
  return value
}

exports.requireEnv = requireEnv

Object.defineProperty(exports, '__esModule', { value: true })
