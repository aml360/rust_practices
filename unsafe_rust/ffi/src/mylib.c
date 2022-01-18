#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>

int my_c_function(char *byte_ptr)
{
  *byte_ptr = *byte_ptr + 254;
  return 5;
}