#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>

void* run_stop_in_loop(void* data) {
    void *handle;
    void (*func_stop_lib)(void);
    const char* lib_path = (char*) data;
    handle = dlopen(lib_path, RTLD_LAZY);
    *(void**)(&func_stop_lib) = dlsym(handle, "stop_lib");
    if (!func_stop_lib) {
        /* no such symbol */
        fprintf(stderr, "Error: %s\n", dlerror());
        dlclose(handle);
        return data;
    }
    sleep(6);
//    func_stop_lib();
    return data;
}

int main(int argc, char** argv) {
    void *handle;
    void (*func_run_lib)(const char*, const char*, const char*);

    pthread_t thread_id;


    printf("argc = %d\n", argc);

    if (argc != 5) {
        fprintf(stderr, "Usage: lib_path email password\n");
        return EXIT_FAILURE;
    }
    pthread_create(&thread_id, NULL, run_stop_in_loop, (void*) argv[1]);

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
    pthread_join(thread_id, NULL);
    dlclose(handle);
    printf("Finished\n");
    return EXIT_SUCCESS;
}