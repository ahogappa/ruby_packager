use kompo_fs::*;
use std::{collections::HashMap, ffi::CStr};

fn initialize_thread_context(
) -> std::sync::Arc<std::sync::RwLock<std::collections::HashMap<libc::pthread_t, bool>>> {
    let mut thread_context = HashMap::new();
    thread_context.insert(unsafe { libc::pthread_self() }, false);
    std::sync::Arc::new(std::sync::RwLock::new(thread_context))
}

// pthread_create
static PTHREAD_CREATE_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        *mut libc::pthread_t,
        *const libc::pthread_attr_t,
        *const unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        *const libc::c_void,
    ) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"pthread_create\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            *mut libc::pthread_t,
            *const libc::pthread_attr_t,
            *const unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
            *const libc::c_void,
        ) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn pthread_create(
    thread: *mut libc::pthread_t,
    attr: *const libc::pthread_attr_t,
    start_routine: *const unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
    arg: *const libc::c_void,
) -> libc::c_int {
    let ret = PTHREAD_CREATE_HANDLE(thread, attr, start_routine, arg);
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    {
        let mut binding = binding.write().expect("THREAD_CONTEXT is posioned");
        let context = binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone();
        binding.insert(*thread, context);
    }

    ret
}

// open
static OPEN_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(*const libc::c_char, libc::c_int, libc::mode_t) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"open\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(*const libc::c_char, libc::c_int, libc::mode_t) -> libc::c_int,
    >(handle)
});

const ALLOW_OPEN_PATTARN1: i32 = libc::O_RDONLY | libc::O_NONBLOCK | libc::O_CLOEXEC;
const ALLOW_OPEN_PATTARN2: i32 = libc::O_RDONLY | libc::O_NONBLOCK;
const ALLOW_OPEN_PATTARN3: i32 = libc::O_RDONLY | libc::O_CLOEXEC;
const ALLOW_OPEN_PATTARN4: i32 = libc::O_RDONLY | libc::O_NOATIME;

#[no_mangle]
unsafe extern "C-unwind" fn open(
    path: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::mode_t,
) -> libc::c_int {
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    // if cstr_path.to_str().unwrap().contains("puma") {
    // println!("rust open {}, bool: {}", cstr_path.to_str().unwrap(), bool);
    // }
    if bool {
        if ALLOW_OPEN_PATTARN1 == oflag
            || ALLOW_OPEN_PATTARN2 == oflag
            || ALLOW_OPEN_PATTARN3 == oflag
            || ALLOW_OPEN_PATTARN4 == oflag
        {
            if let Some(fd) = open_from_fs(path) {
                fd
            } else {
                errno::set_errno(errno::Errno(libc::ENOENT));
                -1
            }
        } else {
            errno::set_errno(errno::Errno(libc::EROFS));
            -1
        }
    } else {
        OPEN_HANDLE(path, oflag, mode)
    }
}

// openat
static OPENAT_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
        libc::mode_t,
    ) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"openat\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
            libc::mode_t,
        ) -> libc::c_int,
    >(handle)
});

const ALLOW_OPENAT_PATTARN1: i32 = libc::O_RDONLY | libc::O_CLOEXEC | libc::O_DIRECTORY;

#[no_mangle]
unsafe extern "C-unwind" fn openat(
    dirfd: libc::c_int,
    pathname: *const libc::c_char,
    flags: libc::c_int,
    mode: libc::mode_t,
) -> libc::c_int {
    println!(
        "rust openat: path: {:?}, fd: {}, flags: {}, mode: {}",
        CStr::from_ptr(pathname),
        dirfd,
        flags,
        mode
    );
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if ALLOW_OPENAT_PATTARN1 == flags {
            if let Some(fd) = open_at_from_fs(pathname, "") {
                println!("rust openat: fd: {}", fd);
                fd
            } else {
                errno::set_errno(errno::Errno(libc::ENOENT));
                -1
            }
        } else {
            errno::set_errno(errno::Errno(libc::EROFS));
            -1
        }
        // OPENAT_HANDLE(dirfd, pathname, flags, mode)
    } else {
        OPENAT_HANDLE(dirfd, pathname, flags, mode)
    }
}

