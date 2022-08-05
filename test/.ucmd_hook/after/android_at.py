#!/usr/bin/env python3 

import os
import sys
import shutil


def get_na_path(): 
    return os.path.join(sys.argv[2])

def get_build_path():
    return os.path.join(sys.argv[3], "../../build") 


def run():
    build_path = get_build_path()
    na_path = get_na_path()
    if len(na_path) == 0:
        print("Not found na-proj path!Hook exist!")
    print("Build path is ->%s"%build_path)
    print("Native path is ->%s"%na_path)
    gen_list = [
        "UnityfoNa/unityLibrary/src/main/jniLibs",
        "UnityfoNa/unityLibrary/src/main/assets",
        ]
    tar_list = [
        "unityLibrary/src/main/jniLibs",
        "unityLibrary/src/main/assets"
    ]

    if len(gen_list) != len(tar_list):
        print("Copy from path count must equal tar path count!")
        sys.exit(2)

    for i in gen_list:
        f_p = os.path.join(build_path, i)
        t_p = os.path.join(na_path, i)
        if not os.path.isdir(f_p):
            print("Copy from path <%s> is not exist!"%f_p)
            sys.exit(2)

        if os.path.isdir(t_p):
            shutil.rmtree(t_p)
        print("Coyp the path...from <{f}> \n to <{t}>\n".format(f = f_p, t = t_p))
        shutil.copytree(f_p, t_p)
