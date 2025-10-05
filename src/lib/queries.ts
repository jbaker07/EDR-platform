export const q = {
  opensearch: (cgrp: string) => `correlation_group_id:"${cgrp}" OR cgrp:"${cgrp}"`,
  splunk: (cgrp: string) => `index=edr_correlated_v2 (correlation_group_id="${cgrp}" OR Cgrp_s="${cgrp}")`,
  sentinel: (cgrp: string) => `EDR_CorrelatedV2_CL | where Cgrp_s == "${cgrp}"`
};