// mmap
static MMAP_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        addr: *mut libc::c_void,
        length: libc::size_t,
        prot: libc::c_int,
        flags: libc::c_int,
        fd: libc::c_int,
        offset: libc::off_t,
    ) -> *mut libc::c_void,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"mmap\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            addr: *mut libc::c_void,
            length: libc::size_t,
            prot: libc::c_int,
            flags: libc::c_int,
            fd: libc::c_int,
            offset: libc::off_t,
        ) -> *mut libc::c_void,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn mmap(
    addr: *mut libc::c_void,
    length: libc::size_t,
    prot: libc::c_int,
    flags: libc::c_int,
    fd: libc::c_int,
    offset: libc::off_t,
) -> *mut libc::c_void {
    if fd == -1 {
        return MMAP_HANDLE(addr, length, prot, flags, fd, offset);
    }

    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        let mm = MMAP_HANDLE(
            addr,
            length,
            libc::PROT_READ | libc::PROT_WRITE, // write by kompo_fs::read_from_fs()
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
            -1,
            offset,
        );

        if mm == libc::MAP_FAILED {
            return mm;
        }

        if let Some(_) = read_from_fs(fd, mm, length) {
            mm
        } else {
            errno::set_errno(errno::Errno(libc::EBADF));
            libc::MAP_FAILED
        }
    } else {
        MMAP_HANDLE(addr, length, prot, flags, fd, offset)
    }
}

// read
static READ_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        fd: libc::c_int,
        buf: *mut libc::c_void,
        count: libc::size_t,
    ) -> libc::ssize_t,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"read\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            fd: libc::c_int,
            buf: *mut libc::c_void,
            count: libc::size_t,
        ) -> libc::ssize_t,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn read(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    count: libc::size_t,
) -> libc::ssize_t {
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(result) = read_from_fs(fd, buf, count) {
            result
        } else {
            errno::set_errno(errno::Errno(libc::EBADF));
            -1
        }
    } else {
        READ_HANDLE(fd, buf, count)
    }
}

// readv
static READV_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(libc::c_int, *const libc::iovec, libc::c_int) -> libc::ssize_t,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"readv\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(libc::c_int, *const libc::iovec, libc::c_int) -> libc::ssize_t,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn readv(
    fd: libc::c_int,
    iov: *const libc::iovec,
    iovcnt: libc::c_int,
) -> libc::ssize_t {
    println!("rust readv");

    READV_HANDLE(fd, iov, iovcnt)
}

//pread
static PREAD_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        libc::c_int,
        *mut libc::c_void,
        libc::size_t,
        libc::off_t,
    ) -> libc::ssize_t,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"pread\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            libc::c_int,
            *mut libc::c_void,
            libc::size_t,
            libc::off_t,
        ) -> libc::ssize_t,
    >(handle)
});

#[no_mangle]
pub unsafe extern "C-unwind" fn pread(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    count: libc::size_t,
    offset: libc::off_t,
) -> libc::ssize_t {
    println!("rust pread");

    PREAD_HANDLE(fd, buf, count, offset)
}

//lseek
static LSEEK_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(libc::c_int, libc::off_t, libc::c_int) -> libc::off_t,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"lseek\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(libc::c_int, libc::off_t, libc::c_int) -> libc::off_t,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn lseek(
    fildes: libc::c_int,
    offset: libc::off_t,
    whence: libc::c_int,
) -> libc::off_t {
    LSEEK_HANDLE(fildes, offset, whence)
}

//stat
static STAT_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(*const libc::c_char, *mut libc::stat) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"stat\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(*const libc::c_char, *mut libc::stat) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn stat(path: *const libc::c_char, buf: *mut libc::stat) -> libc::c_int {
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(result) = stat_from_fs(path, buf) {
            result
        } else {
            errno::set_errno(errno::Errno(libc::EBADF));
            -1
        }
    } else {
        STAT_HANDLE(path, buf)
    }
}

//fstat
static FSTAT_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(fildes: libc::c_int, buf: *mut libc::stat) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"fstat\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(fildes: libc::c_int, buf: *mut libc::stat) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn fstat(fildes: libc::c_int, buf: *mut libc::stat) -> libc::c_int {
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(result) = fstat_from_fs(fildes, buf) {
            return result;
        } else {
            return -1;
        }
    } else {
        return FSTAT_HANDLE(fildes, buf);
    }
}

//lstat
static LSTAT_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(path: *const libc::c_char, buf: *mut libc::stat) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"lstat\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(path: *const libc::c_char, buf: *mut libc::stat) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn lstat(path: *const libc::c_char, buf: *mut libc::stat) -> libc::c_int {
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(result) = stat_from_fs(path, buf) {
            result
        } else {
            errno::set_errno(errno::Errno(libc::EBADF));
            -1
        }
    } else {
        LSTAT_HANDLE(path, buf)
    }
}

//close
static CLOSE_HANDLE: std::sync::LazyLock<unsafe extern "C-unwind" fn(libc::c_int) -> libc::c_int> =
    std::sync::LazyLock::new(|| unsafe {
        let handle = libc::dlsym(libc::RTLD_NEXT, b"close\0".as_ptr() as _);
        std::mem::transmute::<
            *mut libc::c_void,
            unsafe extern "C-unwind" fn(libc::c_int) -> libc::c_int,
        >(handle)
    });

