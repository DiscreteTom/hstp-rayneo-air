# HyperStudio's Transform Provider for RayNeo Air/1S AR Glasses

![license](https://img.shields.io/github/license/DiscreteTom/hstp-rayneo-air?style=flat-square)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/DiscreteTom/hstp-rayneo-air?style=flat-square)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/DiscreteTom/hstp-rayneo-air/build.yml?style=flat-square)](https://github.com/DiscreteTom/hstp-rayneo-air/actions/workflows/build.yml)

## Download

From [Releases](https://github.com/DiscreteTom/hstp-rayneo-air/releases).

## Usage

This works with [HyperStudio](https://github.com/DiscreteTom/HyperStudio) V2.

Replace `HyperStudio_Data/Plugins/x86_64/TransformProvider.dll` with the `TransformProvider.dll` in the release, and place `XRSDK.dll` into `HyperStudio_Data/Plugins/x86_64/`.

> After the action there should be 3 DLLs in `HyperStudio_Data/Plugins/x86_64/`: `TransformProvider.dll`, `XRSDK.dll` and `grpc_csharp_ext.dll`.

> The `XRSDK.dll` is from [RayNeo's MirrorStudio](https://bbs.leiniao.com/thread/173).

## Build

```bash
cargo build --release
```

## [CHANGELOG](CHANGELOG.md)
