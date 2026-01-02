import os
import shutil
import sys
import subprocess

def main():
    print("Building HD-50 Portable Vault...")
    
    # 1. Build Standard Release (Windows)
    # Target: x86_64-pc-windows-msvc (Default on Windows)
    # Using --jobs 1 to prevent 'os error 32' (file locking) on this environment
    cmd = ["cargo", "build", "--release", "--jobs", "1"]
    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, stderr=sys.stderr, stdout=sys.stdout)
    
    if result.returncode != 0:
        print("Build failed!")
        sys.exit(1)

    # 2. Package
    target_dir = os.path.join("target", "dist")
    if os.path.exists(target_dir):
        shutil.rmtree(target_dir)
    os.makedirs(target_dir)

    # Source files
    exe_src = os.path.join("target", "release", "hd-50-vault.exe")
    autorun_src = "autorun.inf"
    
    if not os.path.exists(exe_src):
         # Try without suffix if weird
         exe_src = os.path.join("target", "release", "hd-50-vault")
         
    if not os.path.exists(exe_src):
        print(f"Error: Executable not found at {exe_src}")
        sys.exit(1)
        
    # Copy to dist
    shutil.copy(exe_src, os.path.join(target_dir, "HD50.exe"))
    
    if os.path.exists(autorun_src):
        shutil.copy(autorun_src, os.path.join(target_dir, "autorun.inf"))

    print(f"Success! Distributable package created at: {target_dir}")
    print("Instructions:")
    print("1. Copy ALL contents of 'target/dist' to the root of your USB drive.")
    print("2. (Optional) Create a test folder 'UNLOCKED_CONTENTS' with files to start with, then run the tool to lock it.")

if __name__ == "__main__":
    main()