#[no_mangle]
unsafe extern "C-unwind" fn close(d: libc::c_int) -> libc::c_int {
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        close_from_fs(d);
    }
    CLOSE_HANDLE(d) // kompo_fs' inner fd made by dup(). so, close it.
}

//getcwd
static GETCWD_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        buf: *mut libc::c_char,
        length: libc::size_t,
    ) -> *const libc::c_char,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"getcwd\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            buf: *mut libc::c_char,
            length: libc::size_t,
        ) -> *const libc::c_char,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn getcwd(
    buf: *mut libc::c_char,
    length: libc::size_t,
) -> *const libc::c_char {
    println!("rust getcwd");
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(path) = getcwd_from_fs(buf, length) {
            path
        } else {
            errno::set_errno(errno::Errno(libc::ERANGE));
            std::ptr::null()
        }
    } else {
        GETCWD_HANDLE(buf, length)
    }
}

//getwd
// static GETWD_HANDLE: std::sync::LazyLock<
//     unsafe extern "C-unwind" fn(path_name: *const libc::c_char) -> *const libc::c_char,
// > = std::sync::LazyLock::new(|| unsafe {
//     let handle = libc::dlsym(libc::RTLD_NEXT, b"getwd\0".as_ptr() as _);
//     std::mem::transmute::<
//         *mut libc::c_void,
//         unsafe extern "C-unwind" fn(path_name: *const libc::c_char) -> *const libc::c_char,
//     >(handle)
// });

// #[no_mangle]
// unsafe extern "C-unwind" fn getwd(path_name: *const libc::c_char) -> *const libc::c_char {
//     println!("rust getwd: {:?}", CStr::from_ptr(path_name));

//     GETWD_HANDLE(path_name)
// }

//execv
static EXECV_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        prog: *const libc::c_char,
        argv: *const *const libc::c_char,
    ) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"execv\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            prog: *const libc::c_char,
            argv: *const *const libc::c_char,
        ) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn execv(
    prog: *const libc::c_char,
    argv: *const *const libc::c_char,
) -> libc::c_int {
    println!("rust execv");

    EXECV_HANDLE(prog, argv)
}

//access
static ACCSESS_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(path: *const libc::c_char, amode: libc::c_int) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"access\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(path: *const libc::c_char, amode: libc::c_int) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn access(path: *const libc::c_char, amode: libc::c_int) -> libc::c_int {
    println!("rust access");

    ACCSESS_HANDLE(path, amode)
}

//opendir
static OPENDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(dirname: *const libc::c_char) -> *mut libc::DIR,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"opendir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(dirname: *const libc::c_char) -> *mut libc::DIR,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn opendir(dirname: *const libc::c_char) -> *mut libc::DIR {
    println!("rust opendir: {:?}", CStr::from_ptr(dirname));

    OPENDIR_HANDLE(dirname)
}

//fdopendir
static FDOPENDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(fd: libc::c_int) -> *mut libc::DIR,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"fdopendir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(fd: libc::c_int) -> *mut libc::DIR,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn fdopendir(fd: libc::c_int) -> *mut libc::DIR {
    println!("rust fdopendir: {:?}", fd);

    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(dir) = fdopendir_from_fs(fd) {
            dir
        } else {
            errno::set_errno(errno::Errno(libc::EBADF));
            std::ptr::null_mut()
        }
    } else {
        FDOPENDIR_HANDLE(fd)
    }
}

//readdir
static READDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> *mut libc::dirent,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"readdir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> *mut libc::dirent,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn readdir(dirp: *mut libc::DIR) -> *mut libc::dirent {
    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    // println!("rust readdir: {:?}, {}", dirp, bool);
    if bool {
        if let Some(dirent) = readdir_from_fs(dirp) {
            dirent
        } else {
            errno::set_errno(errno::Errno(libc::EBADF));
            std::ptr::null_mut()
        }
    } else {
        READDIR_HANDLE(dirp)
    }
}

//telledir
static TELLDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> libc::c_long,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"telldir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> libc::c_long,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn telldir(dirp: *mut libc::DIR) -> libc::c_long {
    println!("rust telldir: {:?}", dirp);

    TELLDIR_HANDLE(dirp)
}

//rewinddir
static REWINDDIR_HANDLE: std::sync::LazyLock<unsafe extern "C-unwind" fn(dirp: *mut libc::DIR)> =
    std::sync::LazyLock::new(|| unsafe {
        let handle = libc::dlsym(libc::RTLD_NEXT, b"rewinddir\0".as_ptr() as _);
        std::mem::transmute::<*mut libc::c_void, unsafe extern "C-unwind" fn(dirp: *mut libc::DIR)>(
            handle,
        )
    });

