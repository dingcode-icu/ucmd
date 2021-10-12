#!/usr/bin/env python3 

import sys 
import os
import platform


try:
    import pbxproj

except ImportError:
    print("Not found pbxproj lib, install it now")
    ret = os.system("pip install pbxproj")
    if (ret == 1):
        sys.exit(1)
    
from lib import xcode_mode
from lib.xcode_mode import PreparatoryWork
from lib.xcode_mode import Xcode


def get_na_path(): 
    return os.path.join(sys.argv[2], "iPhone")

def get_build_path():
    return os.path.join(sys.argv[3], "../../build/iPhone") 


def run():
    b_p = get_build_path()
    n_p = get_na_path()
    print("Build path is ->%s"%b_p)
    print("Native path is ->%s"%n_p)
    print("PreparatoryWork...")
    PreparatoryWork(b_p, n_p, [], [], ["Classes/Native", "Data"], ["", ""])
    print("Done!")

    print("Xcode handle...")
    xcode = Xcode(n_p, ["Classes/Native"], [] )
    xcode.addfolderToXcode()
    print("Done!")
    if platform.system() == "Windows":
        xcode.save("project.pbxproj")
    else:
        xcode.save()
    print("Suc!")
