// { "args": ["one", "two", "three" ] }

#include <assert.h>
#include <string.h>

int main(int argc, char* argv[]) {
	assert(argc == 4);
	assert(strstr(argv[0], "args.wasm") != NULL);
	assert(strcmp(argv[1], "one") == 0);
	assert(strcmp(argv[2], "two") == 0);
	assert(strcmp(argv[3], "three") == 0);
}
