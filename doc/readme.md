[TOC]

------



## Desc

rust开发的针对unity辅助的命令行工具集

## Usage

```
sh:ucmd <subcommand> [options] [args] params..
```

#### 支持的子命令subcommand:

##### build-player

 用于构建unity各个平台播放器，使用package.yaml进行配置

示例配置:

```yaml
unity_bin : /Applications/Unity/Hub/Editor/2019.4.26f1c1/Unity.app/Contents/MacOS/Unity"
unity_proj : /Users/mac/data0/public_work/pinyin-unity-android/proj/pinyin
log_output_path : /Users/mac/data0/public_work/pinyin-unity-android/build

#-----------------------platform-----------------------
android:
  - na_path : /Users/mac/data0/public_work/pinyin_android
  - method : ZybEditor.PerformBuildAndroid.ExportProjAsset
ios :
  - na_path : /Users/mac/data0/public_work/pinyin_ios
  - method : ZybEditor.PerformBuild.ExportProjAsset
#-----------------------platform-----------------------
args: -quit -batchmode

```

