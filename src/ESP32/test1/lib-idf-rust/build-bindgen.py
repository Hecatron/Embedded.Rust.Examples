#! python3
import os
import sys
import subprocess
import shutil
from os import path
from glob import glob

class BuildBindgen(object):
    '''Bindgen Build Script'''

    def __init__(self):
        '''Class Init'''
        self.BINDGEN = 'bindgen'
        if 'BINDGEN' in os.environ:
            self.BINDGEN = os.environ.get('BINDGEN')
        if 'SYS_ROOT' in os.environ:
            self.SYS_ROOT = os.environ.get('SYS_ROOT')
        else:
            raise Exception("SYS_ROOT env variable not specified")
        if 'IDF_PATH' in os.environ:
            self.IDF_PATH = os.environ.get('IDF_PATH')
        else:
            raise Exception("IDF_PATH env variable not specified")
        if 'LIBCLANG_PATH' not in os.environ:
            raise Exception("LIBCLANG_PATH env variable needs to be specified for bindgen")

    def check_exe(self):
        '''Check to make sure bindgen exists on the path'''
        if not shutil.which(self.BINDGEN):
            print("")
            print("The 'bindgen' command was not found. Make sure you have bindgen")
            print("installed via 'cargo install bindgen'")
            return False
        return True

    def run_cmd(self, cmdarray, workingdir, captureout=False):
        '''Run a command'''
        stdout = stderr = None
        if captureout:
            stdout = stderr = subprocess.PIPE
        proc = subprocess.Popen(cmdarray, cwd=workingdir, stdout=stdout, stderr=stderr, universal_newlines=True)
        proc_out, proc_err = proc.communicate()
        if proc.returncode != 0:
            raise RuntimeError('Failure to run command')
        return stdout, stderr

    def get_clangflags(self):
        clangflags = ["--sysroot=" + self.SYS_ROOT]
        # -I$(pwd)/build/include - from the build using idf.py todo look at this
        clangflags += ["-D__bindgen"]
        #clangflags += ["-target", "xtensa"] - not in mainline
        clangflags += ["-x", "c"]

        # Search for all include directories
        incpath1 = path.join(self.IDF_PATH, "components", "*/include")
        incpath1 = path.abspath(incpath1)
        incpath2 = path.join(self.IDF_PATH, "components", "**/*/include")
        incpath2 = path.abspath(incpath2)
        incs1 = glob(incpath1)
        incs2 = glob(incpath2)
        for item in (incs1 + incs2):
            clangflags += ["-I" + item]
        return clangflags


    def run(self, args):
        cmdopts = [self.BINDGEN]
        cmdopts += ["--use-core", "--no-layout-tests"]
        cmdopts += ["--output", "src/bindings.rs"]
        cmdopts += ["src/bindings.h", "--"]
        clangflags = self.get_clangflags()
        cmdopts += clangflags
        #print(" ".join(cmdopts))

        # TODO need a libclang.dll
        self.run_cmd(cmdopts, '.')

    def main(self):
        if not self.check_exe():
            return
        self.run(sys.argv[1:])

if __name__ == "__main__":
    BuildBindgen().main()
