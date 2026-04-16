import subprocess as sp
import time
import sys

from os import listdir
from os.path import isfile,join,isdir,dirname

bin_path = "../../target/debug/posix-shell-rust"
executable = join(dirname(__file__), bin_path) 

bash_cmd = ["bash", "--posix"]

subfolders = ["simple","hard","error"]

input_modes = ["file", "string", "stdin"]

def run_cmd(cmd):
    return sp.run(
        cmd,
        stdout=sp.PIPE,
        stderr=sp.PIPE,
        text=True
    )

def run_shell(executable_cmd, mode, file_path, script_content):
    if mode == "file":
        return run_cmd(executable_cmd + [file_path])

    if mode == "string":
        return run_cmd(executable_cmd + ["-c", script_content])

    if mode == "stdin":
        return sp.run(
            executable_cmd,
            input=script_content,
            stdout=sp.PIPE,
            stderr=sp.PIPE,
            text=True
        )

    raise ValueError(f"Unknown mode: {mode}")

def run_module(test_dir,module):
    module_path = join(test_dir,module)
    
    if not isdir(module_path):
        print(f"Module folder '{module}' not found in {test_dir}")
        return

    total = 0
    passed = 0

    for sub in subfolders:
        folder_path = join(module_path,sub)
        if not isdir(folder_path):
            continue


        files = [f for f in listdir(folder_path) if isfile(join(folder_path,f))]


        for file in files:
            file_path = join(folder_path, file)
            print(f"\n=== Running test: {file_path} ===")
        
            with open(file_path, "r") as f:
                script = f.read()
        
            for mode in input_modes:
                try:
                    res_42sh = run_shell([executable], mode, file_path, script)
                except Exception as e:
                    print("EXEC ERROR:", e)
                    raise
        
                res_bash = run_shell(bash_cmd, mode, file_path, script)
        
                total += 1
                passed += compare_results(file_path, mode, res_42sh, res_bash)


    return total, passed

RED     = "\033[31m"
GREEN   = "\033[32m"
YELLOW  = "\033[33m"
CYAN    = "\033[36m"
DIM     = "\033[2m"
RESET   = "\033[0m"
BOLD    = "\033[1m"

def compare_results(file_path, mode, r1, r2):
    ok = True
    prefix = f"{CYAN}[{mode.upper()}]{RESET}"

    if r1.returncode != r2.returncode:
        print(f"{prefix} {RED}[EXIT CODE]{RESET} "
              f"42sh={r1.returncode} bash={r2.returncode}")
        ok = False

    if r1.stdout != r2.stdout:
        print(f"{prefix} {RED}[STDOUT DIFF]{RESET}")
        print(f"{DIM}--- 42sh ---{RESET}")
        print(r1.stdout)
        print(f"{DIM}--- bash ---{RESET}")
        print(r2.stdout)
        ok = False

    bash_has_stderr = bool(r2.stderr)
    sh_has_stderr = bool(r1.stderr)

    if bash_has_stderr != sh_has_stderr:
        print(f"{prefix} {YELLOW}[STDERR PRESENCE DIFF]{RESET}")
        print(f"{DIM}--- 42sh stderr present: {sh_has_stderr} ---{RESET}")
        if r1.stderr:
            print(r1.stderr)
        print(f"{DIM}--- bash stderr present: {bash_has_stderr} ---{RESET}")
        if r2.stderr:
            print(r2.stderr)
        ok = False

    if ok:
        print(f"{prefix} {GREEN}OK{RESET}")
        return 1

    print(f"{prefix} {RED}FAIL{RESET}")
    return 0

def run_tests(test_dir,module):
    to_run = []
    if module is None or module=="None":
        to_run = [d for d in listdir(test_dir) if not isfile(join(test_dir,d))]
    else:
        to_run = [module]
    total = 0
    passed = 0
    for m in to_run:
        total_tmp,passed_tmp = run_module(test_dir,m)
        total += total_tmp
        passed += passed_tmp

    if total==0:
        return 0;

    return passed/total*100;

if __name__ == "__main__":
    test_dir = sys.argv[1]
    module_arg = None
    out_file = "None"
    if len(sys.argv) > 2:
        module_arg = sys.argv[2]
    if len(sys.argv) > 3:
        out_file = sys.argv[3]
    if len(sys.argv) > 4:
        bin_path = sys.argv[4]
        executable = bin_path
    cov = run_tests(test_dir,module_arg)
    if out_file != "None":
        with open(out_file,"w") as f:
            f.write(f"{int(cov)}")
