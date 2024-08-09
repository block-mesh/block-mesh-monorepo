#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dlfcn.h>

int main(int argc, char** argv) {
    void *handle;
    void (*func_run_lib)(const char*, const char*, const char*);

    printf("argc = %d\n", argc);

    if (argc != 5) {
        fprintf(stderr, "Usage: lib_path email password\n");
        return EXIT_FAILURE;
    }
    handle = dlopen(argv[1], RTLD_LAZY);
    if (!handle) {
        /* fail to load the library */
        fprintf(stderr, "Error: %s\n", dlerror());
        return EXIT_FAILURE;
    }

    *(void**)(&func_run_lib) = dlsym(handle, "run_lib");
    if (!func_run_lib) {
        /* no such symbol */
        fprintf(stderr, "Error: %s\n", dlerror());
        dlclose(handle);
        return EXIT_FAILURE;
    }

    printf("url = '%s' , email = '%s' , password = '%s'\n", argv[2], argv[3], argv[4]);
    func_run_lib(argv[2], argv[3], argv[4]);
    dlclose(handle);
    printf("Finished\n");
    return EXIT_SUCCESS;
}