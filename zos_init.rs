use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;

fn main() {
    println!("🚀 ZOS Init - PID 1 Replacement");
    
    // Verify we are PID 1
    if std::process::id() != 1 {
        eprintln!("❌ Must run as PID 1");
        std::process::exit(1);
    }
    
    // Mount essential filesystems
    mount_essential();
    
    // Load SELinux policy
    load_selinux();
    
    // Setup cgroups
    setup_cgroups();
    
    // Start ZOS server
    start_zos_server();
    
    // Reap zombies (PID 1 responsibility)
    reap_zombies();
}

fn mount_essential() {
    println!("📁 Mounting essential filesystems");
    
    mount("proc", "/proc", "proc");
    mount("sysfs", "/sys", "sysfs");
    mount("devtmpfs", "/dev", "devtmpfs");
    mount("tmpfs", "/run", "tmpfs");
}

fn mount(source: &str, target: &str, fstype: &str) {
    Command::new("mount")
        .args(&["-t", fstype, source, target])
        .status()
        .ok();
}

fn load_selinux() {
    println!("🔐 Loading SELinux policy");
    
    Command::new("load_policy")
        .status()
        .ok();
}

fn setup_cgroups() {
    println!("📊 Setting up cgroups");
    
    std::fs::create_dir_all("/sys/fs/cgroup/net_cls/zos-audited").ok();
    std::fs::write("/sys/fs/cgroup/net_cls/zos-audited/net_cls.classid", "0x00100001").ok();
}

fn start_zos_server() {
    println!("🌐 Starting ZOS server");
    
    let plugins = [
        "/nix/store/.../lib/libzos_dns_server.so",
        "/nix/store/.../lib/libzos_file_proxy.so",
        "/nix/store/.../lib/libzos_github_proxy.so",
        "/nix/store/.../lib/libzos_nix_proxy.so",
        "/nix/store/.../lib/libzos_llm_proxy.so",
    ];
    
    std::thread::spawn(|| {
        Command::new("zos_server")
            .args(&plugins.iter().flat_map(|p| ["--plugin", p]).collect::<Vec<_>>())
            .spawn()
            .expect("Failed to start ZOS server");
    });
}

fn reap_zombies() {
    println!("👻 Reaping zombies (PID 1 duty)");
    
    loop {
        unsafe {
            libc::waitpid(-1, std::ptr::null_mut(), libc::WNOHANG);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
