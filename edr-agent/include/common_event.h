#pragma once
#include <stdint.h>
#define CE_SENSOR_PROC    "proc"
#define CE_SENSOR_FILE    "file"
#define CE_SENSOR_NET     "net"
#define CE_SENSOR_WX      "wx"
#define CE_SENSOR_EXPLOIT "exploit"
#define CE_SENSOR_EXTERNAL "external"
typedef struct {
  uint64_t ts_ns;
  const char *sensor;
  uint32_t pid, ppid, uid;
  const char *comm;
  const char *exe;
  // pointers to argv array are handled at print time
} ce_proc_header_t;
