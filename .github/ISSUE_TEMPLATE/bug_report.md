---
name: Bug report
about: Report a reproducible problem with RocmTop
title: "[bug] "
labels: [bug]
assignees: []
---

## Summary

<!-- A clear, concise description of what went wrong. -->

## Steps to reproduce

1. 
2. 
3. 

## Expected behaviour

<!-- What you thought should happen. -->

## Actual behaviour

<!-- What actually happened. Include screenshots if the bug is visual. -->

## Environment

- **RocmTop version**: <!-- visible in the window footer, e.g. v1.0.0 -->
- **Distro / kernel**: <!-- output of `uname -r` and `cat /etc/os-release | head -2` -->
- **GPU**: <!-- output of `lspci -nn | grep -Ei 'vga|3d'` -->
- **WebKitGTK version**: <!-- `pkg-config --modversion webkit2gtk-4.1` -->
- **libayatana-appindicator installed?** yes / no

## Sysfs dump (when applicable)

<details>
<summary>ls -la /sys/class/drm/card*/device/</summary>

```
<!-- paste output here -->
```

</details>

## Logs

<!--
Run RocmTop from a terminal (`./RocmTop-x86_64.AppImage` or
`npm run tauri dev`) and paste anything printed to stderr.
-->

```
```
