# winget

Manifests for [winget-pkgs](https://github.com/microsoft/winget-pkgs), package `AtunFrito.TunaFonts`.

## First version (by hand, once)

After the release is **published** on GitHub:

```powershell
./winget/preparar.ps1 0.2.0
winget validate --manifest winget/manifests/a/AtunFrito/TunaFonts/0.2.0
winget install --manifest winget/manifests/a/AtunFrito/TunaFonts/0.2.0   # optional: try it
```

Then submit it, either with [wingetcreate](https://github.com/microsoft/winget-create):

```powershell
wingetcreate submit winget/manifests/a/AtunFrito/TunaFonts/0.2.0
```

or by opening a pull request to `microsoft/winget-pkgs` that adds that folder at `manifests/a/AtunFrito/TunaFonts/0.2.0/`. A Microsoft moderator reviews it; it usually takes a few days.

`winget/manifests/` is generated and not committed.

## Next versions (automatic)

Once the package exists in winget, `.github/workflows/winget.yml` sends each new version when its release is published, using [komac](https://github.com/russellbanks/Komac) (fixed version, checked by SHA-256).

It needs:

1. A fork of `microsoft/winget-pkgs` in the same GitHub account.
2. A classic token with the `public_repo` scope, saved as the repository secret `WINGET_TOKEN`.

Without the secret the workflow does nothing.
