use crate::{
    fuse_conn_info, //
    fuse_ctx,
    fuse_entry_param,
    fuse_file_info,
    fuse_ino_t,
    fuse_lowlevel_ops,
    fuse_req_t,
    fuse_session,
};
use libc::{
    c_char, //
    c_double,
    c_int,
    c_uint,
    c_void,
    dev_t,
    gid_t,
    mode_t,
    off_t,
    pid_t,
    size_t,
    stat,
    uid_t,
};

macro_rules! c_fn {
    ($($t:ty),*) => { Option<unsafe extern "C" fn($($t),*)> };
    ($($t:ty,)+) => { c_fn!($($t),*) }
}

extern "C" {
    pub fn fuse_session_new_wrapped(
        argc: c_int,
        argv: *const *const c_char,
        op: *const fuse_lowlevel_ops,
        userdata: *mut c_void,
    ) -> *mut fuse_session;
}

extern "C" {
    pub fn fuse_ctx_uid(ctx: *const fuse_ctx) -> uid_t;
    pub fn fuse_ctx_gid(ctx: *const fuse_ctx) -> gid_t;
    pub fn fuse_ctx_pid(ctx: *const fuse_ctx) -> pid_t;
    pub fn fuse_ctx_umask(ctx: *const fuse_ctx) -> mode_t;
}

extern "C" {
    pub fn fuse_conn_info_capable(conn: *const fuse_conn_info) -> c_uint;
    pub fn fuse_conn_info_congestion_threshold(conn: *const fuse_conn_info) -> c_uint;
    pub fn fuse_conn_info_max_background(conn: *const fuse_conn_info) -> c_uint;
    pub fn fuse_conn_info_max_read(conn: *const fuse_conn_info) -> c_uint;
    pub fn fuse_conn_info_proto_major(conn: *const fuse_conn_info) -> c_uint;
    pub fn fuse_conn_info_proto_minor(conn: *const fuse_conn_info) -> c_uint;
    pub fn fuse_conn_info_set_congestion_threshold(conn: *mut fuse_conn_info, threshold: c_uint);
    pub fn fuse_conn_info_set_max_background(conn: *mut fuse_conn_info, max_background: c_uint);
    pub fn fuse_conn_info_set_max_read(conn: *mut fuse_conn_info, max_read: c_uint);
    pub fn fuse_conn_info_set_time_gran(conn: *mut fuse_conn_info, time_gran: c_uint);
    pub fn fuse_conn_info_set_want(conn: *mut fuse_conn_info, want: c_uint);
    pub fn fuse_conn_info_time_gran(conn: *const fuse_conn_info) -> c_uint;
    pub fn fuse_conn_info_want(conn: *const fuse_conn_info) -> c_uint;
}

extern "C" {
    pub fn fuse_file_info_fh(fi: *const fuse_file_info) -> u64;
    pub fn fuse_file_info_flags(fi: *const fuse_file_info) -> c_int;
    pub fn fuse_file_info_flock_release(fi: *const fuse_file_info) -> c_uint;
    pub fn fuse_file_info_flush(fi: *const fuse_file_info) -> c_uint;
    pub fn fuse_file_info_lock_owner(fi: *const fuse_file_info) -> u64;
    pub fn fuse_file_info_set_direct_io(fi: *mut fuse_file_info, direct_io: c_int);
    pub fn fuse_file_info_set_fh(fi: *mut fuse_file_info, fh: u64);
    pub fn fuse_file_info_set_keep_cache(fi: *mut fuse_file_info, keep_cache: c_uint);
    pub fn fuse_file_info_set_nonseekable(fi: *mut fuse_file_info, nonseekable: c_uint);
    pub fn fuse_file_info_writepage(fi: *const fuse_file_info) -> c_uint;

    #[cfg(feature = "cache-readdir")]
    pub fn fuse_file_info_set_cache_readdir(fi: *mut fuse_file_info, cache_readdir: c_uint);
}

