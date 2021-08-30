### Desc

rust开发针对unity的命令行工具

工程路径：
```
--------|-proj 		 //rust工程本身
	|-build          //构建结果
        |-doc            //文档
	|-makefile       //构建脚本
```

### Usage


## 构建
> make clean && make

## 使用
> sh:ucmd <subcommand> [options] [args] params..

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

