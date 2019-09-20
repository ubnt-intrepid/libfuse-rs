#include <fuse_lowlevel.h>
#include <malloc.h>
#include <sys/stat.h>

struct fuse_session*
fuse_session_new_wrapped(int argc, char const* const* argv,
                         struct fuse_lowlevel_ops const* op, void* userdata)
{
    struct fuse_args args = FUSE_ARGS_INIT(argc, (char**)argv);
    struct fuse_session* se =
        fuse_session_new(&args, op, sizeof(struct fuse_lowlevel_ops), userdata);
    fuse_opt_free_args(&args);
    return se;
}

uid_t
fuse_ctx_uid(struct fuse_ctx const* ctx)
{
    return ctx->uid;
}

gid_t
fuse_ctx_gid(struct fuse_ctx const* ctx)
{
    return ctx->gid;
}

pid_t
fuse_ctx_pid(struct fuse_ctx const* ctx)
{
    return ctx->pid;
}

mode_t
fuse_ctx_umask(struct fuse_ctx const* ctx)
{
    return ctx->umask;
}

struct fuse_entry_param*
fuse_entry_param_new(void)
{
    return (struct fuse_entry_param*)malloc(sizeof(struct fuse_entry_param));
}

void
fuse_entry_param_ino(struct fuse_entry_param* e, fuse_ino_t ino)
{
    e->ino = ino;
}

void
fuse_entry_param_generation(struct fuse_entry_param* e, uint64_t gen)
{
    e->generation = gen;
}

void
fuse_entry_param_attr(struct fuse_entry_param* e, struct stat const* attr)
{
    e->attr = *attr;
}

void
fuse_entry_param_attr_timeout(struct fuse_entry_param* e, double timeout)
{
    e->attr_timeout = timeout;
}

void
fuse_entry_param_entry_timeout(struct fuse_entry_param* e, double timeout)
{
    e->entry_timeout = timeout;
}

struct fuse_lowlevel_ops*
fuse_ll_ops_new(void)
{
    return (struct fuse_lowlevel_ops*)malloc(sizeof(struct fuse_lowlevel_ops));
}

void
fuse_ll_ops_on_init(struct fuse_lowlevel_ops* op,
                    void (*init)(void*, struct fuse_conn_info*))
{
    op->init = init;
}

void
fuse_ll_ops_on_destroy(struct fuse_lowlevel_ops* op, void (*destroy)(void*))
{
    op->destroy = destroy;
}

void
fuse_ll_ops_on_lookup(struct fuse_lowlevel_ops* op,
                      void (*lookup)(fuse_req_t, fuse_ino_t, char const*))
{
    op->lookup = lookup;
}

void
fuse_ll_ops_on_forget(struct fuse_lowlevel_ops* op,
                      void (*forget)(fuse_req_t, fuse_ino_t, uint64_t))
{
    op->forget = forget;
}

void
fuse_ll_ops_on_getattr(struct fuse_lowlevel_ops* op,
                       void (*getattr)(fuse_req_t, fuse_ino_t,
                                       struct fuse_file_info*))
{
    op->getattr = getattr;
}

void
fuse_ll_ops_on_setattr(struct fuse_lowlevel_ops* op,
                       void (*setattr)(fuse_req_t, fuse_ino_t, struct stat*,
                                       int, struct fuse_file_info*))
{
    op->setattr = setattr;
}

void
fuse_ll_ops_on_readlink(struct fuse_lowlevel_ops* op,
                        void (*readlink)(fuse_req_t, fuse_ino_t))
{
    op->readlink = readlink;
}

void
fuse_ll_ops_on_mknod(struct fuse_lowlevel_ops* op,
                     void (*mknod)(fuse_req_t, fuse_ino_t, const char*, mode_t,
                                   dev_t))
{
    op->mknod = mknod;
}

void
fuse_ll_ops_on_mkdir(struct fuse_lowlevel_ops* op,
                     void (*mkdir)(fuse_req_t, fuse_ino_t, const char*, mode_t))
{
    op->mkdir = mkdir;
}

