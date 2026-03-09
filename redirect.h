#ifndef PRINTF_HOOK_H
#define PRINTF_HOOK_H

#ifdef __cplusplus
  #include <cstdio>
  #include <stdio.h>
#else
  #include <stdio.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif
    int custom_printf(const char *format, ...);
    int custom_fprintf(FILE *stream, const char *format, ...);
#ifdef __cplusplus
}
#endif

#define printf(...) custom_printf(__VA_ARGS__)
#define fprintf(...) custom_fprintf(__VA_ARGS__)

#endif
