'use strict'

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let loadError = null

function isMusl() {
  try {
    return readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
  } catch {
    return true
  }
}

const platformTriple = (() => {
  if (platform === 'linux') {
    if (arch === 'x64') return isMusl() ? 'linux-x64-musl' : 'linux-x64-gnu'
    if (arch === 'arm64') return 'linux-arm64-gnu'
  }
  if (platform === 'darwin') {
    if (arch === 'x64') return 'darwin-x64'
    if (arch === 'arm64') return 'darwin-arm64'
  }
  if (platform === 'win32') {
    if (arch === 'x64') return 'win32-x64-msvc'
  }
  return null
})()

if (!platformTriple) {
  throw new Error(`asynmov: unsupported platform ${platform}-${arch}`)
}

// 1. Try the local .node file (dev builds, CI)
const localPath = join(__dirname, `asynmov.${platformTriple}.node`)
if (existsSync(localPath)) {
  try {
    nativeBinding = require(localPath)
  } catch (e) {
    loadError = e
  }
}

// 2. Fall back to the platform-specific npm package
if (!nativeBinding && !loadError) {
  try {
    nativeBinding = require(`@asynmov/asynmov-${platformTriple}`)
  } catch (e) {
    loadError = e
  }
}

if (!nativeBinding) {
  if (loadError) throw loadError
  throw new Error(`asynmov: failed to load native binding for ${platformTriple}`)
}

module.exports = nativeBinding
