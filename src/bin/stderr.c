// { "stderr": "Hello, stderr!" }

#include <stdio.h>

int main(void) {
	if (fputs("Hello, stderr!", stderr) == 0) {
		return ferror(stderr);
	}

	return 0;
}
