#!/bin/sh
set -eu

registry=/tmp/dioxus-components
primitives=$(cargo pkgid dioxus-primitives)
rev=${primitives#*rev=}
rev=${rev%%#*}
icons=$(cargo pkgid dioxus-icons)
icons=${icons##*@}

rm -rf "$registry"
git clone --quiet https://github.com/DioxusLabs/components "$registry"
git -C "$registry" checkout --quiet -b starter "$rev"
python3 - "$registry" "$rev" "=$icons" <<'EOF'
import json, pathlib, sys

registry, rev, icons = sys.argv[1:]
pins = {'dioxus-primitives': {'rev': rev}, 'dioxus-icons': {'version': icons}}
for manifest in pathlib.Path(registry, 'preview/src/components').glob('*/component.json'):
    component = json.loads(manifest.read_text())
    for dependency in component.get('cargoDependencies', []):
        if isinstance(dependency, dict) and dependency['name'] in pins:
            dependency.update(pins[dependency['name']])
        elif isinstance(dependency, str) and dependency in pins:
            raise SystemExit(f'{manifest} names {dependency} without a version')
    if 'componentDependencies' in component:
        if not all(isinstance(dependency, str) for dependency in component['componentDependencies']):
            raise SystemExit(f'{manifest} depends on a component outside this registry')
        component['componentDependencies'] = [
            {'name': dependency, 'git': f'file://{registry}'} for dependency in component['componentDependencies']
        ]
    manifest.write_text(json.dumps(component, indent=2) + '\n')
EOF
git -C "$registry" -c user.name=starter -c user.email=starter@localhost commit --quiet --all \
  --message "Pin dependencies to the starter's versions"

dx components clean
dx components add --git "file://$registry" "$@"
