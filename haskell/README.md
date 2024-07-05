# Haskell

## Installation

Used this command to install [GHCup](https://www.haskell.org/ghcup/) on Windows:

```console
Set-ExecutionPolicy Bypass -Scope Process -Force;[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; try { Invoke-Command -ScriptBlock ([ScriptBlock]::Create((Invoke-WebRequest https://www.haskell.org/ghcup/sh/bootstrap-haskell.ps1 -UseBasicParsing))) -ArgumentList $true } catch { Write-Error $_ }
```

Then used GHCup to install `ghc` and `hsl`.

```console
> ghcup install ghc
> ghcup install hsl
```

Created symlinks to the specific `ghc` version. Replace 9.4.8 with the installed version (use `ghcup list` to figure this out).

```console
> ghcup set ghc 9.4.8
```