extern "C" {
    pub fn fuse_ll_ops_new() -> *mut fuse_lowlevel_ops;
    pub fn fuse_ll_ops_on_init(
        op: *mut fuse_lowlevel_ops,
        init: c_fn!(*mut c_void, *mut fuse_conn_info),
    );
    pub fn fuse_ll_ops_on_destroy(op: *mut fuse_lowlevel_ops, destroy: c_fn!(*mut c_void));
    pub fn fuse_ll_ops_on_lookup(
        op: *mut fuse_lowlevel_ops,
        lookup: c_fn!(fuse_req_t, fuse_ino_t, *const c_char),
    );
    pub fn fuse_ll_ops_on_forget(
        op: *mut fuse_lowlevel_ops,
        forget: c_fn!(fuse_req_t, fuse_ino_t, u64),
    );
    pub fn fuse_ll_ops_on_getattr(
        op: *mut fuse_lowlevel_ops,
        getattr: c_fn!(fuse_req_t, fuse_ino_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_setattr(
        op: *mut fuse_lowlevel_ops,
        setattr: c_fn!(
            fuse_req_t,
            fuse_ino_t,
            *const stat,
            c_int,
            *mut fuse_file_info,
        ),
    );
    pub fn fuse_ll_ops_on_readlink(
        op: *mut fuse_lowlevel_ops,
        readlink: c_fn!(fuse_req_t, fuse_ino_t),
    );
    pub fn fuse_ll_ops_on_mknod(
        op: *mut fuse_lowlevel_ops,
        mknod: c_fn!(fuse_req_t, fuse_ino_t, *const c_char, mode_t, dev_t),
    );
    pub fn fuse_ll_ops_on_mkdir(
        op: *mut fuse_lowlevel_ops,
        mkdir: c_fn!(fuse_req_t, fuse_ino_t, *const c_char, mode_t),
    );
    pub fn fuse_ll_ops_on_unlink(
        op: *mut fuse_lowlevel_ops,
        unlink: c_fn!(fuse_req_t, fuse_ino_t, *const c_char),
    );
    pub fn fuse_ll_ops_on_rmdir(
        op: *mut fuse_lowlevel_ops,
        rmdir: c_fn!(fuse_req_t, fuse_ino_t, *const c_char),
    );
    pub fn fuse_ll_ops_on_symlink(
        op: *mut fuse_lowlevel_ops,
        symlink: c_fn!(fuse_req_t, *const c_char, fuse_ino_t, *const c_char),
    );
    pub fn fuse_ll_ops_on_rename(
        op: *mut fuse_lowlevel_ops,
        rename: c_fn!(
            fuse_req_t,
            fuse_ino_t,
            *const c_char,
            fuse_ino_t,
            *const c_char,
            c_uint
        ),
    );
    pub fn fuse_ll_ops_on_link(
        op: *mut fuse_lowlevel_ops,
        link: c_fn!(fuse_req_t, fuse_ino_t, fuse_ino_t, *const c_char),
    );
    pub fn fuse_ll_ops_on_open(
        op: *mut fuse_lowlevel_ops,
        open: c_fn!(fuse_req_t, fuse_ino_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_read(
        op: *mut fuse_lowlevel_ops,
        read: c_fn!(fuse_req_t, fuse_ino_t, size_t, off_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_write(
        op: *mut fuse_lowlevel_ops,
        write: c_fn!(
            fuse_req_t,
            fuse_ino_t,
            *const c_char,
            size_t,
            off_t,
            *mut fuse_file_info
        ),
    );
    pub fn fuse_ll_ops_on_flush(
        op: *mut fuse_lowlevel_ops,
        flush: c_fn!(fuse_req_t, fuse_ino_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_release(
        op: *mut fuse_lowlevel_ops,
        release: c_fn!(fuse_req_t, fuse_ino_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_fsync(
        op: *mut fuse_lowlevel_ops,
        fsync: c_fn!(fuse_req_t, fuse_ino_t, c_int, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_opendir(
        op: *mut fuse_lowlevel_ops,
        opendir: c_fn!(fuse_req_t, fuse_ino_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_readdir(
        op: *mut fuse_lowlevel_ops,
        readdir: c_fn!(fuse_req_t, fuse_ino_t, size_t, off_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_releasedir(
        op: *mut fuse_lowlevel_ops,
        releasedir: c_fn!(fuse_req_t, fuse_ino_t, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_fsyncdir(
        op: *mut fuse_lowlevel_ops,
        fsyncdir: c_fn!(fuse_req_t, fuse_ino_t, c_int, *mut fuse_file_info),
    );
    pub fn fuse_ll_ops_on_statfs(op: *mut fuse_lowlevel_ops, statfs: c_fn!(fuse_req_t, fuse_ino_t));
    pub fn fuse_ll_ops_on_setxattr(
        op: *mut fuse_lowlevel_ops,
        setxattr: c_fn!(
            fuse_req_t,
            fuse_ino_t,
            *const c_char,
            *const c_char,
            size_t,
            c_int
        ),
    );
    pub fn fuse_ll_ops_on_getxattr(
        op: *mut fuse_lowlevel_ops,
        setxattr: c_fn!(fuse_req_t, fuse_ino_t, *const c_char, size_t),
    );
    pub fn fuse_ll_ops_on_listxattr(
        op: *mut fuse_lowlevel_ops,
        setxattr: c_fn!(fuse_req_t, fuse_ino_t, size_t),
    );
    pub fn fuse_ll_ops_on_removexattr(
        op: *mut fuse_lowlevel_ops,
        removexattr: c_fn!(fuse_req_t, fuse_ino_t, *const c_char),
    );
    pub fn fuse_ll_ops_on_access(
        op: *mut fuse_lowlevel_ops,
        access: c_fn!(fuse_req_t, fuse_ino_t, c_int),
    );
    pub fn fuse_ll_ops_on_create(
        op: *mut fuse_lowlevel_ops,
        create: c_fn!(
            fuse_req_t,
            fuse_ino_t,
            *const c_char,
            mode_t,
            *mut fuse_file_info
        ),
    );
}

extern "C" {
    pub fn fuse_entry_param_new() -> *mut fuse_entry_param;
    pub fn fuse_entry_param_ino(e: *mut fuse_entry_param, ino: fuse_ino_t);
    pub fn fuse_entry_param_generation(e: *mut fuse_entry_param, generation: u64);
    pub fn fuse_entry_param_attr(e: *mut fuse_entry_param, attr: *const stat);
    pub fn fuse_entry_param_attr_timeout(e: *mut fuse_entry_param, timeout: c_double);
    pub fn fuse_entry_param_entry_timeout(e: *mut fuse_entry_param, timeout: c_double);
}
