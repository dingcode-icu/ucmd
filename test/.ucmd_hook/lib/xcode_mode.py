#!/usr/bin/env python
# -*- coding: utf-8 -*-
# @Author: fasthro
# @Date:   2016-11-15 11:21:33
# @Last Modified by:   fasthro
# @Last Modified time: 2016-11-15 11:21:57
import shutil
import os
import re
import platform
from pbxproj import *
import shutil
# 修改工程
from pbxproj.pbxextensions import TreeType

# 打包之前准备工作
class PreparatoryWork:
    def __init__(self, frompath, topath, filefromls, filetols, folderfromls, foldertols):


        # from 根目录
        self.from_path = frompath
        # to 根目录
        self.to_path = topath

        # 需要 copy 的文件
        self.file_from_path_list = filefromls
        self.file_to_path_list = filetols

        # 需要 copy 的目录
        self.folder_from_list = folderfromls
        print("folder_from_list:===>")
        print(folderfromls)
        print("\n")
        self.folder_to_path_list = foldertols
        print("foldertols:===>")
        print(foldertols)
        print("\n")

        # copy
        self.copy(filefromls, filetols)
        self.copy(folderfromls, foldertols)

    def copy(self, fs, ts):

        for index in range(len(fs)):
            frompath = os.path.join(self.from_path, fs[index])
            topath_temp = os.path.join(self.to_path, ts[index])
            topath = os.path.join(topath_temp, fs[index])

            # 如果已经存在就删除
            if os.path.exists(topath):
                if os.path.isdir(topath):
                    shutil.rmtree(topath)
                else:
                    os.remove(topath)

            if os.path.isfile(frompath):
                print("copy %s -> %s" % (frompath, topath))
                shutil.copy(frompath, topath)
            else:
                print("copy %s -> %s" % (frompath, topath))
                shutil.copytree(frompath, topath)


# Xcode *.pbxproj 相关设置
class Xcode:
    """
    ·xpath : xcode 根目录
    ·folders : 需要添加的文件夹列表
    ·files : 需要添加的文件列表
    """

    def __init__(self, xpath=None, folders=[], files=[]):

        # xcode project path
        self.xcode_project_path = xpath

        # xcode pbxproj path
        if platform.system() == "Windows":
            self.xcode_pbxproj_path = os.path.join(xpath, 'Unity-iPhone.xcodeproj/project.pbxproj.xml')
        else:
            self.xcode_pbxproj_path = os.path.join(xpath, 'Unity-iPhone.xcodeproj/project.pbxproj')
        print("XcodeProjectPath=%s"%(self.xcode_pbxproj_path))
        # need add folders
        self.folders = folders

        # need add files
        self.files = files

        self.project = None

        if self.xcode_pbxproj_path is not None:
            pstr_xml = self.xcode_pbxproj_path[len(self.xcode_pbxproj_path) - 4: len(self.xcode_pbxproj_path)]
            pstr_proj = self.xcode_pbxproj_path[len(self.xcode_pbxproj_path) - 8: len(self.xcode_pbxproj_path)]
            if pstr_xml == '.xml':
                self.project = XcodeProject.LoadFromXML(self.xcode_pbxproj_path)
            elif pstr_proj == '.pbxproj':
                self.project = XcodeProject.load(self.xcode_pbxproj_path)
            else:
                print("xcode load error path = [%s]" % self.xcode_pbxproj_path)

        if self.project is None:
            print("Xcode load error")
        else:
            pass

        # temp file list
        self.temp_files = None
        self.temp_folder = None

    def addfileToXcode(self):
        self.addfiles(self.files)

    def addfolderToXcode(self):
        self.addfolders(self.folders)

    # 导入文件设置 -fno-objc-arc
    def set_file_seting(self, f_path, flag):
        if self.project:
            f_id = self.project.get_file_id_by_path(f_path)
            files = self.project.get_build_files(f_id)

            for f in files:
                f.add_compiler_flag(flag)

    # 添加文件夹
    def addfolders(self, folders):
        
        if self.project:
            self.temp_files = []
            for dpp in folders:
                dp = os.path.join(self.xcode_project_path, dpp)
                self.project.remove_group_by_name("Native")
                if os.path.exists(dp):
                    print ("add folder to xcode path = [%s]" % dp)
                    self.project.add_group("Native" ,dp , self.project.get_or_create_group('Classes'), TreeType.ABSOLUTE)
                    # self.project.add_folder(dp, self.project.get_or_create_group('Classes'), None, False, True, self.project.get_target_by_name("UnityFramework"))
                    # add folder file to xcode
                    self.getfilesdir(dp)

                else:
                    print("add folder path = [%s] is not exist!" % dp)

            print("add folder file : ")
            if len(self.temp_files) > 0:
                self.addfiles(self.temp_files)

    def getfilesdir(self, dp):
        for f in os.listdir(dp):
            f_p = os.path.join(dp, f)
            if os.path.isfile(f_p):
                self.temp_files.append(f_p)
            else:
                cp = re.compile(r".bundle|.framework")
                gp = cp.search(f_p)
                if gp is not None:
                    self.temp_files.append(f_p)
                else:
                    self.getfilesdir(f_p)

    def addfiles(self, files):
        if self.project:
            for fpp in files:
                fp = os.path.join(self.xcode_project_path, fpp)
                if os.path.exists(fp):
                    print("add file to xcode path = [%s]" % fp)
                    self.project.add_file(fp, self.project.get_or_create_group('Native'), "Classes/Native", "UnityFramework", False)
                else:
                    print("add file path = [%s] is not exist!" % fp)

    def addframework(self, frameworks=[], weaks=[], isbase=True):
        if self.project:
            framework_parent = self.project.get_or_create_group('Frameworks')
            for index in range(len(frameworks)):
                fw = frameworks[index]
                we = weaks[index]

                comp = re.compile('.framework$')
                match = comp.search(fw)

                tree = None
                sr = "other"

                if isbase == True:
                    tree = "SDKROOT"
                    sr = "base"

                weak = we == "True"

                if match:
                    print(
                    "add %s framework [ %s ] weak = %s" % (sr, fw, we))

                    self.project.add_file(fw, parent=framework_parent, weak=weak, tree=tree)
                else:
                    print(
                    "add %s libraries [ %s ]" % (sr, fw))

                    self.project.add_file(fw, parent=framework_parent, weak=False, tree=tree)

    def save(self, fp=None):
        if self.project:
            if fp is not None:
                self.project.save(fp)
            else:
                self.project.save()
            print( "save project")
