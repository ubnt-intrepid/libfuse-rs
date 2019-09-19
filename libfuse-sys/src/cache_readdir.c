#include <fuse_lowlevel.h>

void
fuse_file_info_set_cache_readdir(struct fuse_file_info* fi,
                                 unsigned int cache_readdir)
{
    fi->cache_readdir = cache_readdir;
}
