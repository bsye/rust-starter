#!/bin/sh
set -eu

upstream_url=https://github.com/DioxusLabs/components
ours_url=https://github.com/bsye/dioxus-registry
ours_rev=87a6d26ddaeb3ef4b788dafd44fdc3525d9498f1
upstream=/tmp/dioxus-components
ours=/tmp/dioxus-registry
primitives=$(cargo pkgid dioxus-primitives)
rev=${primitives#*rev=}
rev=${rev%%#*}
icons=$(cargo pkgid dioxus-icons)
icons=${icons##*@}

clone() {
  rm -rf "$2"
  git clone --quiet "$1" "$2"
  git -C "$2" checkout --quiet -b starter "$3"
}

clone "$upstream_url" "$upstream" "$rev"
clone "$ours_url" "$ours" "$ours_rev"
python3 - "$upstream_url" "$upstream" "$ours" "$rev" "=$icons" "$ours_rev" <<'EOF'
import json, pathlib, sys

upstream_url, upstream, ours, rev, icons, ours_rev = sys.argv[1:]
pins = {
    'dioxus-primitives': {'rev': rev},
    'dioxus-icons': {'version': icons},
    'dioxus-registry-primitives': {'rev': ours_rev},
}
for registry in (upstream, ours):
    for manifest in pathlib.Path(registry, 'preview/src/components').glob('*/component.json'):
        component = json.loads(manifest.read_text())
        for dependency in component.get('cargoDependencies', []):
            if isinstance(dependency, dict) and dependency['name'] in pins:
                dependency.update(pins[dependency['name']])
            elif isinstance(dependency, str) and dependency in pins:
                raise SystemExit(f'{manifest} names {dependency} without a version')
        resolved = []
        for dependency in component.get('componentDependencies', []):
            if isinstance(dependency, str):
                resolved.append({'name': dependency, 'git': f'file://{registry}'})
            elif dependency.get('git') == upstream_url:
                resolved.append({'name': dependency['name'], 'git': f'file://{upstream}'})
            else:
                raise SystemExit(f'{manifest} depends on a component outside both registries')
        if resolved:
            component['componentDependencies'] = resolved
        manifest.write_text(json.dumps(component, indent=2) + '\n')
EOF
for registry in "$upstream" "$ours"; do
  git -C "$registry" -c user.name=starter -c user.email=starter@localhost commit --quiet --all \
    --message "Pin dependencies to the starter's versions"
done

from_ours=""
from_upstream=""
for name in "$@"; do
  if grep -qF "\"preview/src/components/$name\"" "$ours/component.json"; then
    from_ours="$from_ours $name"
  else
    from_upstream="$from_upstream $name"
  fi
done

dx components clean
if [ -n "$from_upstream" ]; then
  dx components add --git "file://$upstream" $from_upstream
fi
if [ -n "$from_ours" ]; then
  dx components add --git "file://$ours" $from_ours
fi
