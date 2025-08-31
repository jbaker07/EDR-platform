#include <stdio.h>
#include <stdint.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>
#include <bpf/bpf.h>

int main(int argc, char **argv){
    if (argc < 2){ fprintf(stderr, "usage: %s NR [NR...]\n", argv[0]); return 1; }
    int fd = bpf_obj_get("/sys/fs/bpf/edr/maps/edr_allow_syscalls");
    if (fd < 0){ perror("bpf_obj_get"); return 2; }
    uint32_t one = 1;
    for (int i=1;i<argc;i++){
        uint32_t nr = (uint32_t)strtoul(argv[i], NULL, 0);  // accepts 56 or 0x38
        if (bpf_map_update_elem(fd, &nr, &one, BPF_ANY) < 0){ perror("update"); close(fd); return 3; }
        printf("enabled syscall nr=%u\n", nr);
    }
    close(fd);
    return 0;
}