#[no_mangle]
unsafe extern "C-unwind" fn rewinddir(dirp: *mut libc::DIR) {
    println!("rust rewinddir: {:?}", dirp);

    REWINDDIR_HANDLE(dirp)
}

//seekdir
static SEEKDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(dirp: *mut libc::DIR, loc: libc::c_long),
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"seekdir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(dirp: *mut libc::DIR, loc: libc::c_long),
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn seekdir(dirp: *mut libc::DIR, loc: libc::c_long) {
    println!("rust seekdir: {:?}", dirp);

    SEEKDIR_HANDLE(dirp, loc)
}

//dirfd
static DIRFD_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"dirfd\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn dirfd(dirp: *mut libc::DIR) -> libc::c_int {
    println!("rust dirfd: {:?}", dirp);

    DIRFD_HANDLE(dirp)
}

//mkdir
static MKDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(path: *const libc::c_char, mode: libc::mode_t) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"mkdir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(path: *const libc::c_char, mode: libc::mode_t) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn mkdir(path: *const libc::c_char, mode: libc::mode_t) -> libc::c_int {
    println!("rust mkdir: {:?}", CStr::from_ptr(path));

    MKDIR_HANDLE(path, mode)
}

//closedir
static CLOSEDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"closedir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(dirp: *mut libc::DIR) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn closedir(dirp: *mut libc::DIR) -> libc::c_int {
    println!("rust closedir: {:?}", dirp);

    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(fd) = closedir_from_fs(dirp) {
            CLOSE_HANDLE(fd); // kompo_fs' inner fd made by dup(). so, close it.
            0
        } else {
            errno::set_errno(errno::Errno(libc::EBADF));
            -1
        }
    } else {
        CLOSEDIR_HANDLE(dirp)
    }
}

//chdir
static CHDIR_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(path: *const libc::c_char) -> libc::c_int,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"chdir\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(path: *const libc::c_char) -> libc::c_int,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn chdir(path: *const libc::c_char) -> libc::c_int {
    println!("rust chdir: {:?}", CStr::from_ptr(path));

    let binding = std::sync::Arc::clone(THREAD_CONTEXT.get_or_init(initialize_thread_context));
    let bool = {
        let binding = binding.read().expect("THREAD_CONTEXT is posioned");
        binding
            .get(&libc::pthread_self())
            .expect("not found thread id in THREAD_CONTEXT")
            .clone()
    };
    if bool {
        if let Some(_) = chdir_from_fs(path) {
            0
        } else {
            errno::set_errno(errno::Errno(libc::ENOENT));
            -1
        }
    } else {
        CHDIR_HANDLE(path)
    }
}

//readlink
static READLINK_HANDLE: std::sync::LazyLock<
    unsafe extern "C-unwind" fn(
        path: *const libc::c_char,
        buf: *mut libc::c_char,
        bufsz: libc::size_t,
    ) -> libc::ssize_t,
> = std::sync::LazyLock::new(|| unsafe {
    let handle = libc::dlsym(libc::RTLD_NEXT, b"readlink\0".as_ptr() as _);
    std::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C-unwind" fn(
            path: *const libc::c_char,
            buf: *mut libc::c_char,
            bufsz: libc::size_t,
        ) -> libc::ssize_t,
    >(handle)
});

#[no_mangle]
unsafe extern "C-unwind" fn readlink(
    path: *const libc::c_char,
    buf: *mut libc::c_char,
    bufsz: libc::size_t,
) -> libc::ssize_t {
    println!("rust readlink: {:?}", CStr::from_ptr(path));

    READLINK_HANDLE(path, buf, bufsz)
}

// //dlopen
// static DLOPEN_HANDLE: std::sync::LazyLock<
//     unsafe extern "C-unwind" fn(
//         filename: *const libc::c_char,
//         flag: libc::c_int,
//     ) -> *mut libc::c_void,
// > = std::sync::LazyLock::new(|| unsafe {
//     let handle = libc::dlsym(libc::RTLD_NEXT, b"dlopen\0".as_ptr() as _);
//     std::mem::transmute::<
//         *mut libc::c_void,
//         unsafe extern "C-unwind" fn(
//             filename: *const libc::c_char,
//             flag: libc::c_int,
//         ) -> *mut libc::c_void,
//     >(handle)
// });

// #[no_mangle]
// unsafe extern "C-unwind" fn dlopen(
//     filename: *const libc::c_char,
//     flag: libc::c_int,
// ) -> *mut libc::c_void {
//     println!("rust dlopen: {:?}", CStr::from_ptr(filename));

//     DLOPEN_HANDLE(filename, flag)
// }
