
### Attention
工具本身需要依赖对应的unity package配合使用

### Desc

针对unity的命令行工具

工程路径：
```
----|-proj 	     //rust工程本身
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

 用于构建unity各个平台播放器

>  ucmd build-player [FLAGS] <platform> <config>

config是环境配置文件，配置格式使用yaml,示例配置如下:

```yaml
#==========reuqire==========

unity_bin : $Unity                         #unity可执行文件 ex:/Applications/Unity/Hub/Editor/2019.4.26f1c1/Unity.app/Contents/MacOS/Unity
unity_proj : $proj root path               #unity工程路径
log_output_path : $/Users/mac/Desktop      #unity日志输路径
args: -quit -batchmode -isRelease:debug    #通用参数(-isRelease不可删除)
#==========reuqire==========



#==========android reuqire==========
android:
  na_path : $原生工程路径
  method : Ucmd.BuildPlayer.PerformBuildAndroid.ExportProjAsset      #v1.0.0 Ucmd-buildplayer
#==========android reuqire==========


#==========ios reuqire==========
ios :
  na_path : $原生工程路径
  method : ZybEditor.PerformBuild.ExportProjAsset
#==========ios reuqire==========


```

