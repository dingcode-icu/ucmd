:anchor: More user-friendly command line when build unity  g

### Desc

* Extern useful command params for unity 
* Support hook include [beforebinarybuild, afterbinarybuild]
* Including but not limited to [andorid, ios, bundle]

### Usage

**make sure ucmd is executable** If not ,execute like this :

##### mac & linux 

> chmod a+x ./ucmd

##### windows (just makesure ucmd.exe exists)

```
ucmd -h 
ucmd 1.0.0
dwb <dwb@dwb.ren>
Helpful for game project build&&compile
USAGE:
    ucmd [SUBCOMMAND]
FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information
SUBCOMMANDS:
    build-player    build the player
    help            Print this message or the help of the given subcommand(s)
    init            gen config file for ucmd use
```



###### Step1.

Add **upm** in unity dependencies with **UcmdExtra**

> https://github.com/dwbmio/ucmd-extra.git



#### Step2.

Init in your unity project's root path

> ucmd init 

It will create a file <mark>.ucmd</mark> in the path.

```yaml
#======================
#BASE 
#======================
bin: /Applications/Unity/Hub/Editor/2021.3.5f1c1/Unity.app/Contents/MacOS/Unity
args: -quit -batchmode 
#======================
#UCMD EXTRA
#======================
#needed 'ucmd-extra' upm's plugin installed in project
method: Ucmd.BuildPlayer.StaticCall.Run

#extra args supported:
# -isRelease:boolean     
# -buildSymbols:string   c# symbols
# -isExportProj:boolean  

#----android
# -subTarget:string     texture target

#----ios
# -subTarget:string     texture target

ex_args: -isRelease:true

```

##### Step3.

Command to build 

> ucmd build-player . 

detail params for build-player  

you can run **-help** to check 

> ucmd build-player -h 
