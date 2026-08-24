import assert from 'node:assert/strict'
import { readdirSync, readFileSync } from 'node:fs'
import test from 'node:test'

const root = new URL('..', import.meta.url)
const pluginRoot = new URL('../plugins/weavatrix-refactor/', import.meta.url)

const readText = (base, path) => readFileSync(new URL(path, base), 'utf8').replace(/^\uFEFF/, '')
const readJson = (base, path) => JSON.parse(readText(base, path))

const npmManifest = readJson(root, 'npm/weavatrix-refactor/package.json')
const serverManifest = readJson(root, 'server.json')
const cargoManifest = readText(root, 'Cargo.toml')
const contract = readJson(root, 'contract/refactor-tools.v1.json')
const version = npmManifest.version

const manifests = {
    codex: readJson(pluginRoot, '.codex-plugin/plugin.json'),
    cursor: readJson(pluginRoot, '.cursor-plugin/plugin.json'),
    claude: readJson(pluginRoot, '.claude-plugin/plugin.json'),
    grok: readJson(pluginRoot, '.grok-plugin/plugin.json'),
}

test('plugin identity follows the released Refactor host', () => {
    assert.equal(serverManifest.version, version)
    assert.match(cargoManifest, new RegExp(`^version = "${version.replaceAll('.', '\\.')}"$`, 'm'))
    for (const [client, manifest] of Object.entries(manifests)) {
        assert.equal(manifest.name, 'weavatrix-refactor', `${client} plugin name`)
        assert.equal(manifest.version, version, `${client} plugin version`)
        assert.equal(manifest.repository, 'https://github.com/sergii-ziborov/weavatrix-refactor')
        assert.equal(manifest.license, 'MIT')
    }
})

test('both MCP manifests launch only the Refactor extension surface', () => {
    for (const filename of ['.mcp.json', 'mcp.json']) {
        const config = readJson(pluginRoot, filename)
        const server = config.mcpServers?.['weavatrix-refactor']
        assert.ok(server, `${filename} defines weavatrix-refactor`)
        assert.equal(server.type, 'stdio')
        assert.equal(server.command, 'npx')
        assert.deepEqual(server.args, [
            '-y',
            `weavatrix-refactor@${version}`,
            'mcp',
            '.',
            '--profile=refactor',
        ])
        assert.equal(server.env.WEAVATRIX_ALLOW_SOURCE_EDITS, '1')
    }
})

test('all four client marketplaces resolve the bundled plugin', () => {
    const codex = readJson(root, '.agents/plugins/marketplace.json')
    const cursor = readJson(root, '.cursor-plugin/marketplace.json')
    const claude = readJson(root, '.claude-plugin/marketplace.json')
    const grok = readJson(root, '.grok-plugin/marketplace.json')

    assert.equal(codex.plugins[0].source.path, './plugins/weavatrix-refactor')
    assert.equal(cursor.plugins[0].source, 'plugins/weavatrix-refactor')
    assert.equal(cursor.plugins[0].version, version)
    assert.equal(claude.plugins[0].source, './plugins/weavatrix-refactor')
    assert.equal(claude.plugins[0].version, version)
    assert.equal(grok.plugins[0].source.path, './plugins/weavatrix-refactor')
    assert.equal(grok.plugins[0].version, version)
})

test('the optional skill has one compact card per Refactor method', () => {
    const names = contract.tools.map((tool) => tool.name).sort()
    const canonical = readdirSync(new URL('../skill/references/tools/', import.meta.url))
        .filter((name) => name.endsWith('.md'))
        .map((name) => name.slice(0, -3))
        .sort()
    const bundled = readdirSync(new URL('../plugins/weavatrix-refactor/skills/weavatrix-refactor/references/tools/', import.meta.url))
        .filter((name) => name.endsWith('.md'))
        .map((name) => name.slice(0, -3))
        .sort()

    assert.equal(names.length, 11)
    assert.deepEqual(canonical, names)
    assert.deepEqual(bundled, names)
    assert.equal(
        readText(root, 'skill/SKILL.md'),
        readText(pluginRoot, 'skills/weavatrix-refactor/SKILL.md'),
    )
    for (const name of names) {
        assert.equal(
            readText(root, `skill/references/tools/${name}.md`),
            readText(pluginRoot, `skills/weavatrix-refactor/references/tools/${name}.md`),
        )
    }
})
