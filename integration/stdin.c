// { "stdin": "Hello, stdin!" }

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
	char data[32];
	assert(fgets(data, sizeof(data), stdin) != NULL);
	assert(strcmp(data, "Hello, stdin!") == 0);

	return 0;
}
