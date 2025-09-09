const path = require('path');
const fs = require('fs');

const version = process.env.MV_VERSION?.replace('v', '');
const target = process.env.TARGET?.toLowerCase(); // 'desktop' or 'web'

if (!version) {
  throw new Error('MV_VERSION environment variable not set');
}

if (!target || !['desktop', 'web'].includes(target)) {
  throw new Error(
    'TARGET environment variable must be set to "desktop" or "web"'
  );
}

const tauriConfigPath = path.join(__dirname, '../src-tauri/tauri.conf.json');
const packageJsonPath = path.join(__dirname, '../package.json');

if (target === 'desktop') {
  const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf8'));
  tauriConfig.version = version;

  console.log(`Writing version ${version} to ${tauriConfigPath}`);
  fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2));
} else if (target === 'web') {
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  packageJson.version = version;

  console.log(`Writing version ${version} to ${packageJsonPath}`);
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));
}

console.log(`Version update for ${target} build complete.`);