void
fuse_ll_ops_on_unlink(struct fuse_lowlevel_ops* op,
                      void (*unlink)(fuse_req_t, fuse_ino_t, const char*))
{
    op->unlink = unlink;
}

void
fuse_ll_ops_on_rmdir(struct fuse_lowlevel_ops* op,
                     void (*rmdir)(fuse_req_t, fuse_ino_t, const char*))
{
    op->rmdir = rmdir;
}

void
fuse_ll_ops_on_symlink(struct fuse_lowlevel_ops* op,
                       void (*symlink)(fuse_req_t, const char*, fuse_ino_t,
                                       const char*))
{
    op->symlink = symlink;
}

void
fuse_ll_ops_on_rename(struct fuse_lowlevel_ops* op,
                      void (*rename)(fuse_req_t, fuse_ino_t, const char*,
                                     fuse_ino_t, const char*, unsigned int))
{
    op->rename = rename;
}

void
fuse_ll_ops_on_link(struct fuse_lowlevel_ops* op,
                    void (*link)(fuse_req_t, fuse_ino_t, fuse_ino_t,
                                 const char*))
{
    op->link = link;
}

void
fuse_ll_ops_on_open(struct fuse_lowlevel_ops* op,
                    void (*open)(fuse_req_t, fuse_ino_t,
                                 struct fuse_file_info*))
{
    op->open = open;
}

void
fuse_ll_ops_on_read(struct fuse_lowlevel_ops* op,
                    void (*read)(fuse_req_t, fuse_ino_t, size_t, off_t,
                                 struct fuse_file_info*))
{
    op->read = read;
}

void
fuse_ll_ops_on_write(struct fuse_lowlevel_ops* op,
                     void (*write)(fuse_req_t, fuse_ino_t, const char*, size_t,
                                   off_t, struct fuse_file_info*))
{
    op->write = write;
}

void
fuse_ll_ops_on_flush(struct fuse_lowlevel_ops* op,
                     void (*flush)(fuse_req_t, fuse_ino_t,
                                   struct fuse_file_info*))
{
    op->flush = flush;
}

void
fuse_ll_ops_on_release(struct fuse_lowlevel_ops* op,
                       void (*release)(fuse_req_t, fuse_ino_t,
                                       struct fuse_file_info*))
{
    op->release = release;
}

void
fuse_ll_ops_on_fsync(struct fuse_lowlevel_ops* op,
                     void (*fsync)(fuse_req_t, fuse_ino_t, int,
                                   struct fuse_file_info*))
{
    op->fsync = fsync;
}

void
fuse_ll_ops_on_opendir(struct fuse_lowlevel_ops* op,
                       void (*opendir)(fuse_req_t, fuse_ino_t,
                                       struct fuse_file_info*))
{
    op->opendir = opendir;
}

void
fuse_ll_ops_on_readdir(struct fuse_lowlevel_ops* op,
                       void (*readdir)(fuse_req_t, fuse_ino_t, size_t, off_t,
                                       struct fuse_file_info*))
{
    op->readdir = readdir;
}

void
fuse_ll_ops_on_releasedir(struct fuse_lowlevel_ops* op,
                          void (*releasedir)(fuse_req_t, fuse_ino_t,
                                             struct fuse_file_info*))
{
    op->releasedir = releasedir;
}

void
fuse_ll_ops_on_fsyncdir(struct fuse_lowlevel_ops* op,
                        void (*fsyncdir)(fuse_req_t, fuse_ino_t, int,
                                         struct fuse_file_info*))
{
    op->fsyncdir = fsyncdir;
}

void
fuse_ll_ops_on_statfs(struct fuse_lowlevel_ops* op,
                      void (*statfs)(fuse_req_t, fuse_ino_t))
{
    op->statfs = statfs;
}

void
fuse_ll_ops_on_setxattr(struct fuse_lowlevel_ops* op,
                        void (*setxattr)(fuse_req_t, fuse_ino_t, const char*,
                                         const char*, size_t, int))
{
    op->setxattr = setxattr;
}

void
fuse_ll_ops_on_getxattr(struct fuse_lowlevel_ops* op,
                        void (*getxattr)(fuse_req_t, fuse_ino_t, const char*,
                                         size_t))
{
    op->getxattr = getxattr;
}

