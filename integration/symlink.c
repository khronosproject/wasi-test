// { "preopens": { "/scratch": "scratch" } }

#include <assert.h>
#include <string.h>
#include <unistd.h>

int main(void) {
	const char* filepath = "fixture/file";
	const char* linkpath = "/scratch/symlink_to_file";
	assert(symlink(filepath, linkpath) == 0);

	char linkdata[64] = { 0 };
	assert(readlink(linkpath, linkdata, sizeof(linkdata)) >= 0);
	assert(strstr(linkdata, "fixture/file") != NULL);
	assert(unlink(linkpath) == 0);
}
