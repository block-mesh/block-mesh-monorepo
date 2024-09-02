#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * # Safety
 * This method should be called by any external program that want to use BlockMesh Network CLI
 */
int8_t run_lib(const char *url, const char *email, const char *password);

/**
 * # Safety
 * This method should be called by any external program that want to use BlockMesh Network CLI
 */
int8_t stop_lib(const char *url);

/**
 * # Safety
 * This method give insight into current status of lib
 */
int8_t get_lib_status(void);
