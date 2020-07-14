// { "preopens": { "fixture": "fixture", "/scratch": "scratch" } }

#include <assert.h>
#include <unistd.h>

int main(void) {
	// TODO (caspervonb) this definitivly needs more coverage as most
	// implementations allow you to symlink from folders outside of the
	// preopened directories.
	//
	// In-fact, removing the fixture from the preopened paths has no
	// observable effect on any known implementation.
	const char* filepath = "fixture/file";
	const char* linkpath = "/scratch/symlink_to_file";

	assert(symlink(filepath, linkpath) == 0);
	assert(unlink(linkpath) == 0);
}
