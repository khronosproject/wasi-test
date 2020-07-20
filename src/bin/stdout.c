// { "stdout": "Hello, stdout!" }

#include <stdio.h>

int main(void) {
	if (fputs("Hello, stdout!", stdout) == 0) {
		return ferror(stdout);
	}

	return 0;
}
