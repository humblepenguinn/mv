import chalk from 'chalk';
import fs from 'fs-extra';
import path from 'path';
import { PluginOption } from 'vite';

const packageName = 'vite-plugin-wasm-pack';

interface CrateOptions {
  path: string;
  outName?: string;
}

interface VitePluginWasmPackOptions {
  crates: CrateOptions[];
  runPlugin: boolean;
}

function vitePluginWasmPack(options: VitePluginWasmPackOptions): PluginOption {
  const prefix = '@vite-plugin-wasm-pack@';
  const pkgDir = 'pkg';
  let configBase: string;
  let configAssetsDir: string;

  const cratePaths = options.crates;

  const wasmFilename = (crate: CrateOptions) =>
    (crate.outName || path.basename(crate.path)).replace(/-/g, '_') +
    '_bg.wasm';

  type CrateType = { path: string; isNodeModule: boolean };
  const wasmMap = new Map<string, CrateType>();

  cratePaths.forEach((crate) => {
    const wasmFile = wasmFilename(crate);
    wasmMap.set(wasmFile, {
      path: path.join(crate.path, pkgDir, wasmFile),
      isNodeModule: false,
    });
  });

  if (!options.runPlugin) {
    console.log(
      chalk.bold.yellow(
        `[${packageName}] Creating dummy wasm pack so vite stops complaining`
      )
    );

    cratePaths.forEach((crate) => {
      const crateName = crate.outName || path.basename(crate.path);
      const jsName = crateName.replace(/-/g, '_') + '.js';
      const dummyPath = path.join('node_modules', crateName);

      if (!fs.existsSync(dummyPath)) {
        fs.ensureDirSync(dummyPath);

        fs.writeFileSync(
          path.join(dummyPath, jsName),
          '// dummy wasm package so vite stops complaining'
        );

        const dtsName = jsName.replace(/\.js$/, '.d.ts');
        fs.writeFileSync(
          path.join(dummyPath, dtsName),
          `declare module '${crateName}' {}`
        );
      } else {
        console.log(
          chalk.bold.yellow(
            `[${packageName}] wasm package already exists at ${dummyPath}`
          )
        );
      }
    });

    return {
      name: 'vite-plugin-wasm-pack',
    };
  }

  return {
    name: 'vite-plugin-wasm-pack',
    enforce: 'pre',
    configResolved(resolvedConfig) {
      configBase = resolvedConfig.base;
      configAssetsDir = resolvedConfig.build.assetsDir;
    },

    resolveId(id: string) {
      for (const crate of cratePaths) {
        const crateName = crate.outName || path.basename(crate.path);
        if (crateName === id) return prefix + id;
      }
      return null;
    },

    async load(id: string) {
      if (id.startsWith(prefix)) {
        id = id.replace(prefix, '');
        const crate = cratePaths.find(
          (c) => (c.outName || path.basename(c.path)) === id
        );
        if (!crate) return null;
        const moduleJs = path.join(
          './node_modules',
          crate.outName || path.basename(crate.path),
          (crate.outName || path.basename(crate.path)).replace(/-/g, '_') +
            '.js'
        );
        return await fs.readFile(moduleJs, 'utf-8');
      }
    },

    async buildStart() {
      const prepareBuild = async (
        crate: CrateOptions,
        isNodeModule: boolean
      ) => {
        const crateName = crate.outName || path.basename(crate.path);
        const pkgPath = isNodeModule
          ? path.dirname(require.resolve(crate.path))
          : path.join(crate.path, pkgDir);

        if (!fs.existsSync(pkgPath)) {
          const msg = isNodeModule
            ? `Can't find ${pkgPath}, run npm install ${crate.path} first`
            : `Can't find ${pkgPath}, run wasm-pack build ${crate.path} --target web first`;
          console.error(
            chalk.bold.red(`[${packageName}] Error: `) + chalk.bold(msg)
          );
        }

        if (!isNodeModule) {
          try {
            await fs.copy(pkgPath, path.join('node_modules', crateName));
          } catch (error) {
            this.error(`copy crates failed: ${error}`);
          }
        }

        const jsName = crateName.replace(/-/g, '_') + '.js';
        const jsPath = isNodeModule
          ? path.join(pkgPath, jsName)
          : path.join('./node_modules', crateName, jsName);

        let code = fs.readFileSync(path.resolve(jsPath), 'utf-8');
        code = code.replace(/input = new URL\('(.+)'.+;/g, (_match, group1) => {
          return `input = "${path.posix.join(configBase, configAssetsDir, group1)}"`;
        });
        fs.writeFileSync(jsPath, code);
      };

      for (const crate of cratePaths) await prepareBuild(crate, false);
    },

    configureServer({ middlewares }) {
      middlewares.use((req, res, next) => {
        if (req.url) {
          const basename = path.basename(req.url);
          res.setHeader('Cache-Control', 'no-cache, no-store, must-revalidate');
          const entry = wasmMap.get(basename);
          if (basename.endsWith('.wasm') && entry) {
            res.writeHead(200, { 'Content-Type': 'application/wasm' });
            fs.createReadStream(entry.path).pipe(res);
          } else {
            next();
          }
        }
      });
    },

    buildEnd() {
      wasmMap.forEach((crate, fileName) => {
        this.emitFile({
          type: 'asset',
          fileName: `assets/${fileName}`,
          source: fs.readFileSync(crate.path),
        });
      });
    },
  };
}

export default vitePluginWasmPack;

if (typeof module !== 'undefined') {
  module.exports = vitePluginWasmPack;
  vitePluginWasmPack.default = vitePluginWasmPack;
}
