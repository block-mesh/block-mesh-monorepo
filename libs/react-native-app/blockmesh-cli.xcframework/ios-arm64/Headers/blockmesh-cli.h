#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

extern int8_t RUNNING;

/**
 * # Safety
 * This method should be called by any external program that want to use BlockMesh Network CLI
 */
int8_t stop_lib(void);

/**
 * # Safety
 * This method should be called by any external program that want to use BlockMesh Network CLI
 */
int8_t run_lib(const char *url, const char *email, const char *password);