void
fuse_ll_ops_on_listxattr(struct fuse_lowlevel_ops* op,
                         void (*listxattr)(fuse_req_t, fuse_ino_t, size_t))
{
    op->listxattr = listxattr;
}

void
fuse_ll_ops_on_removexattr(struct fuse_lowlevel_ops* op,
                           void (*removexattr)(fuse_req_t, fuse_ino_t,
                                               const char*))
{
    op->removexattr = removexattr;
}

void
fuse_ll_ops_on_access(struct fuse_lowlevel_ops* op,
                      void (*access)(fuse_req_t, fuse_ino_t, int))
{
    op->access = access;
}

void
fuse_ll_ops_on_create(struct fuse_lowlevel_ops* op,
                      void (*create)(fuse_req_t, fuse_ino_t, const char*,
                                     mode_t, struct fuse_file_info*))
{
    op->create = create;
}

unsigned int
fuse_conn_info_proto_major(struct fuse_conn_info const* conn)
{
    return conn->proto_major;
}

unsigned int
fuse_conn_info_proto_minor(struct fuse_conn_info const* conn)
{
    return conn->proto_minor;
}

unsigned int
fuse_conn_info_max_read(struct fuse_conn_info const* conn)
{
    return conn->max_read;
}

unsigned int
fuse_conn_info_capable(struct fuse_conn_info const* conn)
{
    return conn->capable;
}

unsigned int
fuse_conn_info_want(struct fuse_conn_info const* conn)
{
    return conn->want;
}

unsigned int
fuse_conn_info_max_background(struct fuse_conn_info const* conn)
{
    return conn->max_background;
}

unsigned int
fuse_conn_info_congestion_threshold(struct fuse_conn_info const* conn)
{
    return conn->congestion_threshold;
}

unsigned int
fuse_conn_info_time_gran(struct fuse_conn_info const* conn)
{
    return conn->time_gran;
}

void
fuse_conn_info_set_max_read(struct fuse_conn_info* conn, unsigned int max_read)
{
    conn->max_read = max_read;
}

void
fuse_conn_info_set_want(struct fuse_conn_info* conn, unsigned int want)
{
    conn->want = want;
}

void
fuse_conn_info_set_max_background(struct fuse_conn_info* conn,
                                  unsigned int max_background)
{
    conn->max_background = max_background;
}

void
fuse_conn_info_set_congestion_threshold(struct fuse_conn_info* conn,
                                        unsigned int threshold)
{
    conn->congestion_threshold = threshold;
}

void
fuse_conn_info_set_time_gran(struct fuse_conn_info* conn,
                             unsigned int time_gran)
{
    conn->time_gran = time_gran;
}

int
fuse_file_info_flags(struct fuse_file_info const* fi)
{
    return fi->flags;
}

uint64_t
fuse_file_info_fh(struct fuse_file_info const* fi)
{
    return fi->fh;
}

uint64_t
fuse_file_info_lock_owner(struct fuse_file_info const* fi)
{
    return fi->lock_owner;
}

unsigned int
fuse_file_info_flush(struct fuse_file_info const* fi)
{
    return fi->flush;
}

unsigned int
fuse_file_info_nonseekable(struct fuse_file_info const* fi)
{
    return fi->nonseekable;
}

unsigned int
fuse_file_info_flock_release(struct fuse_file_info const* fi)
{
    return fi->flock_release;
}

unsigned int
fuse_file_info_writepage(struct fuse_file_info const* fi)
{
    return fi->writepage;
}

void
fuse_file_info_set_fh(struct fuse_file_info* fi, uint64_t fh)
{
    fi->fh = fh;
}

void
fuse_file_info_set_direct_io(struct fuse_file_info* fi, int direct_io)
{
    fi->direct_io = direct_io;
}

void
fuse_file_info_set_keep_cache(struct fuse_file_info* fi,
                              unsigned int keep_cache)
{
    fi->keep_cache = keep_cache;
}

void
fuse_file_info_set_nonseekable(struct fuse_file_info* fi,
                               unsigned int nonseekable)
{
    fi->nonseekable = nonseekable;
}
