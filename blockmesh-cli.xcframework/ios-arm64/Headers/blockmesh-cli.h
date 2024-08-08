#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

jint Java_xyz_blockmesh_runLib(JNIEnv env,
                               JClass _class,
                               JString url,
                               JString email,
                               JString password);

/**
 * # Safety
 * This method should be called by any external program that want to use BlockMesh Network CLI
 */
int8_t run_lib(const char *url, const char *email, const char *password);
