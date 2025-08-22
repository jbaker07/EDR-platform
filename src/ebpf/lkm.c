#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/kallsyms.h>
#include <linux/syscalls.h>
#include <linux/proc_fs.h>
#include <linux/uaccess.h>
#include <linux/slab.h>
#include <linux/fs.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("EDR Agent");
MODULE_DESCRIPTION("Kernel Module for Rootkit Detection and Trust Event Logging");

static int __init edr_agent_init(void) {
    pr_info("EDR Agent LKM Loaded\n");

    // List loaded modules from /proc/modules
    struct file *f;
    loff_t pos = 0;
    char *buf = kmalloc(4096, GFP_KERNEL);

    if (!buf) {
        pr_err("[EDR LKM] Failed to allocate memory\n");
        return -ENOMEM;
    }

    f = filp_open("/proc/modules", O_RDONLY, 0);
    if (!IS_ERR(f)) {
        ssize_t bytes_read = kernel_read(f, buf, 4095, &pos);
        if (bytes_read >= 0) {
            buf[bytes_read] = '\0';
            pr_info("[EDR LKM] /proc/modules content:\n%s\n", buf);
        } else {
            pr_warn("[EDR LKM] Failed to read from /proc/modules\n");
        }
        filp_close(f, NULL);
    } else {
        pr_warn("[EDR LKM] Could not open /proc/modules\n");
    }

    kfree(buf);

    // Detect presence of sys_call_table
    unsigned long *sct = (unsigned long *)kallsyms_lookup_name("sys_call_table");
    if (sct) {
        pr_info("[EDR LKM] sys_call_table found at: %px\n", sct);
    } else {
        pr_warn("[EDR LKM] sys_call_table not found. Potential stealth hook or hardened kernel.\n");
    }

    // Detect function address of init_module and finit_module
    void *init_mod = (void *)kallsyms_lookup_name("init_module");
    void *finit_mod = (void *)kallsyms_lookup_name("finit_module");

    if (init_mod && finit_mod) {
        pr_info("[EDR LKM] init_module address: %px\n", init_mod);
        pr_info("[EDR LKM] finit_module address: %px\n", finit_mod);
    } else {
        pr_warn("[EDR LKM] init_module or finit_module not found! May indicate rootkit hiding syscall interface.\n");
    }

    // Verify whether init_module is a syscall table entry
    if (sct) {
        unsigned long syscall_init_mod = sct[__NR_init_module];
        pr_info("[EDR LKM] syscall_table[init_module] = %px\n", (void *)syscall_init_mod);

        if (init_mod && syscall_init_mod != (unsigned long)init_mod) {
            pr_warn("[EDR LKM] WARNING: init_module syscall does not match expected address! Possible syscall hook detected.\n");
        }
    }

    return 0;
}

static void __exit edr_agent_exit(void) {
    pr_info("EDR Agent LKM Unloaded\n");
}

module_init(edr_agent_init);
module_exit(edr_agent_exit);
