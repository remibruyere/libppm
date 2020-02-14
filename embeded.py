from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libppm.so")

result = lib.dummy()

print(f"Result : {result}")
